use core_logic::{
    ColorResult,
    data::{ThemeExtended, srgb_hex},
    generate_theme_extended,
    palette::{IntoColor, Oklch, Srgb},
    theme::PaletteColorVariant,
};
use minijinja::{Environment, Error, context};

#[tokio::main]
async fn main() {
    match generate_theme_extended().await {
        ColorResult::Ok(theme) => compile_config_files(theme),
        ColorResult::NetworkError => {
            eprintln!("Network error");
            std::process::exit(1);
        }
        ColorResult::ParseError => {
            eprintln!("Parse error");
            std::process::exit(1);
        }
        ColorResult::PaletteDataParseError => {
            eprintln!("Error parsing palette data");
            std::process::exit(1);
        }
    }
}

fn compile_config_files(theme: ThemeExtended) -> () {
    let mut env = Environment::new();

    let primary_name = theme.primary_palette.name.as_str();
    let primary_variants = theme.primary_palette.variants();
    let primary_50 = oklch_as_hex(get_variant(primary_name, "50", &primary_variants));
    let primary_100 = oklch_as_hex(get_variant(primary_name, "100", &primary_variants));
    let primary_200 = oklch_as_hex(get_variant(primary_name, "200", &primary_variants));
    let primary_300 = oklch_as_hex(get_variant(primary_name, "300", &primary_variants));
    let primary_400 = oklch_as_hex(get_variant(primary_name, "400", &primary_variants));
    let primary_500 = oklch_as_hex(get_variant(primary_name, "500", &primary_variants));
    let primary_600 = oklch_as_hex(get_variant(primary_name, "600", &primary_variants));
    let primary_700 = oklch_as_hex(get_variant(primary_name, "700", &primary_variants));
    let primary_800 = oklch_as_hex(get_variant(primary_name, "800", &primary_variants));
    let primary_900 = oklch_as_hex(get_variant(primary_name, "900", &primary_variants));
    let primary_950 = oklch_as_hex(get_variant(primary_name, "950", &primary_variants));

    let secondary_name = theme.secondary_palette.name.as_str();
    let secondary_variants = theme.secondary_palette.variants();
    let secondary_50 = oklch_as_hex(get_variant(secondary_name, "50", &secondary_variants));
    let secondary_100 = oklch_as_hex(get_variant(secondary_name, "100", &secondary_variants));
    let secondary_200 = oklch_as_hex(get_variant(secondary_name, "200", &secondary_variants));
    let secondary_300 = oklch_as_hex(get_variant(secondary_name, "300", &secondary_variants));
    let secondary_400 = oklch_as_hex(get_variant(secondary_name, "400", &secondary_variants));
    let secondary_500 = oklch_as_hex(get_variant(secondary_name, "500", &secondary_variants));
    let secondary_600 = oklch_as_hex(get_variant(secondary_name, "600", &secondary_variants));
    let secondary_700 = oklch_as_hex(get_variant(secondary_name, "700", &secondary_variants));
    let secondary_800 = oklch_as_hex(get_variant(secondary_name, "800", &secondary_variants));
    let secondary_900 = oklch_as_hex(get_variant(secondary_name, "900", &secondary_variants));
    let secondary_950 = oklch_as_hex(get_variant(secondary_name, "950", &secondary_variants));

    let tertiary_name = theme.tertiary_palette.name.as_str();
    let tertiary_variants = theme.tertiary_palette.variants();
    let tertiary_50 = oklch_as_hex(get_variant(tertiary_name, "50", &tertiary_variants));
    let tertiary_100 = oklch_as_hex(get_variant(tertiary_name, "100", &tertiary_variants));
    let tertiary_200 = oklch_as_hex(get_variant(tertiary_name, "200", &tertiary_variants));
    let tertiary_300 = oklch_as_hex(get_variant(tertiary_name, "300", &tertiary_variants));
    let tertiary_400 = oklch_as_hex(get_variant(tertiary_name, "400", &tertiary_variants));
    let tertiary_500 = oklch_as_hex(get_variant(tertiary_name, "500", &tertiary_variants));
    let tertiary_600 = oklch_as_hex(get_variant(tertiary_name, "600", &tertiary_variants));
    let tertiary_700 = oklch_as_hex(get_variant(tertiary_name, "700", &tertiary_variants));
    let tertiary_800 = oklch_as_hex(get_variant(tertiary_name, "800", &tertiary_variants));
    let tertiary_900 = oklch_as_hex(get_variant(tertiary_name, "900", &tertiary_variants));
    let tertiary_950 = oklch_as_hex(get_variant(tertiary_name, "950", &tertiary_variants));

    let theme_context = context! {
        original => srgb_as_hex(theme.original_color),
        primary_50,
        primary_100,
        primary_200,
        primary_300,
        primary_400,
        primary_500,
        primary_600,
        primary_700,
        primary_800,
        primary_900,
        primary_950,
        secondary_50,
        secondary_100,
        secondary_200,
        secondary_300,
        secondary_400,
        secondary_500,
        secondary_600,
        secondary_700,
        secondary_800,
        secondary_900,
        secondary_950,
        tertiary_50,
        tertiary_100,
        tertiary_200,
        tertiary_300,
        tertiary_400,
        tertiary_500,
        tertiary_600,
        tertiary_700,
        tertiary_800,
        tertiary_900,
        tertiary_950,
    };

    /*
     * todo:
     *
     * add template:
     * ```
     * let tmux_template = fs::read_to_string("/path/to/templates/tmux.conf.j2").unwrap();
     * env.add_template("tmux", &tmux_template).unwrap();
     * ```
     *
     * render template:
     * ```
     * let rendered_tmux = env.get_template("tmux").unwrap().render(&theme_context).unwrap();
     * ```
     *
     * save file:
     * ```
     * fs::write("/path/to/destination/tmux.conf", rendered_tmux).unwrap();
     * ```
     *
     * add the variables to the templates like this: `{{ primary_50 }}`
     * call this from Bash, so it can expand home. Save the files from Rust into a temporary
     * directory that can be deleted from Bash later. `foreground_type` is currently not used.
     */
}

fn srgb_as_hex(color: Srgb<u8>) -> String {
    format!("#{:02X}{:02X}{:02X}", color.red, color.green, color.blue)
}

fn oklch_as_hex(color: Oklch) -> String {
    let rgb: Srgb<f32> = color.into_color();
    let result: Srgb<u8> = rgb.into_format();

    srgb_as_hex(result)
}

fn get_variant(
    color_name: &str,
    variant_name: &str,
    palette: &[&PaletteColorVariant; 11],
) -> Oklch {
    palette
        .iter()
        .find(|v| v.name == format!("{}_{}", color_name, variant_name))
        .unwrap()
        .color
}
