use crate::{
    data::WttrData,
    network::{WttrError, fetch_wttr_data},
};

pub mod data;
mod network;

pub enum ColorResult {
    Ok(WttrData),
    NetworkError,
    ParseError,
}

pub async fn generate_color() -> ColorResult {
    match fetch_wttr_data().await {
        Ok(data) => ColorResult::Ok(data),
        Err(WttrError::NetworkError(error)) => {
            println!("Network error: {}", error);

            ColorResult::NetworkError
        }
        Err(WttrError::ParseError(error)) => {
            println!("Parse error: {}", error);

            ColorResult::ParseError
        }
    }
}
