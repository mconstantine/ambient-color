use chrono::NaiveTime;
use palette::Oklch;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ColorData {
    pub hue: f32,
    pub chroma: f32,
    pub luma: f32,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(into = "String")]
pub enum WeatherCondition {
    Sunny,
    Cloudy,
    Fog,
    Rain,
    Snow,
    Unknown(String),
}

impl Into<String> for WeatherCondition {
    fn into(self) -> String {
        match self {
            WeatherCondition::Sunny => String::from("Sunny"),
            WeatherCondition::Cloudy => String::from("Cloudy"),
            WeatherCondition::Fog => String::from("Fog"),
            WeatherCondition::Rain => String::from("Rain"),
            WeatherCondition::Snow => String::from("Snow"),
            WeatherCondition::Unknown(code) => code,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(into = "String")]
pub enum MoonPhase {
    NewMoon,
    WaxingCrescent,
    FirstQuarter,
    WaxingGibbous,
    FullMoon,
    WaningGibbous,
    LastQuarter,
    WaningCrescent,
    Unknown(String),
}

impl Into<String> for MoonPhase {
    fn into(self) -> String {
        match self {
            MoonPhase::NewMoon => String::from("NewMoon"),
            MoonPhase::WaxingCrescent => String::from("WaxingCrescent"),
            MoonPhase::FirstQuarter => String::from("FirstQuarter"),
            MoonPhase::WaxingGibbous => String::from("WaxingGibbous"),
            MoonPhase::FullMoon => String::from("FullMoon"),
            MoonPhase::WaningGibbous => String::from("WaningGibbous"),
            MoonPhase::LastQuarter => String::from("LastQuarter"),
            MoonPhase::WaningCrescent => String::from("WaningCrescent"),
            MoonPhase::Unknown(moon_phase) => moon_phase,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WeatherData {
    pub max_temperature: i8,
    pub min_temperature: i8,
    pub temperature: i8,
    pub sunrise_time: NaiveTime,
    pub sunset_time: NaiveTime,
    pub weather_condition: WeatherCondition,
    pub moon_phase: MoonPhase,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PaletteColorVariant {
    #[serde(with = "oklch_hex")]
    pub bg: Oklch<f32>,

    #[serde(with = "oklch_hex")]
    pub fg: Oklch<f32>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PaletteColor {
    pub w50: PaletteColorVariant,
    pub w100: PaletteColorVariant,
    pub w200: PaletteColorVariant,
    pub w300: PaletteColorVariant,
    pub w400: PaletteColorVariant,
    pub w500: PaletteColorVariant,
    pub w600: PaletteColorVariant,
    pub w700: PaletteColorVariant,
    pub w800: PaletteColorVariant,
    pub w900: PaletteColorVariant,
    pub w950: PaletteColorVariant,
}
impl PaletteColor {
    pub fn variants(&self) -> [&PaletteColorVariant; 11] {
        [
            &self.w50, &self.w100, &self.w200, &self.w300, &self.w400, &self.w500, &self.w600,
            &self.w700, &self.w800, &self.w900, &self.w950,
        ]
    }
}

#[derive(Serialize, Deserialize)]
pub enum Time {
    Sunrise,
    Day,
    Sunset,
    Night,
}

#[derive(Serialize, Deserialize)]
pub struct Theme {
    pub time: Time,
    pub day_of_year: u32,
    pub color_data: ColorData,
    pub weather_data: WeatherData,
    pub original_color: PaletteColorVariant,
    pub primary_palette: PaletteColor,
    pub opposite_palette: PaletteColor,
    pub secondary_palette: PaletteColor,
    pub tertiary_palette: PaletteColor,
    pub neutral_palette: PaletteColor,
}

pub mod srgb_hex {
    use palette::Srgb;
    use serde::{self, Deserialize, Deserializer, Serializer};
    use std::str::FromStr;

    pub fn serialize<S>(color: &Srgb<u8>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let hex = format!("#{:02X}{:02X}{:02X}", color.red, color.green, color.blue);
        serializer.serialize_str(&hex)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Srgb<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        Srgb::from_str(&s).map_err(serde::de::Error::custom)
    }
}

pub mod oklch_hex {
    use palette::{IntoColor, Oklch, Srgb};
    use serde::{Deserialize, Deserializer, Serializer, de::Error};

    pub fn serialize<S>(color: &Oklch<f32>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let color_rgb: Srgb<f32> = color.clone().into_color();
        let result: Srgb<u8> = color_rgb.into_format();
        let hex = format!("#{:02X}{:02X}{:02X}", result.red, result.green, result.blue);

        serializer.serialize_str(&hex)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Oklch<f32>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let hex = s.trim_start_matches('#');

        if hex.len() != 6 {
            return Err(D::Error::custom(format!(
                "Expected a 6-characters hex color, found {}",
                s
            )));
        }

        let r = u8::from_str_radix(&hex[0..2], 16).map_err(D::Error::custom)?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(D::Error::custom)?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(D::Error::custom)?;

        let srgb_u8 = Srgb::new(r, g, b);
        let srgb_f32: Srgb<f32> = srgb_u8.into_format();

        Ok(srgb_f32.into_color())
    }
}
