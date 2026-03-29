uniffi::setup_scaffolding!();
use core_logic::{ColorResult, generate_color};

#[derive(uniffi::Enum)]
pub enum AndroidColorResult {
    Ok { temperature: i8 },
    NetworkError,
    ParseError,
}

#[uniffi::export]
pub fn generate_color_android() -> AndroidColorResult {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(generate_color());

    match result {
        ColorResult::Ok(data) => AndroidColorResult::Ok {
            temperature: data.temperature,
        },
        ColorResult::NetworkError => AndroidColorResult::NetworkError,
        ColorResult::ParseError => AndroidColorResult::ParseError,
    }
}
