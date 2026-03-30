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

fn load_palette(json_data: &str) -> Vec<PaletteColor> {
    let parsed: JsonColors = serde_json::from_str(json_data).expect("Invalid JSON data");
    let mut result: Vec<PaletteColor> = Vec::new();

    for (hue, variants) in parsed {
        let variant_50 = extract_json_variant(&variants, &hue, "50");
        let variant_100 = extract_json_variant(&variants, &hue, "100");
        let variant_200 = extract_json_variant(&variants, &hue, "200");
        let variant_300 = extract_json_variant(&variants, &hue, "300");
        let variant_400 = extract_json_variant(&variants, &hue, "400");
        let variant_500 = extract_json_variant(&variants, &hue, "500");
        let variant_600 = extract_json_variant(&variants, &hue, "600");
        let variant_700 = extract_json_variant(&variants, &hue, "700");
        let variant_800 = extract_json_variant(&variants, &hue, "800");
        let variant_900 = extract_json_variant(&variants, &hue, "900");
        let variant_950 = extract_json_variant(&variants, &hue, "950");

        let color = PaletteColor {
            name: hue,
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

    result
}

fn extract_json_variant(
    variants: &HashMap<String, String>,
    hue: &String,
    weight: &str,
) -> PaletteColorVariant {
    PaletteColorVariant {
        name: format!("{}_{}", hue, weight),
        color: extract_oklch(
            &hue,
            weight,
            variants
                .get(weight)
                .expect(format!("Missing variant {} for color {}", weight, hue).as_str()),
        ),
    }
}

fn extract_oklch(hue: &String, weight: &str, value: &str) -> Oklch {
    let values = value
        .trim()
        .strip_prefix("oklch(")
        .expect(
            format!(
                "Expected {}_{} to start with \"oklch(\", found {}",
                hue, weight, value
            )
            .as_str(),
        )
        .strip_suffix(")")
        .expect(
            format!(
                "Expected {}_{} to end with \")\", found {}",
                hue, weight, value
            )
            .as_str(),
        );

    let parts: Vec<&str> = values.split_whitespace().collect();

    if parts.len() >= 3 {
        let l_value = parts[0].strip_suffix("%").expect(
            format!(
                "Expected L value of {}_{} to end with \"%\", found {}",
                hue, weight, parts[0]
            )
            .as_str(),
        );

        let l_percentage = l_value.parse::<f32>().expect(
            format!(
                "Expected L value of {}_{} to be a number, found {}",
                hue, weight, l_value
            )
            .as_str(),
        );

        let l = l_percentage / 100.0;

        let c = parts[1].parse::<f32>().expect(
            format!(
                "Expected C value of {}_{} to be a number, found {}",
                hue, weight, parts[1]
            )
            .as_str(),
        );

        let h = parts[2].parse::<f32>().expect(
            format!(
                "Expected H value of {}_{} to be a number, found {}",
                hue, weight, parts[2]
            )
            .as_str(),
        );

        Oklch::new(l, c, h)
    } else {
        println!("Unable to split {}_{}, found {}", hue, weight, values);
        panic!("Failed, see error above")
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

        load_palette(&json_content);
    }
}
