use core_logic::{ColorResult, generate_color};

#[tokio::main]
async fn main() {
    match generate_color().await {
        ColorResult::Ok(data) => match serde_json::to_string(&data) {
            Ok(json_string) => {
                println!("{}", json_string);
                std::process::exit(0);
            }
            Err(error) => {
                eprintln!("Error formatting JSON: {}", error);
                std::process::exit(1);
            }
        },
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
