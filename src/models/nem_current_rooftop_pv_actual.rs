use chrono::{DateTime, Utc};
use serde::{self, Deserialize};

const FORMAT: &str = "%Y/%m/%d %H:%M:%S";

enum RowType {
    Control,
    Information,
    Data,
}

// Information will serve as headers
#[derive(Debug, Deserialize)]
struct InformationRow {
    // first 4 columns not needed
    interval: u32,
    interval_datetime: DateTime<Utc>,
    regionid: String,
    power: f64,
    qi: u32,
    type_: String,
    lastchanged: DateTime<Utc>,
}

// Struct to represent the Data rows
#[derive(Debug, Deserialize)]
struct DataRow {
    interval: u32,
    interval_datetime: DateTime<Utc>,
    regionid: String,
    power: f64,
    qi: u32,
    type_: String,
    lastchanged: DateTime<Utc>,
}

struct ParsedData {
    headers: Vec<InformationRow>,
    data: Vec<DataRow>,
}
