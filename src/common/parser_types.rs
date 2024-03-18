use serde::Deserialize;
use std::fmt;

// Trait for InformationRow
pub trait InformationRowTrait: for<'de> Deserialize<'de> + fmt::Debug + fmt::Display {}

// Trait for DataRow
pub trait DataRowTrait: for<'de> Deserialize<'de> + fmt::Debug + fmt::Display {}

pub trait RowMatcher {
    fn match_row(&self, record: &csv::StringRecord) -> RowAction;
}

pub enum RowAction {
    InformationRow,
    DataRow,
    ControlRow,
    Ignore, // Use this to ignore rows that don't match any criteria
}
