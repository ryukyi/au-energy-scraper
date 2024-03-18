use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use chrono_tz::Tz;
use serde::{self, Deserialize, Deserializer};

#[derive(Debug)]
pub struct DateTimeWithTimeZone(pub DateTime<Utc>);

pub fn deserialize_datetime_with_timezone<'de, D>(
    deserializer: D,
    timezone: Tz,
    format_from: &str,
) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?
        .trim()
        .trim_matches('"')
        .to_string(); // Trim whitespace
    match NaiveDateTime::parse_from_str(&s, format_from) {
        Ok(naive_datetime) => {
            let timezone_aware_datetime = timezone
                .from_local_datetime(&naive_datetime)
                .single()
                .ok_or_else(|| {
                    serde::de::Error::custom("Could not convert to timezone-aware datetime")
                })?;
            Ok(timezone_aware_datetime.with_timezone(&Utc))
        }
        Err(_) => {
            println!("Failed to parse datetime string: '{}'", s); // Log the problematic string
            Err(serde::de::Error::custom(
                "Could not parse datetime string with any provided format",
            ))
        }
    }
}

pub fn deserialize_with_sydney_timezone_and_format<'de, D>(
    deserializer: D,
) -> Result<DateTimeWithTimeZone, D::Error>
where
    D: Deserializer<'de>,
{
    // Specify your timezone and format here
    let tz: Tz = chrono_tz::Australia::Sydney;
    let format: &str = "%Y/%m/%d %H:%M:%S";

    // Now call your original function with all required arguments
    let datetime_utc_result = deserialize_datetime_with_timezone(deserializer, tz, format)?;

    // Wrap the DateTime<Utc> in DateTimeWithTimeZone before returning
    Ok(DateTimeWithTimeZone(datetime_utc_result))
}
