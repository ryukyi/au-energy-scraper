use chrono::TimeZone;
use chrono::{DateTime, NaiveDateTime, Utc};
use chrono_tz::Australia::Sydney; // automatically adjusts for DST
use serde::de::Error;
use serde::{self, Deserialize, Deserializer};

const DATE_FORMAT_FROM: &str = "%Y/%m/%d %H:%M:%S";

pub fn deserialize_sydney_datetime_to_utc<'de, D>(
    deserializer: D,
) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let naive = NaiveDateTime::parse_from_str(&s, DATE_FORMAT_FROM).map_err(D::Error::custom)?;
    let sydney_date = Sydney
        .from_local_datetime(&naive)
        .single()
        .ok_or_else(|| D::Error::custom("Invalid Sydney date/time"))?;
    Ok(sydney_date.with_timezone(&Utc))
}
