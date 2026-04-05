use chrono::{Datelike, Local, NaiveTime};
use palette::{OklabHue, Oklch};

use crate::{
    color::{DailyTemperature, SolarTimes, get_chroma, get_hue, get_luma},
    data::{ColorData, PaletteColorVariant, Theme, WeatherData},
    network::fetch_wttr_data,
    parse::{WttrParseError, parse_wttr_data},
    theme::{generate_palette, generate_palette_with_base_chroma, get_foreground_color},
};

pub use chrono;
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

pub async fn generate_theme() -> ColorResult {
    let data: Result<WeatherData, ColorResult> = async {
        let response = fetch_wttr_data().await?;

        Ok(parse_wttr_data(&response)?)
    }
    .await;

    match data {
        Ok(data) => {
            let now = Local::now();
            let theme = compute_theme(&data, now.ordinal(), now.time());

            ColorResult::Ok(theme)
        }
        Err(error) => error,
    }
}

pub fn compute_theme(data: &WeatherData, day_of_year: u32, time_of_day: NaiveTime) -> Theme {
    let hue = get_hue(day_of_year);

    let chroma = get_chroma(
        DailyTemperature {
            max: data.max_temperature,
            min: data.min_temperature,
        },
        data.temperature,
    );

    let luma = get_luma(
        SolarTimes {
            sunrise: data.sunrise_time,
            sunset: data.sunset_time,
        },
        time_of_day,
    );

    let original_color: Oklch = Oklch {
        l: luma,
        chroma,
        hue: OklabHue::new(hue),
    };

    let primary_palette = generate_palette(hue);
    let opposite_palette = generate_palette((hue + 180.0).rem_euclid(360.0));
    let secondary_palette = generate_palette((hue + 120.0).rem_euclid(360.0));
    let tertiary_palette = generate_palette((hue + 240.0).rem_euclid(360.0));

    let neutral_palette =
        generate_palette_with_base_chroma(original_color.hue.into_positive_degrees(), 0.01);

    let original_color_foreground = get_foreground_color(
        original_color,
        primary_palette.w50.bg,
        primary_palette.w950.bg,
    );

    Theme {
        day_of_year,
        weather_data: (*data).clone(),
        color_data: ColorData { hue, chroma, luma },
        original_color: PaletteColorVariant {
            bg: original_color,
            fg: original_color_foreground,
        },
        primary_palette,
        opposite_palette,
        secondary_palette,
        tertiary_palette,
        neutral_palette,
    }
}
