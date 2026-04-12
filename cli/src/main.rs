use core_logic::{ColorResult, data::Theme, generate_theme};
use directories::ProjectDirs;
use std::{env, fs};

use crate::{templates::compile_config_files, wallpaper::draw_wallpaper};

mod templates;
mod wallpaper;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(|s| s.as_str());

    let theme = match command {
        Some("compile") | Some("draw") => match read_cache() {
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

    match command {
        Some("compile") | None => compile_config_files(&theme),
        _ => (),
    };

    match command {
        Some("draw") => draw_wallpaper(&theme),
        _ => (),
    };
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
