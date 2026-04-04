use chrono::NaiveTime;
use palette::Oklch;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct WeatherData {
    pub max_temperature: i8,
    pub min_temperature: i8,
    pub temperature: i8,
    pub sunrise_time: NaiveTime,
    pub sunset_time: NaiveTime,
}

#[derive(Clone, Serialize)]
pub struct PaletteColorVariant {
    #[serde(with = "oklch_hex")]
    pub bg: Oklch<f32>,

    #[serde(with = "oklch_hex")]
    pub fg: Oklch<f32>,
}

#[derive(Clone, Serialize)]
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

#[derive(Serialize)]
pub enum ForegroundType {
    Dark,
    Light,
}

#[derive(Serialize)]
pub struct Theme {
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
    use serde::Serializer;

    pub fn serialize<S>(color: &Oklch<f32>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let color_rgb: Srgb<f32> = color.clone().into_color();
        let result: Srgb<u8> = color_rgb.into_format();
        let hex = format!("#{:02X}{:02X}{:02X}", result.red, result.green, result.blue);

        serializer.serialize_str(&hex)
    }
}
