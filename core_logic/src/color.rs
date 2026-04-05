use chrono::{NaiveTime, TimeDelta, Timelike};

pub fn get_hue(day_of_year: u32) -> f32 {
    let day = day_of_year.clamp(1, 366) as f32;
    let ratio = (day - 1.0) / 365.0;
    let degrees = (210.0 - (ratio * 360.0)).rem_euclid(360.0);

    degrees
}

#[derive(Debug, Copy, Clone)]
pub struct DailyTemperature {
    pub max: i8,
    pub min: i8,
}

pub fn get_chroma(daily_temperature: DailyTemperature, current_temperature: i8) -> f32 {
    let range = (daily_temperature.max - daily_temperature.min) as f32;
    let reference = (current_temperature - daily_temperature.min) as f32;

    let ratio = if range == 0.0 { 0.5 } else { reference / range };
    let clamped_ratio = ratio.clamp(0.0, 1.0);

    let min_chroma = 0.02;
    let max_chroma = 0.20;

    min_chroma + (clamped_ratio * (max_chroma - min_chroma))
}

#[derive(Debug, Copy, Clone)]
pub struct SolarTimes {
    pub sunrise: NaiveTime,
    pub sunset: NaiveTime,
}

pub fn get_luma(solar_times: SolarTimes, now: NaiveTime) -> f32 {
    let one_hour_before_sunrise = solar_times.sunrise - TimeDelta::hours(1);
    let one_hour_after_sunrise = solar_times.sunrise + TimeDelta::hours(1);
    let one_hour_before_sunset = solar_times.sunset - TimeDelta::hours(1);
    let one_hour_after_sunset = solar_times.sunset + TimeDelta::hours(1);

    let min_luma = 0.15;
    let max_luma = 0.98;

    if now < one_hour_before_sunrise || now > one_hour_after_sunset {
        // night
        min_luma
    } else if now > one_hour_after_sunrise && now < one_hour_before_sunset {
        // day
        max_luma
    } else {
        let is_afternoon = now >= one_hour_before_sunset;

        let min = if is_afternoon {
            one_hour_before_sunset
        } else {
            one_hour_before_sunrise
        };

        let max = if is_afternoon {
            one_hour_after_sunset
        } else {
            one_hour_after_sunrise
        };

        // around sunset
        let min = min.num_seconds_from_midnight();
        let max = max.num_seconds_from_midnight();

        let range = max - min;
        let reference = now.num_seconds_from_midnight() - min;
        let ratio = reference as f32 / range as f32;
        let adjusted_ratio = if is_afternoon { 1.0 - ratio } else { ratio };

        min_luma + (adjusted_ratio * (max_luma - min_luma))
    }
}
