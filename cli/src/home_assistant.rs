use std::time::Duration;

use core_logic::{
    data::Theme,
    palette::{IntoColor, Srgb},
};
use keyring::Entry;
use reqwest::{ClientBuilder, Method, Response};
use serde_json::{Value, json};

const SERVICE: &str = "house";
const ACCOUNT: &str = "ha-token";

pub fn get_token() -> String {
    let entry = Entry::new(SERVICE, ACCOUNT).expect("Failed to initialize keyring entry.");

    entry
        .get_password()
        .expect("Unable to find access token. Plase run `house auth login`.")
}

async fn send_request(
    token: &str,
    method: Method,
    path: &str,
    data: Option<&Value>,
) -> Result<Response, Box<dyn std::error::Error>> {
    let hosts = [
        "http://192.168.1.142:8123",
        "http://192.168.1.169:8123",
        "https://home.mconst.it",
    ];

    let client = ClientBuilder::new()
        .timeout(Duration::from_millis(500))
        .build()
        .expect("Failed to build reqwest client");

    for host in hosts {
        let url = format!("{}{}", host, path);

        let mut request = client
            .request(method.clone(), url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json");

        if let Some(data) = data {
            request = request.json(data);
        }

        let response = request.send().await;

        match response {
            Ok(response) if response.status().is_success() => {
                return Ok(response);
            }
            Ok(response) => {
                return Err(
                    format!("Received error status from HA API: {}", response.status()).into(),
                );
            }
            Err(_) => continue,
        }
    }

    Err("No HA host available".into())
}

pub async fn update_home_assistant(theme: &Theme) {
    let token = get_token();

    let path = "/api/services/input_text/set_value";
    let rgb_f32: Srgb<f32> = theme.original_color.bg.into_color();
    let rgb: Srgb<u8> = rgb_f32.into_format();
    let string = format!("[{}, {}, {}]", rgb.red, rgb.green, rgb.blue);

    let data = json!({
        "entity_id": "input_text.global_ambient_color",
        "value": string,
    });

    if let Err(error) = send_request(&token, Method::POST, &path, Some(&data)).await {
        eprintln!("{}", error);
    };
}
