use chrono::{NaiveTime, TimeDelta, Timelike};
use palette::RgbHue;

pub fn get_hue(day_of_year: u32) -> RgbHue {
    let day = day_of_year.clamp(1, 366) as f32;
    let ratio = (day - 1.0) / 365.0;
    let inverse_ratio = 1.0 - ratio;
    let degrees = (inverse_ratio * 360.0 + 180.0) % 360.0;

    RgbHue::from_degrees(degrees)
}

#[derive(Debug, Copy, Clone)]
pub struct DailyTemperature {
    pub max: i8,
    pub min: i8,
}

pub fn get_saturation(daily_temperature: DailyTemperature, current_temperature: i8) -> f32 {
    let range = (daily_temperature.max - daily_temperature.min) as f32;
    let reference = (current_temperature - daily_temperature.min) as f32;

    if range == 0.0 { 0.5 } else { reference / range }
}

#[derive(Debug, Copy, Clone)]
pub struct SolarTimes {
    pub sunrise: NaiveTime,
    pub sunset: NaiveTime,
}

pub fn get_lightness(solar_times: SolarTimes, now: NaiveTime) -> f32 {
    let one_hour_before_sunrise = solar_times.sunrise - TimeDelta::hours(1);
    let one_hour_after_sunrise = solar_times.sunrise + TimeDelta::hours(1);
    let one_hour_before_sunset = solar_times.sunset - TimeDelta::hours(1);
    let one_hour_after_sunset = solar_times.sunset + TimeDelta::hours(1);

    if now < one_hour_before_sunrise || now > one_hour_after_sunset {
        // night
        0.0
    } else if now > one_hour_after_sunrise && now < one_hour_before_sunset {
        // day
        1.0
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

        if is_afternoon { 1.0 - ratio } else { ratio }
    }
}

#[cfg(test)]
mod test_color {
    use crate::color::{DailyTemperature, SolarTimes, get_hue, get_lightness, get_saturation};
    use approx::assert_abs_diff_eq;
    use chrono::NaiveTime;
    use palette::{Hsv, IntoColor, Srgb};

    #[test]
    fn test_get_hue() {
        // cyan
        let color: Srgb = Hsv::new(get_hue(1), 1.0, 1.0).into_color();

        assert_abs_diff_eq!(color.red, 0.0, epsilon = 0.1);
        assert_abs_diff_eq!(color.green, 1.0, epsilon = 0.1);
        assert_abs_diff_eq!(color.blue, 1.0, epsilon = 0.1);

        // green
        let color: Srgb = Hsv::new(get_hue(61), 1.0, 1.0).into_color();

        assert_abs_diff_eq!(color.red, 0.0, epsilon = 0.1);
        assert_abs_diff_eq!(color.green, 1.0, epsilon = 0.1);
        assert_abs_diff_eq!(color.blue, 0.0, epsilon = 0.1);

        // yellow
        let color: Srgb = Hsv::new(get_hue(121), 1.0, 1.0).into_color();

        assert_abs_diff_eq!(color.red, 1.0, epsilon = 0.1);
        assert_abs_diff_eq!(color.green, 1.0, epsilon = 0.1);
        assert_abs_diff_eq!(color.blue, 0.0, epsilon = 0.1);

        // red
        let color: Srgb = Hsv::new(get_hue(181), 1.0, 1.0).into_color();

        assert_abs_diff_eq!(color.red, 1.0, epsilon = 0.1);
        assert_abs_diff_eq!(color.green, 0.0, epsilon = 0.1);
        assert_abs_diff_eq!(color.blue, 0.0, epsilon = 0.1);

        // magenta
        let color: Srgb = Hsv::new(get_hue(241), 1.0, 1.0).into_color();

        assert_abs_diff_eq!(color.red, 1.0, epsilon = 0.1);
        assert_abs_diff_eq!(color.green, 0.0, epsilon = 0.1);
        assert_abs_diff_eq!(color.blue, 1.0, epsilon = 0.1);

        // blue
        let color: Srgb = Hsv::new(get_hue(301), 1.0, 1.0).into_color();

        assert_abs_diff_eq!(color.red, 0.0, epsilon = 0.1);
        assert_abs_diff_eq!(color.green, 0.0, epsilon = 0.1);
        assert_abs_diff_eq!(color.blue, 1.0, epsilon = 0.1);
    }

    #[test]
    fn test_get_saturation() {
        let daily_temperature = DailyTemperature { max: 20, min: 5 };

        // max daily temperature
        assert_eq!(get_saturation(daily_temperature, 20), 1.0);

        // min daily temperature
        assert_eq!(get_saturation(daily_temperature, 5), 0.0);

        // somewhere in between
        assert_abs_diff_eq!(
            get_saturation(daily_temperature, 10),
            0.333,
            epsilon = 0.001
        );
    }

    #[test]
    fn test_get_lightness() {
        let solar_times = SolarTimes {
            sunrise: NaiveTime::from_hms_opt(7, 0, 0).unwrap(),
            sunset: NaiveTime::from_hms_opt(19, 0, 0).unwrap(),
        };

        // early morning
        assert_eq!(
            get_lightness(solar_times, NaiveTime::from_hms_opt(5, 0, 0).unwrap()),
            0.0
        );

        // before sunrise
        assert_eq!(
            get_lightness(solar_times, NaiveTime::from_hms_opt(6, 30, 0).unwrap()),
            0.25
        );

        // at sunrise
        assert_eq!(
            get_lightness(solar_times, NaiveTime::from_hms_opt(7, 0, 0).unwrap()),
            0.5
        );

        // after sunrise
        assert_eq!(
            get_lightness(solar_times, NaiveTime::from_hms_opt(7, 30, 0).unwrap()),
            0.75
        );

        // morning
        assert_eq!(
            get_lightness(solar_times, NaiveTime::from_hms_opt(8, 0, 0).unwrap()),
            1.0
        );

        // noon
        assert_eq!(
            get_lightness(solar_times, NaiveTime::from_hms_opt(12, 0, 0).unwrap()),
            1.0
        );

        // afternoon
        assert_eq!(
            get_lightness(solar_times, NaiveTime::from_hms_opt(18, 0, 0).unwrap()),
            1.0
        );

        // before sunset
        assert_eq!(
            get_lightness(solar_times, NaiveTime::from_hms_opt(18, 30, 0).unwrap()),
            0.75
        );

        // at sunset
        assert_eq!(
            get_lightness(solar_times, NaiveTime::from_hms_opt(19, 0, 0).unwrap()),
            0.5
        );

        // after sunset
        assert_eq!(
            get_lightness(solar_times, NaiveTime::from_hms_opt(19, 30, 0).unwrap()),
            0.25
        );

        // evening
        assert_eq!(
            get_lightness(solar_times, NaiveTime::from_hms_opt(20, 0, 0).unwrap()),
            0.0
        );

        // night
        assert_eq!(
            get_lightness(solar_times, NaiveTime::from_hms_opt(21, 0, 0).unwrap()),
            0.0
        );
    }
}
