uniffi::setup_scaffolding!();
use core_logic::{
    ColorResult, generate_theme,
    palette::{FromColor, Oklch, Srgb},
};

#[derive(uniffi::Enum)]
pub enum AndroidColorResult {
    Ok {
        background_color: u32,
        foreground_color: u32,
    },
    NetworkError,
    ParseError,
    PaletteDataParseError,
}

pub trait IntoAndroidColor {
    fn to_android_argb(&self) -> u32;
}

impl IntoAndroidColor for Oklch<f32> {
    fn to_android_argb(&self) -> u32 {
        let rgb_f32: Srgb<f32> = Srgb::from_color(*self);
        let rgb_u8: Srgb<u8> = rgb_f32.into_format();

        let r = rgb_u8.red as u32;
        let g = rgb_u8.green as u32;
        let b = rgb_u8.blue as u32;
        let a = 255u32;

        (a << 24) | (r << 16) | (g << 8) | b
    }
}

#[uniffi::export]
pub fn generate_theme_android() -> AndroidColorResult {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(generate_theme());

    match result {
        ColorResult::Ok(theme) => AndroidColorResult::Ok {
            background_color: theme.original_color.bg.to_android_argb(),
            foreground_color: theme.original_color.fg.to_android_argb(),
        },
        ColorResult::NetworkError => AndroidColorResult::NetworkError,
        ColorResult::ParseError => AndroidColorResult::ParseError,
    }
}
