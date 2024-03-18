use crate::parsers::generic_csv_parser::ParsedData;
use serde::{self, Deserialize, Deserializer};
use chrono::{DateTime, NaiveDateTime, Utc};
use chrono_tz::Australia::Sydney; // automatically adjusts for DST
use serde::de::Error;
use chrono::TimeZone;
use std::fmt;

const DATE_FORMAT_FROM: &str = "%Y/%m/%d %H:%M:%S";

fn deserialize_sydney_date_to_utc<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let naive = NaiveDateTime::parse_from_str(&s, DATE_FORMAT_FROM)
        .map_err(D::Error::custom)?;
    let sydney_date = Sydney.from_local_datetime(&naive).single().ok_or_else(|| D::Error::custom("Invalid Sydney date/time"))?;
    Ok(sydney_date.with_timezone(&Utc))
}

#[derive(Debug)]
pub struct RooftopPvActualParsedData {
    information_rows: Vec<RooftopPvActualInformationRow>,
    data_rows: Vec<RooftopPvActualDataRow>,
}

impl ParsedData for RooftopPvActualParsedData {
    type InformationRow = RooftopPvActualInformationRow;
    type DataRow = RooftopPvActualDataRow;

    fn new() -> Self {
        RooftopPvActualParsedData {
            information_rows: Vec::new(),
            data_rows: Vec::new(),
        }
    }

    fn add_information_row(&mut self, row: Self::InformationRow) {
        self.information_rows.push(row);
    }

    fn add_data_row(&mut self, row: Self::DataRow) {
        self.data_rows.push(row);
    }
}

// Define the structs for information and data rows
// Updated struct to represent the Information row (I row)
#[derive(Debug, Deserialize)]
pub struct RooftopPvActualInformationRow {
    // The first three fields are constant and represent metadata about the row
    #[serde(rename = "CSVROWIDENTIFIER")]
    csv_row_identifier: String, // "I"
    #[serde(rename = "CATEGORY")]
    category: String,
    #[serde(rename = "REPORT_TYPE")]
    report_type: String,
    #[serde(rename = "REPORT_TYPE_INT")]
    report_type_int: String,
    #[serde(rename = "INTERVAL_DATETIME")]
    interval_datetime: String,
    #[serde(rename = "REGIONID")]
    regionid: String,
    #[serde(rename = "POWER")]
    power: String, // Assuming power can be a floating-point number
    #[serde(rename = "QI")]
    qi: String,
    #[serde(rename = "TYPE")]
    type_: String,
    #[serde(rename = "LASTCHANGED")]
    lastchanged: String,
}

impl fmt::Display for RooftopPvActualInformationRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RooftopPvActualInformationRow: {{ csv_row_identifier: {}, category: {}, report_type: {}, report_type_int: {}, interval_datetime: {}, regionid: {}, power: {}, qi: {}, type_: {}, lastchanged: {} }}",
            self.csv_row_identifier,
            self.category,
            self.report_type,
            self.report_type_int,
            self.interval_datetime,
            self.regionid,
            self.power,
            self.qi,
            self.type_,
            self.lastchanged
        )
    }
}

// Updated struct to represent the Data row (D row)
#[derive(Debug, Deserialize)]
pub struct RooftopPvActualDataRow {
    // The first three fields are constant and represent metadata about the row
    #[serde(rename = "CSVROWIDENTIFIER")]
    csv_row_identifier: String, // "I"
    #[serde(rename = "CATEGORY")]
    category: String,
    #[serde(rename = "REPORT_TYPE")]
    report_type: String,
    #[serde(rename = "REPORT_TYPE_INT")]
    report_type_int: String,
    #[serde(rename = "INTERVAL_DATETIMEZONE")]
    #[serde(deserialize_with = "deserialize_sydney_date_to_utc")]
    interval_datetime: DateTime<Utc>, // Adjusted to use DateTimeWithTimeZone
    #[serde(rename = "REGIONID")]
    regionid: String,
    #[serde(rename = "POWER")]
    power: f64, // Assuming power can be a floating-point number
    #[serde(rename = "QI")]
    qi: u32,
    #[serde(rename = "TYPE")]
    type_: String,
    #[serde(rename = "LASTCHANGED")]
    lastchanged: String,
}

impl fmt::Display for RooftopPvActualDataRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Example implementation, adjust according to your needs
        write!(
            f,
            "RooftopPvActualDataRow: {{ {}, {}, {}, {}, {}, {}, {}, {}, {}, {} }}",
            self.csv_row_identifier,
            self.category,
            self.report_type,
            self.report_type_int,
            self.interval_datetime,
            self.regionid,
            self.power,
            self.qi,
            self.type_,
            self.lastchanged
        )
    }
}

impl fmt::Display for RooftopPvActualParsedData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RooftopPvActualParsedData:\n")?;
        for info_row in &self.information_rows {
            // Utilize the Display implementation of RooftopPvActualInformationRow
            write!(f, "\t{}\n", info_row)?;
        }

        for data_row in &self.data_rows {
            // Utilize the Display implementation of RooftopPvActualDataRow
            write!(f, "\t{}\n", data_row)?;
        }

        Ok(())
    }
}