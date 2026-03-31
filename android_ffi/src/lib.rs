uniffi::setup_scaffolding!();
use core_logic::{ColorResult, generate_color, palette::Srgb};

#[derive(uniffi::Enum)]
pub enum AndroidColorResult {
    Ok { background_color: u32 },
    NetworkError,
    ParseError,
    PaletteDataLoadingError,
    PaletteDataParseError,
}

pub trait IntoAndroidColor {
    fn to_android_argb(&self) -> u32;
}

impl IntoAndroidColor for Srgb<u8> {
    fn to_android_argb(&self) -> u32 {
        let r = self.red as u32;
        let g = self.green as u32;
        let b = self.blue as u32;
        let a = 255u32;

        (a << 24) | (r << 16) | (g << 8) | b
    }
}

#[uniffi::export]
pub fn generate_color_android() -> AndroidColorResult {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(generate_color());

    match result {
        ColorResult::Ok(theme) => AndroidColorResult::Ok {
            background_color: theme.background_color.to_android_argb(),
        },
        ColorResult::NetworkError => AndroidColorResult::NetworkError,
        ColorResult::ParseError => AndroidColorResult::ParseError,
        ColorResult::PaletteDataLoadingError => AndroidColorResult::PaletteDataLoadingError,
        ColorResult::PaletteDataParseError => AndroidColorResult::PaletteDataParseError,
    }
}
