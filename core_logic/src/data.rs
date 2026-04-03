use palette::Srgb;
use serde::{Deserialize, Serialize};

use crate::theme::PaletteColor;

#[derive(Serialize)]
pub enum ForegroundType {
    Dark,
    Light,
}

#[derive(Serialize, Deserialize)]
pub struct Theme {
    #[serde(with = "srgb_hex")]
    pub background_color: Srgb<u8>,

    #[serde(with = "srgb_hex")]
    pub foreground_color: Srgb<u8>,
}

#[derive(Serialize)]
pub struct ThemeExtended {
    #[serde(with = "srgb_hex")]
    pub original_color: Srgb<u8>,
    pub original_foreground_type: ForegroundType,
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
