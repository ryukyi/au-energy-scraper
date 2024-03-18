use csv::{ReaderBuilder, Trim};
use serde::Deserialize;
use std::error::Error;
use std::fmt;
use std::io::Read;

#[derive(Debug, Deserialize)]
enum RowType {
    #[serde(rename = "C")]
    Control,
    #[serde(rename = "I")]
    Information,
    #[serde(rename = "D")]
    Data,
}

#[derive(PartialEq, Eq)]
enum ParseState {
    Control,
    Information,
    Data,
}

pub trait ParsedData
where
    Self::InformationRow: for<'de> Deserialize<'de> + fmt::Debug + fmt::Display,
    Self::DataRow: for<'de> Deserialize<'de> + fmt::Debug + fmt::Display,
{
    type InformationRow;
    type DataRow;

    fn new() -> Self;
    fn add_information_row(&mut self, row: Self::InformationRow);
    fn add_data_row(&mut self, row: Self::DataRow);
}

pub fn parse_csv<R: Read, T: ParsedData>(reader: R) -> Result<T, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .trim(Trim::All)
        .has_headers(false)
        .flexible(true) // Allow rows with varying numbers of fields
        .from_reader(reader);

    let mut parsed_data = T::new();
    let mut state = ParseState::Control;
    for result in rdr.records() {
        let record = result?;
        let first_cell = record.get(0).unwrap_or("");

        match first_cell {
            "I" => {
                state = ParseState::Information;
                let information_row: T::InformationRow = record.deserialize(None)?;
                parsed_data.add_information_row(information_row);
            }
            "D" => {
                state = ParseState::Data;
                let data_row: T::DataRow = record.deserialize(None)?;
                parsed_data.add_data_row(data_row);
                // No need to change state, directly handling based on row type
            }
            "C" => {
                if state == ParseState::Data
                    && record.get(1).unwrap_or("").to_uppercase() == "END OF REPORT"
                {
                    break;
                }
                continue;
            }
            _ => {
                // Handle unexpected row types
                return Err(From::from(format!(
                    "Unexpected row type or transition: {}",
                    first_cell
                )));
            }
        }
    }
    Ok(parsed_data)
}
