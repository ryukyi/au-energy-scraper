use chrono::{Duration, NaiveDate, NaiveDateTime, Timelike};
use std::convert::TryFrom;

use std::fmt;

pub struct TimestampStrftime(String);

impl TryFrom<&str> for TimestampStrftime {
    type Error = chrono::format::ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        NaiveDateTime::parse_from_str(value, "%Y%m%d%H%M%S")?;
        Ok(TimestampStrftime(value.to_owned()))
    }
}

impl fmt::Display for TimestampStrftime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TimestampStrftimeRange ({})", self.0)
    }
}

pub enum Interval {
    FiveMinutes,
    ThirtyMinutes,
}

impl Interval {
    fn to_duration(&self) -> Duration {
        match self {
            Interval::FiveMinutes => Duration::try_seconds(5 * 60).expect("Invalid duration"), // 5 minutes in seconds
            Interval::ThirtyMinutes => Duration::try_seconds(30 * 60).expect("Invalid duration"),
        }
    }

    fn seconds(&self) -> i64 {
        match self {
            Interval::FiveMinutes => 5 * 60,    // 5 minutes in seconds
            Interval::ThirtyMinutes => 30 * 60, // 30 minutes in seconds
        }
    }
}

pub struct TimestampGenerator {
    start_date: NaiveDateTime,
    end_date: NaiveDateTime,
    increment: Interval,
}

impl TimestampGenerator {
    pub fn new(start_date: NaiveDateTime, end_date: NaiveDateTime, increment: Interval) -> Self {
        let mut generator = TimestampGenerator {
            start_date,
            end_date,
            increment,
        };

        generator.adjust_start_date();
        generator.adjust_end_date();

        generator
    }

    fn adjust_start_date(&mut self) {
        let interval_seconds = self.increment.seconds();
        let start_adjustment = (self.start_date.minute() as i64 * 60
            + self.start_date.second() as i64)
            % interval_seconds;
        if start_adjustment != 0 {
            self.start_date = self.start_date
                - Duration::try_seconds(start_adjustment)
                    .expect("Couldn't subtract seconds from start date");
        }
    }

    fn adjust_end_date(&mut self) {
        let interval_seconds = self.increment.seconds();
        let end_adjustment = interval_seconds
            - ((self.end_date.minute() as i64 * 60 + self.end_date.second() as i64)
                % interval_seconds);
        if end_adjustment != interval_seconds {
            self.end_date +=
                Duration::try_seconds(end_adjustment).expect("Couldn't add seconds to end date");
        }
    }

    pub fn generate(&self) -> Vec<TimestampStrftime> {
        let interval_duration = self.increment.to_duration();
        let mut date = self.start_date;
        let mut timestamps = Vec::new();

        while date <= self.end_date {
            if let Ok(ts) =
                TimestampStrftime::try_from(date.format("%Y%m%d%H%M%S").to_string().as_str())
            {
                timestamps.push(ts);
            }
            date += interval_duration;
        }

        timestamps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp_strftime_range_try_from_valid() {
        let valid_str = "20230101123045"; // YYYYMMDDHHMMSS format
        let result = TimestampStrftime::try_from(valid_str);
        assert!(result.is_ok());
        let timestamp = result.unwrap();
        assert_eq!(
            timestamp.to_string(),
            "TimestampStrftimeRange (20230101123045)"
        );
    }

    #[test]
    fn test_timestamp_strftime_range_try_from_invalid() {
        let invalid_str = "not a timestamp";
        let result = TimestampStrftime::try_from(invalid_str);
        assert!(result.is_err());
    }

    #[test]
    fn test_interval_to_duration_five_minutes() {
        let interval = Interval::FiveMinutes;
        let duration = interval.to_duration();
        assert_eq!(
            duration,
            Duration::try_seconds(5 * 60).expect("Failed to create duration for 5 minutes")
        );
    }

    #[test]
    fn test_interval_to_duration_thirty_minutes() {
        let interval = Interval::ThirtyMinutes;
        let duration = interval.to_duration();
        assert_eq!(
            duration,
            Duration::try_seconds(30 * 60).expect("Failed to create duration for 30 minutes")
        );
    }

    #[test]
    fn test_interval_seconds_five_minutes() {
        let interval = Interval::FiveMinutes;
        let seconds = interval.seconds();
        assert_eq!(seconds, 5 * 60);
    }

    #[test]
    fn test_interval_seconds_thirty_minutes() {
        let interval = Interval::ThirtyMinutes;
        let seconds = interval.seconds();
        assert_eq!(seconds, 30 * 60);
    }

    #[test]
    fn test_adjust_start_date() {
        let start_date_opt =
            NaiveDate::from_ymd_opt(2023, 4, 1).and_then(|date| date.and_hms_opt(0, 3, 0)); // 3 minutes past the hour

        // Ensure start_date is valid before proceeding
        let start_date = start_date_opt.expect("Invalid start date");
        let end_date =
            start_date + Duration::try_hours(1).expect("Failed to create duration of 1 hour");
        let mut generator = TimestampGenerator::new(start_date, end_date, Interval::FiveMinutes);

        generator.adjust_start_date();

        assert_eq!(generator.start_date.minute(), 0);
        assert_eq!(generator.start_date.second(), 0);
    }

    #[test]
    fn test_adjust_end_date() {
        let start_date_opt =
            NaiveDate::from_ymd_opt(2023, 4, 1).and_then(|date| date.and_hms_opt(0, 0, 0));

        // Ensure start_date is valid before proceeding
        let start_date = start_date_opt.expect("Invalid start date");
        let end_date = start_date
            + Duration::try_minutes(58).expect("Failed to create duration of 58 minutes");
        let mut generator = TimestampGenerator::new(start_date, end_date, Interval::FiveMinutes);

        generator.adjust_end_date();

        // The end date should be adjusted to the next 5-minute interval
        assert_eq!(generator.end_date.minute(), 0);
        assert_eq!(generator.end_date.hour(), 1);
    }
}
