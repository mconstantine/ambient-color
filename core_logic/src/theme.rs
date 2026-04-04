use palette::{IntoColor, Oklch, Srgb, color_difference::Wcag21RelativeContrast};

use crate::data::{PaletteColor, PaletteColorVariant};

pub fn generate_palette(hue: f32) -> PaletteColor {
    generate_palette_with_base_chroma(hue, 0.20)
}

pub fn generate_palette_with_base_chroma(hue: f32, base_chroma: f32) -> PaletteColor {
    let w50 = Oklch::new(0.98, base_chroma * 0.10, hue);
    let w100 = Oklch::new(0.92, base_chroma * 0.18, hue);
    let w200 = Oklch::new(0.88, base_chroma * 0.25, hue);
    let w300 = Oklch::new(0.80, base_chroma * 0.50, hue);
    let w400 = Oklch::new(0.70, base_chroma * 0.85, hue);
    let w500 = Oklch::new(0.60, base_chroma * 1.00, hue);
    let w600 = Oklch::new(0.50, base_chroma * 0.95, hue);
    let w700 = Oklch::new(0.40, base_chroma * 0.80, hue);
    let w800 = Oklch::new(0.30, base_chroma * 0.55, hue);
    let w900 = Oklch::new(0.20, base_chroma * 0.45, hue);
    let w950 = Oklch::new(0.15, base_chroma * 0.30, hue);

    PaletteColor {
        w50: PaletteColorVariant {
            bg: w50,
            fg: get_foreground_color(w50, w50, w950),
        },
        w100: PaletteColorVariant {
            bg: w100,
            fg: get_foreground_color(w100, w50, w950),
        },
        w200: PaletteColorVariant {
            bg: w200,
            fg: get_foreground_color(w200, w50, w950),
        },
        w300: PaletteColorVariant {
            bg: w300,
            fg: get_foreground_color(w300, w50, w950),
        },
        w400: PaletteColorVariant {
            bg: w400,
            fg: get_foreground_color(w400, w50, w950),
        },
        w500: PaletteColorVariant {
            bg: w500,
            fg: get_foreground_color(w500, w50, w950),
        },
        w600: PaletteColorVariant {
            bg: w600,
            fg: get_foreground_color(w600, w50, w950),
        },
        w700: PaletteColorVariant {
            bg: w700,
            fg: get_foreground_color(w700, w50, w950),
        },
        w800: PaletteColorVariant {
            bg: w800,
            fg: get_foreground_color(w800, w50, w950),
        },
        w900: PaletteColorVariant {
            bg: w900,
            fg: get_foreground_color(w900, w50, w950),
        },
        w950: PaletteColorVariant {
            bg: w950,
            fg: get_foreground_color(w950, w50, w950),
        },
    }
}

pub fn get_foreground_color(background_color: Oklch, w50: Oklch, w950: Oklch) -> Oklch {
    let rgb: Srgb<f32> = background_color.into_color();

    if rgb.relative_luminance().luma > 0.179 {
        w950
    } else {
        w50
    }
}
