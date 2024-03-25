use chrono::{DateTime, Utc};
use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;
use std::fmt;
use std::io::{BufRead, BufReader};

use crate::common::parser_types::ProcessRecord;
use crate::utils::datetimezone_conversion::deserialize_sydney_datetime_to_utc;

// Updated struct to represent the Data row (D row)
#[derive(Debug, Deserialize)]
pub struct RooftopPvActualData {
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
    interval_datetime: DateTime<Utc>,
    #[serde(rename = "REGIONID")]
    regionid: String,
    #[serde(rename = "POWER")]
    power: Option<f64>, // Assuming power can be a floating-point number
    #[serde(rename = "QI")]
    qi: Option<f64>,
    #[serde(rename = "TYPE")]
    type_: String,
    #[serde(rename = "LASTCHANGED")]
    #[serde(deserialize_with = "deserialize_sydney_datetime_to_utc")]
    lastchanged: DateTime<Utc>,
}

impl fmt::Display for RooftopPvActualData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RooftopPvActualData: {{ csv_row_identifier: {:?}, category: {:?}, report_type: {:?}, report_type_int: {:?}, interval_datetime: {:?}, regionid: {:?}, power: {:?}, qi: {:?}, type_: {:?}, lastchanged: {:?} }}",
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

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum RecordCurrentRooftopPvActual {
    Variant1(RooftopPvActualData),
    // potentially more variants later
}

impl fmt::Display for RecordCurrentRooftopPvActual {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RecordCurrentRooftopPvActual::Variant1(interconnector_data) => {
                write!(f, "{}", interconnector_data)
            }
        }
    }
}

impl ProcessRecord<RecordCurrentRooftopPvActual> for RooftopPvActualData {
    fn process(line: &str) -> Result<RecordCurrentRooftopPvActual, Box<dyn Error>> {
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(line.as_bytes());
        let record = rdr
            .deserialize::<RooftopPvActualData>()
            .next()
            .ok_or("Error attempting to deserialize InterconnectorData")??;
        Ok(RecordCurrentRooftopPvActual::Variant1(record))
    }
}

pub fn process_file_current_rooftop_actual(
    contents: String,
) -> Result<Vec<RecordCurrentRooftopPvActual>, Box<dyn Error>> {
    // Convert the contents string to byte data
    let byte_data = contents.as_bytes();
    let reader = BufReader::new(byte_data);
    let mut records: Vec<RecordCurrentRooftopPvActual> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        if line.starts_with("I,ROOFTOP,ACTUAL") {
            println!("{:?}", &line);
        }
        match line.chars().next() {
            Some('C') | Some('I') => continue,
            Some('D') => {
                if line.starts_with("D,ROOFTOP,ACTUAL") {
                    // Assuming RooftopPvActualData::process is similar to InterconnectorData::process and PriceData::process
                    records.push(RooftopPvActualData::process(&line)?);
                } else {
                    println!("Parser Error: RooftopPvActualData\n line: {:?}", &line);
                    return Err("Unknown record type for RooftopPvActualData".into());
                }
            }
            _ => return Err("Invalid line format".into()),
        }
    }

    Ok(records)
}
