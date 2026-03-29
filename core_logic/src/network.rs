use reqwest::Error as ReqwestError;
use std::num::ParseIntError;

use crate::data::WttrData;

pub enum WttrError {
    NetworkError(ReqwestError),
    ParseError(ParseIntError),
}
impl From<ReqwestError> for WttrError {
    fn from(value: ReqwestError) -> Self {
        WttrError::NetworkError(value)
    }
}
impl From<ParseIntError> for WttrError {
    fn from(value: ParseIntError) -> Self {
        WttrError::ParseError(value)
    }
}

pub async fn fetch_wttr_data() -> Result<WttrData, WttrError> {
    // Example: "+18°C"
    let response = reqwest::get("https://wttr.in?format=%t")
        .await?
        .text()
        .await?;

    Ok(WttrData {
        temperature: parse_temperature(&response)?,
    })
}

/**
 * Takes a string like "+18°C" and turns it into a number like 18
 */
fn parse_temperature(temperature: &str) -> Result<i8, ParseIntError> {
    let numeric_part = temperature
        .chars()
        .filter(|char| char.is_ascii_digit() || *char == '-')
        .collect::<String>();

    numeric_part.parse()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_temperature() {
        assert_eq!(parse_temperature("-1°C").unwrap(), -1);
        assert_eq!(parse_temperature("+18°C").unwrap(), 18);
    }
}
