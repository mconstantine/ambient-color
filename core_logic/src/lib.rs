use chrono::{Datelike, Local, NaiveTime};
use palette::{Hsv, IntoColor, Srgb};

use crate::{
    color::{DailyTemperature, SolarTimes, get_hue, get_lightness, get_saturation},
    data::Theme,
    distance::{get_closest_palette_color, get_foreground_color},
    network::fetch_wttr_data,
    parse::{WttrParseError, parse_wttr_data},
    theme::{OklchExtractionError, PALETTE_JSON, load_palette},
};

pub use chrono;
pub use palette;

mod color;
pub mod data;
mod distance;
mod network;
mod parse;
mod theme;

pub struct ColorData {
    pub max_temperature: i8,
    pub min_temperature: i8,
    pub temperature: i8,
    pub sunrise_time: NaiveTime,
    pub sunset_time: NaiveTime,
}

pub enum ColorResult {
    Ok(Theme),
    NetworkError,
    ParseError,
    PaletteDataParseError,
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

impl From<OklchExtractionError> for ColorResult {
    fn from(value: OklchExtractionError) -> Self {
        match value {
            OklchExtractionError::MissingVariant { hue, weight } => {
                println!("Missing variatnt {}_{} in palette data", hue, weight)
            }
            OklchExtractionError::Prefix { data } => {
                println!(
                    "Invalid palette data in {}_{}: expected prefix \"oklch(\", found {}",
                    data.hue, data.weight, data.received
                )
            }
            OklchExtractionError::Suffix { data } => {
                println!(
                    "Invalid palette data in {}_{}: expected suffix \")\", found {}",
                    data.hue, data.weight, data.received
                )
            }
            OklchExtractionError::Format { data } => {
                println!(
                    "Invalid format for variant {}_{} in palette data: found {}",
                    data.hue, data.weight, data.received
                )
            }
            OklchExtractionError::LFormat { data } => {
                println!(
                    "Invalid format for L value in {}_{}: found {}",
                    data.hue, data.weight, data.received
                )
            }
            OklchExtractionError::CFormat { data } => {
                println!(
                    "Invalid format for C value in {}_{}: found {}",
                    data.hue, data.weight, data.received
                )
            }
            OklchExtractionError::HFormat { data } => {
                println!(
                    "Invalid format for H value in {}_{}: found {}",
                    data.hue, data.weight, data.received
                )
            }
        };

        ColorResult::PaletteDataParseError
    }
}

pub async fn generate_color() -> ColorResult {
    let result: Result<Theme, ColorResult> = async {
        let response = fetch_wttr_data().await?;
        let data = parse_wttr_data(&response)?;
        let now = Local::now();

        compute_theme(&data, now.ordinal(), now.time())
    }
    .await;

    match result {
        Ok(theme) => ColorResult::Ok(theme),
        Err(error) => error,
    }
}

pub fn compute_theme(
    data: &ColorData,
    day_of_year: u32,
    time_of_day: NaiveTime,
) -> Result<Theme, ColorResult> {
    let hue = get_hue(day_of_year);

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
        time_of_day,
    );

    let generated_color = Hsv::new(hue, saturation, lightness);

    let palette = load_palette(PALETTE_JSON)?;

    let (closest_color, closest_variant_oklch) =
        get_closest_palette_color(&generated_color, &palette);

    let background_color_rgb: Srgb<f32> = closest_variant_oklch.color.into_color();

    let foreground_color_rgb = get_foreground_color(background_color_rgb, closest_color);
    let background_color: Srgb<u8> = background_color_rgb.into_format();
    let foreground_color: Srgb<u8> = foreground_color_rgb.into_format();

    Ok(Theme {
        background_color,
        foreground_color,
    })
}
