use chrono::{Datelike, Local, NaiveTime};
use palette::{Hsl, IntoColor, Oklch, ShiftHue};

use crate::{
    color::{DailyTemperature, SolarTimes, get_hue, get_lightness, get_saturation},
    data::{PaletteColorVariant, Theme},
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

pub async fn generate_theme() -> ColorResult {
    let data: Result<ColorData, ColorResult> = async {
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

pub fn compute_theme(data: &ColorData, day_of_year: u32, time_of_day: NaiveTime) -> Theme {
    let original_color_hsv = generate_color(data, day_of_year, time_of_day);
    let original_color: Oklch = original_color_hsv.into_color();
    let opposite_color = original_color_hsv.shift_hue(180.0);
    let secondary_color = original_color_hsv.shift_hue(120.0);
    let tertiary_color = original_color_hsv.shift_hue(240.0);

    let primary_palette = generate_palette(original_color.hue.into_positive_degrees());
    let opposite_palette = generate_palette(opposite_color.hue.into_positive_degrees());
    let secondary_palette = generate_palette(secondary_color.hue.into_positive_degrees());
    let tertiary_palette = generate_palette(tertiary_color.hue.into_positive_degrees());

    let neutral_palette =
        generate_palette_with_base_chroma(original_color.hue.into_positive_degrees(), 0.01);

    let original_color_foreground = get_foreground_color(
        original_color,
        primary_palette.w50.bg,
        primary_palette.w950.bg,
    );

    Theme {
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

fn generate_color(data: &ColorData, day_of_year: u32, time_of_day: NaiveTime) -> Hsl {
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

    Hsl::new(hue, saturation, lightness)
}
