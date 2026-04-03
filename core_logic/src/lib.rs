use chrono::{Datelike, Local, NaiveTime};
use palette::{Hsv, IntoColor, ShiftHue, Srgb};

use crate::{
    color::{DailyTemperature, SolarTimes, get_hue, get_lightness, get_saturation},
    data::{Theme, ThemeExtended},
    distance::{get_closest_palette_color, get_foreground_color, get_foreground_type},
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
pub mod theme;

pub struct ColorData {
    pub max_temperature: i8,
    pub min_temperature: i8,
    pub temperature: i8,
    pub sunrise_time: NaiveTime,
    pub sunset_time: NaiveTime,
}

pub enum ColorResult<T> {
    Ok(T),
    NetworkError,
    ParseError,
    PaletteDataParseError,
}

impl<T> From<reqwest::Error> for ColorResult<T> {
    fn from(_: reqwest::Error) -> Self {
        ColorResult::NetworkError
    }
}

impl<T> From<WttrParseError> for ColorResult<T> {
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

impl<T> From<OklchExtractionError> for ColorResult<T> {
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

pub async fn generate_theme() -> ColorResult<Theme> {
    let result: Result<Theme, ColorResult<Theme>> = async {
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
) -> Result<Theme, ColorResult<Theme>> {
    let generated_color = generate_color(data, day_of_year, time_of_day)?;
    let palette = load_palette(PALETTE_JSON)?;

    let (closest_color_primary, closest_variant_oklch_primary) =
        get_closest_palette_color(&generated_color, &palette);

    let background_color_rgb: Srgb<f32> = closest_variant_oklch_primary.color.into_color();
    let foreground_color_rgb = get_foreground_color(background_color_rgb, closest_color_primary);

    let background_color: Srgb<u8> = background_color_rgb.into_format();
    let foreground_color: Srgb<u8> = foreground_color_rgb.into_format();

    Ok(Theme {
        background_color,
        foreground_color,
    })
}

pub async fn generate_theme_extended() -> ColorResult<ThemeExtended> {
    let result: Result<ThemeExtended, ColorResult<ThemeExtended>> = async {
        let response = fetch_wttr_data().await?;
        let data = parse_wttr_data(&response)?;
        let now = Local::now();

        compute_theme_extended(&data, now.ordinal(), now.time())
    }
    .await;

    match result {
        Ok(theme) => ColorResult::Ok(theme),
        Err(error) => error,
    }
}

pub fn compute_theme_extended(
    data: &ColorData,
    day_of_year: u32,
    time_of_day: NaiveTime,
) -> Result<ThemeExtended, ColorResult<ThemeExtended>> {
    let generated_color = generate_color(data, day_of_year, time_of_day)?;
    let opposite_color = generated_color.shift_hue(180.0);
    let secondary_color = generated_color.shift_hue(120.0);
    let tertiary_color = generated_color.shift_hue(240.0);

    let palette = load_palette(PALETTE_JSON)?;

    let (primary_palette, primary_variant) = get_closest_palette_color(&generated_color, &palette);
    let (opposite_palette, opposite_variant) = get_closest_palette_color(&opposite_color, &palette);

    let (secondary_palette, secondary_variant) =
        get_closest_palette_color(&secondary_color, &palette);

    let (tertiary_palette, tertiary_variant) = get_closest_palette_color(&tertiary_color, &palette);

    let neutral_palette = palette
        .iter()
        .find(|c| c.name == "neutral")
        .cloned()
        .unwrap();

    let primary_500 = primary_palette
        .variants()
        .iter()
        .find(|v| v.name == format!("{}_500", primary_palette.name))
        .cloned()
        .unwrap();

    let opposite_500 = opposite_palette
        .variants()
        .iter()
        .find(|v| v.name == format!("{}_500", opposite_palette.name))
        .cloned()
        .unwrap();

    let secondary_500 = secondary_palette
        .variants()
        .iter()
        .find(|v| v.name == format!("{}_500", secondary_palette.name))
        .cloned()
        .unwrap();

    let tertiary_500 = tertiary_palette
        .variants()
        .iter()
        .find(|v| v.name == format!("{}_500", tertiary_palette.name))
        .cloned()
        .unwrap();

    let original_color_rgb: Srgb<f32> = generated_color.into_color();
    let primary_color_rgb: Srgb<f32> = primary_variant.color.into_color();
    let primary_500_rgb: Srgb<f32> = primary_500.color.into_color();
    let opposite_color_rgb: Srgb<f32> = opposite_variant.color.into_color();
    let opposite_500_rgb: Srgb<f32> = opposite_500.color.into_color();
    let secondary_color_rgb: Srgb<f32> = secondary_variant.color.into_color();
    let secondary_500_rgb: Srgb<f32> = secondary_500.color.into_color();
    let tertiary_color_rgb: Srgb<f32> = tertiary_variant.color.into_color();
    let tertiary_500_rgb: Srgb<f32> = tertiary_500.color.into_color();

    let original_color: Srgb<u8> = original_color_rgb.into_format();
    let original_foreground_type = get_foreground_type(opposite_color_rgb);

    let primary_foreground_type = get_foreground_type(primary_color_rgb);
    let primary_500_foreground_type = get_foreground_type(primary_500_rgb);
    let opposite_foreground_type = get_foreground_type(opposite_color_rgb);
    let opposite_500_foreground_type = get_foreground_type(opposite_500_rgb);
    let secondary_foreground_type = get_foreground_type(secondary_color_rgb);
    let secondary_500_foreground_type = get_foreground_type(secondary_500_rgb);
    let tertiary_foreground_type = get_foreground_type(tertiary_color_rgb);
    let tertiary_500_foreground_type = get_foreground_type(tertiary_500_rgb);

    Ok(ThemeExtended {
        original_color,
        original_foreground_type,
        primary_variant,
        primary_palette,
        primary_foreground_type,
        primary_500_foreground_type,
        opposite_variant,
        opposite_palette,
        opposite_foreground_type,
        opposite_500_foreground_type,
        secondary_variant,
        secondary_palette,
        secondary_foreground_type,
        secondary_500_foreground_type,
        tertiary_variant,
        tertiary_palette,
        tertiary_foreground_type,
        tertiary_500_foreground_type,
        neutral_palette,
    })
}

fn generate_color<T>(
    data: &ColorData,
    day_of_year: u32,
    time_of_day: NaiveTime,
) -> Result<Hsv, ColorResult<T>> {
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

    Ok(Hsv::new(hue, saturation, lightness))
}
