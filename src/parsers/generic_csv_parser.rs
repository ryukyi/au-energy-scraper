use csv::{ReaderBuilder, Trim};
use serde::Deserialize;
use std::error::Error;
use std::fmt;
use std::io::Read;

// Trait for InformationRow
pub trait InformationRowTrait: for<'de> Deserialize<'de> + fmt::Debug + fmt::Display {}

// Trait for DataRow
pub trait DataRowTrait: for<'de> Deserialize<'de> + fmt::Debug + fmt::Display {}

pub trait RowMatcher {
    fn match_row(&self, record: &csv::StringRecord) -> RowAction;
}

struct MyCustomMatcher;

impl RowMatcher for MyCustomMatcher {
    fn match_row(&self, record: &csv::StringRecord) -> RowAction {
        if record.get(0) == Some("I") && record.get(2) == Some("ACTUAL") {
            RowAction::InformationRow
        } else if record.get(0) == Some("D") {
            RowAction::DataRow
        } else if record.get(0) == Some("C") {
            RowAction::ControlRow
        } else {
            RowAction::Ignore
        }
    }
}

pub enum RowAction {
    InformationRow,
    DataRow,
    ControlRow,
    Ignore, // Use this to ignore rows that don't match any criteria
}

pub trait ParsedData {
    type InformationRow: InformationRowTrait;
    type DataRow: DataRowTrait;
    type Matcher: RowMatcher;

    fn new() -> Self;
    fn add_rows(&mut self, rows: Vec<(Self::InformationRow, Vec<Self::DataRow>)>);
    fn matcher(&self) -> &Self::Matcher;
}

pub fn parse_csv<R: Read, T: ParsedData>(reader: R) -> Result<T, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .trim(Trim::All)
        .has_headers(false)
        .flexible(true) // Allow rows with varying numbers of fields
        .from_reader(reader);

    let mut parsed_data = T::new();
    let mut rows: Vec<(<T as ParsedData>::InformationRow, Vec<<T as ParsedData>::DataRow>)> = Vec::new();
    let mut current_data_rows = Vec::new();
    let mut current_info_row: Option<T::InformationRow> = None;

    for result in rdr.records() {
        let record = result?;
        let action = parsed_data.matcher().match_row(&record);

        match action {
            RowAction::InformationRow => {
                // Process as information row
            },
            RowAction::DataRow => {
                // Process as data row
            },
            RowAction::ControlRow => {
                // Process as control row
            },
            RowAction::Ignore => {
                // Ignore this row
            },
        }
    }

    parsed_data.add_rows(rows);

    Ok(parsed_data)
}
