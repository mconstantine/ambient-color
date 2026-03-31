use core_logic::{ColorResult, data::Theme, generate_color};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
#[serde(tag = "status", content = "data")]
enum TSColorResult {
    #[serde(rename = "Ok")]
    Ok(Theme),

    #[serde(rename = "NetworkError")]
    NetworkError,

    #[serde(rename = "ParseError")]
    ParseError,

    #[serde(rename = "PaletteDataLoadingError")]
    PaletteDataLoadingError,

    #[serde(rename = "PaletteDataParseError")]
    PaletteDataParseError,
}

#[wasm_bindgen]
pub async fn generate_color_web() -> Result<JsValue, JsValue> {
    let result = generate_color().await;

    let ts_result = match result {
        ColorResult::Ok(data) => TSColorResult::Ok(data),
        ColorResult::NetworkError => TSColorResult::NetworkError,
        ColorResult::ParseError => TSColorResult::ParseError,
        ColorResult::PaletteDataLoadingError => TSColorResult::PaletteDataLoadingError,
        ColorResult::PaletteDataParseError => TSColorResult::PaletteDataParseError,
    };

    let js_value = serde_wasm_bindgen::to_value(&ts_result)?;

    Ok(js_value)
}
