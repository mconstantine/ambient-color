use std::fs;

use core_logic::{ColorResult, data::Theme, generate_theme};
use minijinja::{Environment, Value, context};

#[tokio::main]
async fn main() {
    match generate_theme().await {
        ColorResult::Ok(theme) => compile_config_files(theme),
        ColorResult::NetworkError => {
            eprintln!("Network error");
            std::process::exit(1);
        }
        ColorResult::ParseError => {
            eprintln!("Parse error");
            std::process::exit(1);
        }
    }
}

fn compile_config_files(theme: Theme) -> () {
    let mut env = Environment::new();

    env.add_filter("no_hashtag", no_hashtag);

    let context = context! {
        original => theme.original_color,
        primary => theme.primary_palette,
        opposite => theme.opposite_palette,
        secondary => theme.secondary_palette,
        tertiary => theme.tertiary_palette,
        neutral => theme.neutral_palette,
    };

    digest_template(&context, &mut env, "ashell", "ashell.toml");
    digest_template(&context, &mut env, "clipse", "clipse_theme.json");
    digest_template(&context, &mut env, "nvim", "colors.lua");
    digest_template(&context, &mut env, "fnott", "fnott.ini");
    digest_template(&context, &mut env, "foot", "foot.ini");
    digest_template(&context, &mut env, "gtk", "gtk.css");
    digest_template(&context, &mut env, "hyprland", "hyprland.conf");
    digest_template(&context, &mut env, "hyprlock", "hyprlock.conf");
    digest_template(&context, &mut env, "rofi", "rofi.rasi");
    digest_template(&context, &mut env, "tmux", "tmux.conf");
    digest_template(&context, &mut env, "zsh", ".zshrc");
}

fn digest_template(
    context: &Value,
    env: &mut Environment,
    template_name: &str,
    file_path: &str,
) -> () {
    let template = fs::read_to_string(format!("./templates/{}", file_path))
        .expect(format!("{} template not found", template_name).as_str());

    let rendered = env
        .render_str(&template, context)
        .expect(format!("Unable to render {} template", template_name).as_str());

    fs::write(format!("./rendered_templates/{}", file_path), rendered)
        .expect(format!("Unable to save rendered {} template", template_name).as_str());
}

fn no_hashtag(string: String) -> String {
    String::from(string.strip_prefix("#").unwrap())
}
