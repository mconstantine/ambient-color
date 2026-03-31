use std::collections::HashMap;

use palette::Oklch;

type JsonColors = HashMap<String, HashMap<String, String>>;

struct PaletteColorVariant<T = f32> {
    /**
     * Example: "red_500"
     */
    name: String,
    color: Oklch<T>,
}

struct PaletteColor {
    /**
     * Example: red
     */
    name: String,
    variant_50: PaletteColorVariant,
    variant_100: PaletteColorVariant,
    variant_200: PaletteColorVariant,
    variant_300: PaletteColorVariant,
    variant_400: PaletteColorVariant,
    variant_500: PaletteColorVariant,
    variant_600: PaletteColorVariant,
    variant_700: PaletteColorVariant,
    variant_800: PaletteColorVariant,
    variant_900: PaletteColorVariant,
    variant_950: PaletteColorVariant,
}

fn load_palette<'a>(json_data: &str) -> Result<Vec<PaletteColor>, OklchExtractionError> {
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
struct OklchExtrationErrorData {
    hue: String,
    weight: String,
    received: String,
}

#[derive(Debug)]
enum OklchExtractionError {
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
    use crate::theme::load_palette;
    use std::fs;

    #[test]
    fn test_load_palette() {
        let file_path = "./palette.json";
        let json_content = fs::read_to_string(file_path).expect("Unable to read JSON file");

        load_palette(&json_content).unwrap();
    }
}
