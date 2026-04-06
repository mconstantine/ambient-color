use chrono::NaiveTime;
use serde::Deserialize;

use crate::{
    WeatherData,
    data::{MoonPhase, WeatherCondition},
};

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

    #[serde(rename = "weatherCode")]
    weather_code: String,
}

#[derive(Deserialize)]
struct Astronomy {
    sunrise: String,
    sunset: String,
    moon_phase: String,
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
    let weather_condition = parse_weather_code(&current_condition.weather_code);
    let moon_phase = parse_moon_phase(&astronomy.moon_phase);

    Ok(WeatherData {
        weather_condition,
        max_temperature,
        min_temperature,
        temperature,
        sunrise_time,
        sunset_time,
        moon_phase,
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

fn parse_weather_code(weather_code: &str) -> WeatherCondition {
    // source: https://www.worldweatheronline.com/feed/wwoConditionCodes.txt
    match weather_code {
        "113" => WeatherCondition::Sunny,
        "122" => WeatherCondition::Cloudy,
        "119" => WeatherCondition::Cloudy,
        "116" => WeatherCondition::Cloudy,
        "260" => WeatherCondition::Fog,
        "248" => WeatherCondition::Fog,
        "143" => WeatherCondition::Fog,
        "389" => WeatherCondition::Rain,
        "386" => WeatherCondition::Rain,
        "359" => WeatherCondition::Rain,
        "356" => WeatherCondition::Rain,
        "353" => WeatherCondition::Rain,
        "314" => WeatherCondition::Rain,
        "311" => WeatherCondition::Rain,
        "308" => WeatherCondition::Rain,
        "35" => WeatherCondition::Rain,
        "302" => WeatherCondition::Rain,
        "299" => WeatherCondition::Rain,
        "296" => WeatherCondition::Rain,
        "293" => WeatherCondition::Rain,
        "284" => WeatherCondition::Rain,
        "281" => WeatherCondition::Rain,
        "266" => WeatherCondition::Rain,
        "263" => WeatherCondition::Rain,
        "377" => WeatherCondition::Rain,
        "374" => WeatherCondition::Rain,
        "350" => WeatherCondition::Rain,
        "200" => WeatherCondition::Rain,
        "185" => WeatherCondition::Rain,
        "176" => WeatherCondition::Rain,
        "395" => WeatherCondition::Snow,
        "392" => WeatherCondition::Snow,
        "371" => WeatherCondition::Snow,
        "368" => WeatherCondition::Snow,
        "365" => WeatherCondition::Snow,
        "362" => WeatherCondition::Snow,
        "338" => WeatherCondition::Snow,
        "335" => WeatherCondition::Snow,
        "332" => WeatherCondition::Snow,
        "329" => WeatherCondition::Snow,
        "326" => WeatherCondition::Snow,
        "323" => WeatherCondition::Snow,
        "320" => WeatherCondition::Snow,
        "317" => WeatherCondition::Snow,
        "230" => WeatherCondition::Snow,
        "227" => WeatherCondition::Snow,
        "182" => WeatherCondition::Snow,
        "179" => WeatherCondition::Snow,
        _ => WeatherCondition::Unknown(String::from(weather_code)),
    }
}

fn parse_moon_phase(moon_phase: &str) -> MoonPhase {
    match moon_phase {
        "New Moon" => MoonPhase::NewMoon,
        "Waxing Crescent" => MoonPhase::WaxingCrescent,
        "First Quarter" => MoonPhase::FirstQuarter,
        "Waxing Gibbous" => MoonPhase::WaxingGibbous,
        "Full Moon" => MoonPhase::FullMoon,
        "Waning Gibbous" => MoonPhase::WaningGibbous,
        "Last Quarter" => MoonPhase::LastQuarter,
        "Waning Crescent" => MoonPhase::WaningCrescent,
        _ => MoonPhase::Unknown(String::from(moon_phase)),
    }
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
