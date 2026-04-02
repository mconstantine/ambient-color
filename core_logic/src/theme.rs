use crate::data::oklch_hex;
use palette::Oklch;
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
    pub color: Oklch<f32>,
}

#[derive(Clone, Serialize)]
pub struct PaletteColor {
    /**
     * Example: red
     */
    pub name: String,
    pub variant_50: PaletteColorVariant,
    pub variant_100: PaletteColorVariant,
    pub variant_200: PaletteColorVariant,
    pub variant_300: PaletteColorVariant,
    pub variant_400: PaletteColorVariant,
    pub variant_500: PaletteColorVariant,
    pub variant_600: PaletteColorVariant,
    pub variant_700: PaletteColorVariant,
    pub variant_800: PaletteColorVariant,
    pub variant_900: PaletteColorVariant,
    pub variant_950: PaletteColorVariant,
}
impl PaletteColor {
    pub fn variants(&self) -> [&PaletteColorVariant; 11] {
        [
            &self.variant_50,
            &self.variant_100,
            &self.variant_200,
            &self.variant_300,
            &self.variant_400,
            &self.variant_500,
            &self.variant_600,
            &self.variant_700,
            &self.variant_800,
            &self.variant_900,
            &self.variant_950,
        ]
    }
}

pub fn load_palette<'a>(json_data: &str) -> Result<Vec<PaletteColor>, OklchExtractionError> {
    let parsed: JsonColors = serde_json::from_str(json_data).expect("Invalid JSON data");
    let mut result: Vec<PaletteColor> = Vec::new();

    for (hue, variants) in parsed {
        let variant_50 = extract_json_variant(&variants, &hue, "50")?;
        let variant_100 = extract_json_variant(&variants, &hue, "100")?;
        let variant_200 = extract_json_variant(&variants, &hue, "200")?;
        let variant_300 = extract_json_variant(&variants, &hue, "300")?;
        let variant_400 = extract_json_variant(&variants, &hue, "400")?;
        let variant_500 = extract_json_variant(&variants, &hue, "500")?;
        let variant_600 = extract_json_variant(&variants, &hue, "600")?;
        let variant_700 = extract_json_variant(&variants, &hue, "700")?;
        let variant_800 = extract_json_variant(&variants, &hue, "800")?;
        let variant_900 = extract_json_variant(&variants, &hue, "900")?;
        let variant_950 = extract_json_variant(&variants, &hue, "950")?;

        let color = PaletteColor {
            name: hue.clone(),
            variant_50,
            variant_100,
            variant_200,
            variant_300,
            variant_400,
            variant_500,
            variant_600,
            variant_700,
            variant_800,
            variant_900,
            variant_950,
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

    let color = extract_oklch(hue, weight, variant)?;

    Ok(PaletteColorVariant {
        name: format!("{}_{}", hue, weight),
        color,
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
