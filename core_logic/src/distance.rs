use palette::{
    Hsv, IntoColor, Oklab, Srgb,
    color_difference::{EuclideanDistance, Wcag21RelativeContrast},
};

use crate::{
    data::ForegroundType,
    theme::{PaletteColor, PaletteColorVariant},
};

pub fn get_closest_palette_color(
    reference: &Hsv,
    palette: &[PaletteColor],
) -> (PaletteColor, PaletteColorVariant) {
    let ref_oklab: Oklab = (*reference).into_color();

    let variant = palette
        .iter()
        .flat_map(|color| color.variants())
        .min_by(|a, b| {
            let a_oklab: Oklab = a.bg.into_color();
            let b_oklab: Oklab = b.bg.into_color();
            let distance_a = a_oklab.distance(ref_oklab);
            let distance_b = b_oklab.distance(ref_oklab);

            distance_a.partial_cmp(&distance_b).unwrap()
        })
        .cloned()
        .expect("palette should not be empty");

    let color_name = *variant.name.split('_').collect::<Vec<_>>().get(0).unwrap();

    let color = palette
        .iter()
        .find(|color| color.name == color_name)
        .cloned()
        .unwrap();

    (color, variant)
}

pub fn get_foreground_color(background_color: Srgb<f32>, palette_color: PaletteColor) -> Srgb {
    let foreground_type = get_foreground_type(background_color);

    match foreground_type {
        ForegroundType::Dark => {
            let dark: Srgb<f32> = palette_color.w950.bg.into_color();
            dark
        }
        ForegroundType::Light => {
            let light: Srgb<f32> = palette_color.w50.bg.into_color();
            light
        }
    }
}

pub fn get_foreground_type(background_color: Srgb<f32>) -> ForegroundType {
    if background_color.relative_luminance().luma > 0.179 {
        ForegroundType::Dark
    } else {
        ForegroundType::Light
    }
}
