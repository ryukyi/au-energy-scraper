use csv::{ReaderBuilder, Trim};
use std::error::Error;
use std::io::Read;

use crate::common::parser_types::{DataRowTrait, InformationRowTrait, RowAction, RowMatcher};

pub trait ParsedData {
    type InformationRow: InformationRowTrait;
    type DataRow: DataRowTrait;
    type Matcher: RowMatcher;

    fn new() -> Self;
    fn add_rows(&mut self, rows: Vec<(Self::InformationRow, Vec<Self::DataRow>)>);
    fn matcher(&self) -> &Self::Matcher;
}

// Adjust the function to take a vector of tuples, each containing a reader and a mutable reference to its associated ParsedData
pub fn parse_csv<R: Read, T: ParsedData>(readers: Vec<(R, &mut T)>) -> Result<(), Box<dyn Error>> {
    for (reader, parsed_data) in readers {
        let mut rdr = ReaderBuilder::new()
            .trim(Trim::All)
            .has_headers(false)
            .flexible(true) // Allow rows with varying numbers of fields
            .from_reader(reader);

        let mut rows: Vec<(T::InformationRow, Vec<T::DataRow>)> = Vec::new();
        let mut current_data_rows: Vec<T::DataRow> = Vec::new();
        let mut current_info_row: Option<T::InformationRow> = None;

        for result in rdr.records() {
            let record = result?;
            let action = parsed_data.matcher().match_row(&record);

            match action {
                RowAction::InformationRow => {
                    if let Ok(info_row) = record.deserialize::<T::InformationRow>(None) {
                        if let Some(current_info) = current_info_row.take() {
                            rows.push((current_info, std::mem::take(&mut current_data_rows)));
                        }
                        current_info_row = Some(info_row);
                    } else {
                        // Handle deserialization error
                    }
                }
                RowAction::DataRow => {
                    if let Ok(data_row) = record.deserialize::<T::DataRow>(None) {
                        current_data_rows.push(data_row);
                    } else {
                        // Handle deserialization error
                    }
                }
                RowAction::ControlRow => {
                    if record.get(1).unwrap_or("").to_uppercase() == "END OF REPORT" {
                        if let Some(current_info) = current_info_row.take() {
                            rows.push((current_info, std::mem::take(&mut current_data_rows)));
                        }
                        break;
                    }
                }
                RowAction::Ignore => {
                    // Ignore this row
                }
            }
        }

        parsed_data.add_rows(rows);
    }

    Ok(())
}
