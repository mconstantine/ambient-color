use std::fs;

use core_logic::{
    ColorResult,
    data::{ForegroundType, ThemeExtended},
    generate_theme_extended,
    palette::{IntoColor, Oklch, Srgb},
    theme::PaletteColorVariant,
};
use minijinja::{Environment, context};

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

    env.add_filter("no_hashtag", no_hashtag);

    let primary_name = theme.primary_palette.name.as_str();
    let primary_variants = theme.primary_palette.variants();
    let primary = oklch_as_hex(theme.primary_variant.color);
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

    let opposite_name = theme.opposite_palette.name.as_str();
    let opposite_variants = theme.opposite_palette.variants();
    let opposite = oklch_as_hex(theme.opposite_variant.color);
    let opposite_50 = oklch_as_hex(get_variant(opposite_name, "50", &opposite_variants));
    let opposite_100 = oklch_as_hex(get_variant(opposite_name, "100", &opposite_variants));
    let opposite_200 = oklch_as_hex(get_variant(opposite_name, "200", &opposite_variants));
    let opposite_300 = oklch_as_hex(get_variant(opposite_name, "300", &opposite_variants));
    let opposite_400 = oklch_as_hex(get_variant(opposite_name, "400", &opposite_variants));
    let opposite_500 = oklch_as_hex(get_variant(opposite_name, "500", &opposite_variants));
    let opposite_600 = oklch_as_hex(get_variant(opposite_name, "600", &opposite_variants));
    let opposite_700 = oklch_as_hex(get_variant(opposite_name, "700", &opposite_variants));
    let opposite_800 = oklch_as_hex(get_variant(opposite_name, "800", &opposite_variants));
    let opposite_900 = oklch_as_hex(get_variant(opposite_name, "900", &opposite_variants));
    let opposite_950 = oklch_as_hex(get_variant(opposite_name, "950", &opposite_variants));

    let secondary_name = theme.secondary_palette.name.as_str();
    let secondary_variants = theme.secondary_palette.variants();
    let secondary = oklch_as_hex(theme.secondary_variant.color);
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
    let tertiary = oklch_as_hex(theme.tertiary_variant.color);
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

    let neutral_variants = theme.neutral_palette.variants();
    let neutral_50 = oklch_as_hex(get_variant("neutral", "50", &neutral_variants));
    let neutral_100 = oklch_as_hex(get_variant("neutral", "100", &neutral_variants));
    let neutral_200 = oklch_as_hex(get_variant("neutral", "200", &neutral_variants));
    let neutral_300 = oklch_as_hex(get_variant("neutral", "300", &neutral_variants));
    let neutral_400 = oklch_as_hex(get_variant("neutral", "400", &neutral_variants));
    let neutral_500 = oklch_as_hex(get_variant("neutral", "500", &neutral_variants));
    let neutral_600 = oklch_as_hex(get_variant("neutral", "600", &neutral_variants));
    let neutral_700 = oklch_as_hex(get_variant("neutral", "700", &neutral_variants));
    let neutral_800 = oklch_as_hex(get_variant("neutral", "800", &neutral_variants));
    let neutral_900 = oklch_as_hex(get_variant("neutral", "900", &neutral_variants));
    let neutral_950 = oklch_as_hex(get_variant("neutral", "950", &neutral_variants));

    let primary_foreground = match theme.primary_foreground_type {
        ForegroundType::Light => primary_50.clone(),
        ForegroundType::Dark => primary_950.clone(),
    };

    let primary_500_foreground = match theme.primary_500_foreground_type {
        ForegroundType::Light => primary_50.clone(),
        ForegroundType::Dark => primary_950.clone(),
    };

    let opposite_foreground = match theme.opposite_foreground_type {
        ForegroundType::Light => opposite_50.clone(),
        ForegroundType::Dark => opposite_950.clone(),
    };

    let opposite_500_foreground = match theme.opposite_500_foreground_type {
        ForegroundType::Light => opposite_50.clone(),
        ForegroundType::Dark => opposite_950.clone(),
    };

    let secondary_foreground = match theme.secondary_foreground_type {
        ForegroundType::Light => secondary_50.clone(),
        ForegroundType::Dark => secondary_950.clone(),
    };

    let secondary_500_foreground = match theme.secondary_500_foreground_type {
        ForegroundType::Light => secondary_50.clone(),
        ForegroundType::Dark => secondary_950.clone(),
    };

    let tertiary_foreground = match theme.tertiary_foreground_type {
        ForegroundType::Light => tertiary_50.clone(),
        ForegroundType::Dark => tertiary_950.clone(),
    };

    let tertiary_500_foreground = match theme.tertiary_500_foreground_type {
        ForegroundType::Light => tertiary_50.clone(),
        ForegroundType::Dark => tertiary_950.clone(),
    };

    let theme_context = context! {
        original => srgb_as_hex(theme.original_color),
        primary,
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
        primary_foreground,
        primary_500_foreground,
        opposite,
        opposite_50,
        opposite_100,
        opposite_200,
        opposite_300,
        opposite_400,
        opposite_500,
        opposite_600,
        opposite_700,
        opposite_800,
        opposite_900,
        opposite_950,
        opposite_foreground,
        opposite_500_foreground,
        secondary,
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
        secondary_foreground,
        secondary_500_foreground,
        tertiary,
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
        tertiary_foreground,
        tertiary_500_foreground,
        neutral_50,
        neutral_100,
        neutral_200,
        neutral_300,
        neutral_400,
        neutral_500,
        neutral_600,
        neutral_700,
        neutral_800,
        neutral_900,
        neutral_950,
    };

    let tmux_template =
        fs::read_to_string("./templates/tmux.conf").expect("tmux template not found");

    let ashell_template =
        fs::read_to_string("./templates/ashell.toml").expect("ashell template not found");

    let clipse_template =
        fs::read_to_string("./templates/clipse_theme.json").expect("clipse template not found");

    let foot_template =
        fs::read_to_string("./templates/foot.ini").expect("feet template not found");

    let hyprland_template =
        fs::read_to_string("./templates/hyprland.conf").expect("hyprland template not found");

    let fnott_template =
        fs::read_to_string("./templates/fnott.ini").expect("fnott template not found");

    let hyprlock_template =
        fs::read_to_string("./templates/hyprlock.conf").expect("hyprlock template not found");

    let gtk_template = fs::read_to_string("./templates/gtk.css").expect("gtk template not found");

    let rofi_template =
        fs::read_to_string("./templates/rofi.rasi").expect("rofi template not found");

    let nvim_template =
        fs::read_to_string("./templates/colors.lua").expect("nvim template not found");

    env.add_template("tmux", &tmux_template)
        .expect("Unable to add tmux template");

    env.add_template("ashell", &ashell_template)
        .expect("Unable to add ashell template");

    env.add_template("clipse", &clipse_template)
        .expect("Unable to add clipse template");

    env.add_template("foot", &foot_template)
        .expect("Unable to add foot template");

    env.add_template("hyprland", &hyprland_template)
        .expect("Unable to add hyprland template");

    env.add_template("fnott", &fnott_template)
        .expect("Unable to add fnott template");

    env.add_template("hyprlock", &hyprlock_template)
        .expect("Unable to add hyprlock template");

    env.add_template("gtk", &gtk_template)
        .expect("Unable to add gtk template");

    env.add_template("rofi", &rofi_template)
        .expect("Unable to add rofi template");

    env.add_template("nvim", &nvim_template)
        .expect("Unable to add nvim template");

    let rendered_tmux = env
        .get_template("tmux")
        .unwrap()
        .render(&theme_context)
        .expect("Unable to render tmux template");

    let rendered_ashell = env
        .get_template("ashell")
        .unwrap()
        .render(&theme_context)
        .expect("Unable to render ashell template");

    let rendered_clipse = env
        .get_template("clipse")
        .unwrap()
        .render(&theme_context)
        .expect("Unable to render clipse template");

    let rendered_foot = env
        .get_template("foot")
        .unwrap()
        .render(&theme_context)
        .expect("Unable to render foot template");

    let rendered_hyprland = env
        .get_template("hyprland")
        .unwrap()
        .render(&theme_context)
        .expect("Unable to render hyprland template");

    let rendered_fnott = env
        .get_template("fnott")
        .unwrap()
        .render(&theme_context)
        .expect("Unable to render fnott template");

    let rendered_hyprlock = env
        .get_template("hyprlock")
        .unwrap()
        .render(&theme_context)
        .expect("Unable to render hyprlock template");

    let rendered_gtk = env
        .get_template("gtk")
        .unwrap()
        .render(&theme_context)
        .expect("Unable to render gtk template");

    let rendered_rofi = env
        .get_template("rofi")
        .unwrap()
        .render(&theme_context)
        .expect("Unable to render rofi template");

    let rendered_nvim = env
        .get_template("nvim")
        .unwrap()
        .render(&theme_context)
        .expect("Unable to render nvim template");

    fs::write("./rendered_templates/tmux.conf", rendered_tmux)
        .expect("Unable to save rendered tmux template");

    fs::write("./rendered_templates/ashell.toml", rendered_ashell)
        .expect("Unable to save rendered ashell template");

    fs::write("./rendered_templates/clipse_theme.json", rendered_clipse)
        .expect("Unable to save rendered clipse template");

    fs::write("./rendered_templates/foot.ini", rendered_foot)
        .expect("Unable to save rendered foot template");

    fs::write("./rendered_templates/hyprland.conf", rendered_hyprland)
        .expect("Unable to save rendered hyprland template");

    fs::write("./rendered_templates/fnott.ini", rendered_fnott)
        .expect("Unable to save rendered fnott template");

    fs::write("./rendered_templates/hyprlock.conf", rendered_hyprlock)
        .expect("Unable to save rendered hyprlock template");

    fs::write("./rendered_templates/gtk.css", rendered_gtk)
        .expect("Unable to save rendered gtk template");

    fs::write("./rendered_templates/rofi.rasi", rendered_rofi)
        .expect("Unable to save rendered rofi template");

    fs::write("./rendered_templates/colors.lua", rendered_nvim)
        .expect("Unable to save rendered nvim template");
}

fn no_hashtag(string: String) -> String {
    String::from(string.strip_prefix("#").unwrap())
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
