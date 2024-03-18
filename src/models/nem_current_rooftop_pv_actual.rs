use crate::parsers::generic_csv_parser::ParsedData;
use chrono::{DateTime, Utc};
use serde::{self, Deserialize};
use std::fmt;

use crate::common::parser_types::{DataRowTrait, InformationRowTrait, RowAction, RowMatcher};
use crate::time_utils::datetimezone_conversion::deserialize_sydney_datetime_to_utc;

#[derive(Debug)]
pub struct RooftopPvActualMatcher;

impl RowMatcher for RooftopPvActualMatcher {
    fn match_row(&self, record: &csv::StringRecord) -> RowAction {
        if record.get(0) == Some("I") && record.get(2) == Some("ACTUAL") {
            RowAction::InformationRow
        } else if record.get(0) == Some("D") {
            RowAction::DataRow
        } else if record.get(0) == Some("C")
            && record.get(1).unwrap_or("").to_uppercase() == "END OF REPORT"
        {
            RowAction::ControlRow
        } else {
            RowAction::Ignore
        }
    }
}

#[derive(Debug)]
pub struct RooftopPvActualParsedData {
    rows: Vec<(RooftopPvActualInformationRow, Vec<RooftopPvActualDataRow>)>,
    matcher: RooftopPvActualMatcher,
}

impl ParsedData for RooftopPvActualParsedData {
    type InformationRow = RooftopPvActualInformationRow;
    type DataRow = RooftopPvActualDataRow;
    // Specify the type for Matcher
    type Matcher = RooftopPvActualMatcher;

    fn new() -> Self {
        RooftopPvActualParsedData {
            rows: Vec::new(),
            matcher: RooftopPvActualMatcher, // Initialize the matcher
        }
    }

    fn add_rows(&mut self, rows: Vec<(Self::InformationRow, Vec<Self::DataRow>)>) {
        self.rows.extend(rows);
    }

    // Implement the matcher function to return a reference to the matcher instance
    fn matcher(&self) -> &Self::Matcher {
        &self.matcher
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

impl InformationRowTrait for RooftopPvActualInformationRow {}

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
    #[serde(deserialize_with = "deserialize_sydney_datetime_to_utc")]
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

impl DataRowTrait for RooftopPvActualDataRow {}

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
        writeln!(f, "RooftopPvActualParsedData:")?;
        for (info_row, data_rows) in &self.rows {
            // Utilize the Display implementation of RooftopPvActualInformationRow
            writeln!(f, "\t{}", info_row)?;
            for data_row in data_rows {
                // Utilize the Display implementation of RooftopPvActualDataRow
                writeln!(f, "\t\t{}", data_row)?;
            }
        }

        Ok(())
    }
}
