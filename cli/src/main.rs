use std::{env, fs, path::Path};

use core_logic::{ColorResult, data::Theme, generate_theme};
use directories::ProjectDirs;
use minijinja::{Environment, Value, context};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let theme = match args.get(1).map(|s| s.as_str()) {
        Some("compile") => match read_cache() {
            Ok(theme) => theme,
            Err(error) => {
                eprintln!("{}", error);
                std::process::exit(1);
            }
        },
        _ => match generate_theme().await {
            ColorResult::Ok(theme) => theme,
            ColorResult::NetworkError => {
                eprintln!("Network error");
                std::process::exit(1);
            }
            ColorResult::ParseError => {
                eprintln!("Parse error");
                std::process::exit(1);
            }
        },
    };

    compile_config_files(&theme);
}

fn compile_config_files(theme: &Theme) -> () {
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

    if let Some(project_dirs) = ProjectDirs::from("it", "mconst", "ambient-color") {
        let config_dir = project_dirs.config_dir();
        let cache_dir = project_dirs.cache_dir();
        let templates_dir = config_dir.join("templates");

        if let Err(error) = fs::create_dir_all(cache_dir) {
            eprintln!("Failed to create cache directory: {}", error);
            return;
        }

        let dirs: Directories = Directories {
            source: &templates_dir,
            destination: cache_dir,
        };

        digest_template(&dirs, &context, &mut env, "ashell", "ashell.toml");
        digest_template(&dirs, &context, &mut env, "clipse", "clipse_theme.json");
        digest_template(&dirs, &context, &mut env, "nvim", "colors.lua");
        digest_template(&dirs, &context, &mut env, "fnott", "fnott.ini");
        digest_template(&dirs, &context, &mut env, "foot", "foot.ini");
        digest_template(&dirs, &context, &mut env, "gtk", "gtk.css");
        digest_template(&dirs, &context, &mut env, "hyprland", "hyprland.conf");
        digest_template(&dirs, &context, &mut env, "hyprlock", "hyprlock.conf");
        digest_template(&dirs, &context, &mut env, "rofi", "rofi.rasi");
        digest_template(&dirs, &context, &mut env, "tmux", "tmux.conf");
        digest_template(&dirs, &context, &mut env, "zsh", ".zshrc");

        match serde_json::to_string(theme) {
            Ok(json) => {
                if let Err(error) = fs::write(cache_dir.join("data.json"), json) {
                    eprintln!("Unable to write data into cache: {}", error);
                }
            }
            Err(error) => eprintln!("Failed to serialize data: {}", error),
        }
    } else {
        eprintln!("Unable to access project directories");
    }
}

struct Directories<'a> {
    source: &'a Path,
    destination: &'a Path,
}

fn digest_template<'a>(
    directories: &Directories<'a>,
    context: &Value,
    env: &mut Environment,
    template_name: &str,
    file_path: &str,
) -> () {
    let template = fs::read_to_string(directories.source.join(file_path))
        .expect(format!("{} template not found", template_name).as_str());

    let rendered = env
        .render_str(&template, context)
        .expect(format!("Unable to render {} template", template_name).as_str());

    fs::write(directories.destination.join(file_path), rendered)
        .expect(format!("Unable to save rendered {} template", template_name).as_str());
}

fn no_hashtag(string: String) -> String {
    String::from(string.strip_prefix("#").unwrap())
}

fn read_cache() -> Result<Theme, String> {
    let project_dirs = ProjectDirs::from("it", "mconst", "ambient-color")
        .ok_or(format!("Unable to access project directories"))?;

    let cache_dir = project_dirs.cache_dir();

    let content_string = fs::read_to_string(cache_dir.join("data.json"))
        .map_err(|error| format!("Unable to read cache file: {}", error))?;

    let theme = serde_json::from_str::<Theme>(&content_string)
        .map_err(|error| format!("Unable to parse cached data into theme: {}", error))?;

    Ok(theme)
}
