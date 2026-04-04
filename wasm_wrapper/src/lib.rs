use core_logic::{
    ColorResult,
    chrono::NaiveTime,
    compute_theme,
    data::{Theme, WeatherData},
    generate_theme,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Deserialize)]
pub struct TSColorInput {
    pub max_temperature: i8,
    pub min_temperature: i8,
    pub temperature: i8,
    pub sunrise_time: String,
    pub sunset_time: String,
    pub day_of_year: u32,
    pub now: String,
}

#[derive(Serialize)]
#[serde(tag = "status", content = "data")]
enum TSColorResult {
    #[serde(rename = "Ok")]
    Ok(Theme),

    #[serde(rename = "NetworkError")]
    NetworkError,

    #[serde(rename = "ParseError")]
    ParseError,

    #[serde(rename = "InvalidInput")]
    InvalidInput,
}

#[wasm_bindgen]
pub async fn generate_theme_web() -> Result<JsValue, JsValue> {
    let result = generate_theme().await;
    let ts_result = parse_color_result(result);
    let js_value = serde_wasm_bindgen::to_value(&ts_result)?;

    Ok(js_value)
}

#[wasm_bindgen]
pub fn compute_theme_web(input: JsValue) -> Result<JsValue, JsValue> {
    let result: TSColorResult = match parse_and_compute(input) {
        Ok(result) => result,
        Err(result) => result,
    };

    let js_value = serde_wasm_bindgen::to_value(&result)?;

    Ok(js_value)
}

fn parse_and_compute(input: JsValue) -> Result<TSColorResult, TSColorResult> {
    let data: TSColorInput =
        serde_wasm_bindgen::from_value(input).map_err(|_| TSColorResult::InvalidInput)?;

    let sunrise_time = NaiveTime::parse_from_str(&data.sunrise_time, "%H:%M:%S")
        .map_err(|_| TSColorResult::InvalidInput)?;

    let sunset_time = NaiveTime::parse_from_str(&data.sunset_time, "%H:%M:%S")
        .map_err(|_| TSColorResult::InvalidInput)?;

    let now = NaiveTime::parse_from_str(&data.now, "%H:%M:%S")
        .map_err(|_| TSColorResult::InvalidInput)?;

    let color_data = WeatherData {
        max_temperature: data.max_temperature,
        min_temperature: data.min_temperature,
        temperature: data.temperature,
        sunrise_time,
        sunset_time,
    };

    let theme = compute_theme(&color_data, data.day_of_year, now);

    Ok(TSColorResult::Ok(theme))
}

fn parse_color_result(result: ColorResult) -> TSColorResult {
    match result {
        ColorResult::Ok(data) => TSColorResult::Ok(data),
        ColorResult::NetworkError => TSColorResult::NetworkError,
        ColorResult::ParseError => TSColorResult::ParseError,
    }
}
