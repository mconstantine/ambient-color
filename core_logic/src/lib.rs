use chrono::{Datelike, Local};
use palette::{Hsv, IntoColor, Srgb};

use crate::{
    color::{DailyTemperature, SolarTimes, get_hue, get_lightness, get_saturation},
    data::Theme,
    network::fetch_wttr_data,
    parse::{WttrParseError, parse_wttr_data},
};

pub use palette;

mod color;
pub mod data;
mod network;
mod parse;
mod theme;

pub enum ColorResult {
    Ok(Theme),
    NetworkError,
    ParseError,
}

impl From<reqwest::Error> for ColorResult {
    fn from(_: reqwest::Error) -> Self {
        ColorResult::NetworkError
    }
}

impl From<WttrParseError> for ColorResult {
    fn from(error: WttrParseError) -> Self {
        match error {
            WttrParseError::MissingDailyWeather => {
                println!("Missing daily weather from wttr response")
            }
            WttrParseError::MissingAstronomyData => {
                println!("Missing astronomy data from wttr response")
            }
            WttrParseError::MissingCurrentCondition => {
                println!("Missing current weather condition from wttr response")
            }
            WttrParseError::ParseTime { received } => {
                println!("Unable to parse time, found {}", received)
            }
            WttrParseError::ParseTemperature { received } => {
                println!("Unable to parse temperature, found {}", received)
            }
        }

        ColorResult::ParseError
    }
}

pub async fn generate_color() -> ColorResult {
    let result: Result<Theme, ColorResult> = async {
        let response = fetch_wttr_data().await?;
        let data = parse_wttr_data(&response)?;
        let now = Local::now();
        let hue = get_hue(now.ordinal());

        let saturation = get_saturation(
            DailyTemperature {
                max: data.max_temperature,
                min: data.min_temperature,
            },
            data.temperature,
        );

        let lightness = get_lightness(
            SolarTimes {
                sunrise: data.sunrise_time,
                sunset: data.sunset_time,
            },
            now.time(),
        );

        let hsv = Hsv::new(hue, saturation, lightness);
        let rgb: Srgb<f32> = hsv.into_color();
        let primary_color: Srgb<u8> = rgb.into_format();

        Ok(Theme { primary_color })
    }
    .await;

    match result {
        Ok(theme) => ColorResult::Ok(theme),
        Err(error) => error,
    }
}
