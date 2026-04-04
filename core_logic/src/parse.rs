use chrono::NaiveTime;
use serde::Deserialize;

use crate::WeatherData;

#[derive(Debug)]
pub enum WttrParseError {
    MissingCurrentCondition,
    MissingDailyWeather,
    MissingAstronomyData,
    ParseTemperature { received: String },
    ParseTime { received: String },
}

#[derive(Deserialize)]
struct CurrentCondition {
    #[serde(rename = "temp_C")]
    temperature: String,
}

#[derive(Deserialize)]
struct Astronomy {
    sunrise: String,
    sunset: String,
}

#[derive(Deserialize)]
struct DailyWeather {
    #[serde(rename = "maxtempC")]
    max_temperature: String,
    #[serde(rename = "mintempC")]
    min_temperature: String,
    astronomy: Vec<Astronomy>,
}

#[derive(Deserialize)]
struct WttrResponse {
    current_condition: Vec<CurrentCondition>,
    #[serde(rename = "weather")]
    daily_weather: Vec<DailyWeather>,
}

pub fn parse_wttr_data(data: &str) -> Result<WeatherData, WttrParseError> {
    let parsed: WttrResponse = serde_json::from_str(data).expect("Invalid JSON data");

    let current_condition = parsed
        .current_condition
        .get(0)
        .ok_or(WttrParseError::MissingCurrentCondition)?;

    let daily_weather = parsed
        .daily_weather
        .get(0)
        .ok_or(WttrParseError::MissingDailyWeather)?;

    let astronomy = daily_weather
        .astronomy
        .get(0)
        .ok_or(WttrParseError::MissingAstronomyData)?;

    let min_temperature = parse_temperature(&daily_weather.min_temperature).map_err(|_err| {
        WttrParseError::ParseTemperature {
            received: daily_weather.min_temperature.clone(),
        }
    })?;

    let max_temperature = parse_temperature(&daily_weather.max_temperature)?;
    let temperature = parse_temperature(&current_condition.temperature)?;
    let sunrise_time = parse_time(&astronomy.sunrise)?;
    let sunset_time = parse_time(&astronomy.sunset)?;

    Ok(WeatherData {
        max_temperature,
        min_temperature,
        temperature,
        sunrise_time,
        sunset_time,
    })
}

/**
 * Takes a string like "+18°C" and turns it into a number like 18
 */
fn parse_temperature(temperature: &str) -> Result<i8, WttrParseError> {
    let numeric_part = temperature
        .chars()
        .filter(|char| char.is_ascii_digit() || *char == '-')
        .collect::<String>();

    numeric_part
        .parse()
        .map_err(|_err| WttrParseError::ParseTemperature {
            received: String::from(temperature),
        })
}

fn parse_time(time: &str) -> Result<NaiveTime, WttrParseError> {
    NaiveTime::parse_from_str(time, "%I:%M %p").map_err(|_err| WttrParseError::ParseTime {
        received: String::from(time),
    })
}

#[cfg(test)]
mod test_parse {
    use std::fs;

    use super::*;

    #[test]
    fn test_parse_temperature() {
        assert_eq!(parse_temperature("-1°C").unwrap(), -1);
        assert_eq!(parse_temperature("+18°C").unwrap(), 18);
    }

    #[test]
    fn test_parse_time() {
        assert_eq!(
            parse_time("12:30 AM").unwrap().format("%H:%M").to_string(),
            "00:30"
        );

        assert_eq!(
            parse_time("08:30 AM").unwrap().format("%H:%M").to_string(),
            "08:30"
        );

        assert_eq!(
            parse_time("08:30 PM").unwrap().format("%H:%M").to_string(),
            "20:30"
        );

        assert_eq!(
            parse_time("12:30 PM").unwrap().format("%H:%M").to_string(),
            "12:30"
        );
    }

    #[test]
    fn test_parse_wttr_data() {
        let file_path = "./mock-data.json";
        let data = fs::read_to_string(file_path).expect("Cannot read mock JSON data");

        parse_wttr_data(&data).unwrap();
    }
}
