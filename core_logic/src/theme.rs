use crate::data::oklch_hex;
use palette::{IntoColor, Oklch, Srgb, color_difference::Wcag21RelativeContrast};
use serde::Serialize;
use std::collections::HashMap;

pub const PALETTE_JSON: &str = include_str!("palette.json");

type JsonColors = HashMap<String, HashMap<String, String>>;

#[derive(Clone, Serialize)]
pub struct PaletteColorVariant {
    /**
     * Example: "red_500"
     */
    pub name: String,

    #[serde(with = "oklch_hex")]
    pub bg: Oklch<f32>,

    #[serde(with = "oklch_hex")]
    pub fg: Oklch<f32>,
}

#[derive(Clone, Serialize)]
pub struct PaletteColor {
    /**
     * Example: red
     */
    pub name: String,
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

pub fn load_palette<'a>(json_data: &str) -> Result<Vec<PaletteColor>, OklchExtractionError> {
    let parsed: JsonColors = serde_json::from_str(json_data).expect("Invalid JSON data");
    let mut result: Vec<PaletteColor> = Vec::new();

    for (hue, variants) in parsed {
        let w50 = extract_json_variant(&variants, &hue, "50")?;
        let w100 = extract_json_variant(&variants, &hue, "100")?;
        let w200 = extract_json_variant(&variants, &hue, "200")?;
        let w300 = extract_json_variant(&variants, &hue, "300")?;
        let w400 = extract_json_variant(&variants, &hue, "400")?;
        let w500 = extract_json_variant(&variants, &hue, "500")?;
        let w600 = extract_json_variant(&variants, &hue, "600")?;
        let w700 = extract_json_variant(&variants, &hue, "700")?;
        let w800 = extract_json_variant(&variants, &hue, "800")?;
        let w900 = extract_json_variant(&variants, &hue, "900")?;
        let w950 = extract_json_variant(&variants, &hue, "950")?;

        let color = PaletteColor {
            name: hue.clone(),
            w50,
            w100,
            w200,
            w300,
            w400,
            w500,
            w600,
            w700,
            w800,
            w900,
            w950,
        };

        result.push(color);
    }

    Ok(result)
}

#[derive(Debug)]
pub struct OklchExtrationErrorData {
    pub hue: String,
    pub weight: String,
    pub received: String,
}

#[derive(Debug)]
pub enum OklchExtractionError {
    MissingVariant { hue: String, weight: String },
    Prefix { data: OklchExtrationErrorData },
    Suffix { data: OklchExtrationErrorData },
    Format { data: OklchExtrationErrorData },
    LFormat { data: OklchExtrationErrorData },
    CFormat { data: OklchExtrationErrorData },
    HFormat { data: OklchExtrationErrorData },
}

fn extract_json_variant(
    variants: &HashMap<String, String>,
    hue: &str,
    weight: &str,
) -> Result<PaletteColorVariant, OklchExtractionError> {
    let variant = variants
        .get(weight)
        .ok_or(OklchExtractionError::MissingVariant {
            hue: String::from(hue),
            weight: String::from(weight),
        })?;

    let lightest_variant = variants
        .get("50")
        .ok_or(OklchExtractionError::MissingVariant {
            hue: String::from(hue),
            weight: String::from(weight),
        })?;

    let darkest_variant = variants
        .get("950")
        .ok_or(OklchExtractionError::MissingVariant {
            hue: String::from(hue),
            weight: String::from(weight),
        })?;

    let bg = extract_oklch(hue, weight, variant)?;
    let dark = extract_oklch(hue, "50", darkest_variant)?;
    let light = extract_oklch(hue, "50", lightest_variant)?;
    let color_rgb: Srgb<f32> = bg.into_color();

    let fg = if color_rgb.relative_luminance().luma > 0.179 {
        dark
    } else {
        light
    };

    Ok(PaletteColorVariant {
        name: format!("{}_{}", hue, weight),
        bg,
        fg,
    })
}

fn extract_oklch(hue: &str, weight: &str, value: &str) -> Result<Oklch, OklchExtractionError> {
    let values = value
        .trim()
        .strip_prefix("oklch(")
        .ok_or(OklchExtractionError::Prefix {
            data: OklchExtrationErrorData {
                hue: String::from(hue),
                weight: String::from(weight),
                received: String::from(value),
            },
        })?
        .strip_suffix(")")
        .ok_or(OklchExtractionError::Suffix {
            data: OklchExtrationErrorData {
                hue: String::from(hue),
                weight: String::from(weight),
                received: String::from(value),
            },
        })?;

    let parts: Vec<&str> = values.split_whitespace().collect();

    if parts.len() >= 3 {
        let l_value = parts[0]
            .strip_suffix("%")
            .ok_or(OklchExtractionError::LFormat {
                data: OklchExtrationErrorData {
                    hue: String::from(hue),
                    weight: String::from(weight),
                    received: String::from(parts[0]),
                },
            })?;

        let l_percentage =
            l_value
                .parse::<f32>()
                .map_err(|_err| OklchExtractionError::LFormat {
                    data: OklchExtrationErrorData {
                        hue: String::from(hue),
                        weight: String::from(weight),
                        received: String::from(l_value),
                    },
                })?;

        let l = l_percentage / 100.0;

        let c = parts[1]
            .parse::<f32>()
            .map_err(|_err| OklchExtractionError::CFormat {
                data: OklchExtrationErrorData {
                    hue: String::from(hue),
                    weight: String::from(weight),
                    received: String::from(parts[1]),
                },
            })?;

        let h = parts[2]
            .parse::<f32>()
            .map_err(|_err| OklchExtractionError::HFormat {
                data: OklchExtrationErrorData {
                    hue: String::from(hue),
                    weight: String::from(weight),
                    received: String::from(parts[2]),
                },
            })?;

        Ok(Oklch::new(l, c, h))
    } else {
        Err(OklchExtractionError::Format {
            data: OklchExtrationErrorData {
                hue: String::from(hue),
                weight: String::from(weight),
                received: String::from(values),
            },
        })
    }
}

#[cfg(test)]
mod test_theme {
    use crate::theme::{PALETTE_JSON, load_palette};

    #[test]
    fn test_load_palette() {
        load_palette(PALETTE_JSON).unwrap();
    }
}
