use palette::{Hsv, IntoColor, Oklab, color_difference::EuclideanDistance};

use crate::theme::{PaletteColor, PaletteColorVariant};

pub fn get_closest_palette_color(reference: &Hsv, palette: &[PaletteColor]) -> PaletteColorVariant {
    let ref_oklab: Oklab = (*reference).into_color();

    palette
        .iter()
        .flat_map(|color| color.variants())
        .min_by(|a, b| {
            let a_oklab: Oklab = a.color.into_color();
            let b_oklab: Oklab = b.color.into_color();
            let distance_a = a_oklab.distance(ref_oklab);
            let distance_b = b_oklab.distance(ref_oklab);

            distance_a.partial_cmp(&distance_b).unwrap()
        })
        .cloned()
        .expect("palette should not be empty")
}
