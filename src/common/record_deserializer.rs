use csv::ReaderBuilder;
use serde::de::DeserializeOwned;
use std::error::Error;
use std::fmt;

// Updated DeserializeError to include more context
#[derive(Debug, PartialEq)]
pub enum DeserializeError {
    Csv(String),
    NoRecordFound,
}

impl fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DeserializeError::Csv(err) => write!(f, "CSV parsing error: {}", err),
            DeserializeError::NoRecordFound => write!(f, "No record found"),
        }
    }
}

impl Error for DeserializeError {}

impl From<csv::Error> for DeserializeError {
    fn from(err: csv::Error) -> Self {
        DeserializeError::Csv(format!("{}", err))
    }
}

/// Generic deserializer for a line into a RecordType.
///
/// This function takes a string slice as input and attempts to deserialize it
/// into a specified type `T` that implements `DeserializeOwned`. It is designed
/// to be generic to work with any record type that can be deserialized from a string.
///
/// # Arguments
///
/// * `line` - A string slice representing the line to be deserialized.
///
/// # Returns
///
/// This function returns a `Result` which is either:
/// - Ok(T): A successfully deserialized record of type `T`.
/// - Err(Box<dyn Error>): An error encountered during deserialization.
///
// Example usage of `deserialize_record` function:
// let line = "some, csv, line";
// let record: Result<MyRecordType, _> = deserialize_record(line);
// match record {
//     Ok(record) => println!("Successfully deserialized record"),
//     Err(e) => println!("Error deserializing record: {}", e),
// }
pub fn deserialize_record<T: DeserializeOwned>(line: &str) -> Result<T, DeserializeError> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(line.as_bytes());
    rdr.deserialize::<T>()
        .next()
        .ok_or(DeserializeError::NoRecordFound)?
        .map_err(|e| DeserializeError::Csv(format!("Error deserializing line '{}': {}", line, e)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq)]
    struct TestRecord {
        name: String,
        age: u8,
    }

    #[test]
    fn test_deserialize_record_success() {
        let line = "John Doe,30";
        let record: Result<TestRecord, _> = deserialize_record(line);
        assert_eq!(
            record,
            Ok(TestRecord {
                name: "John Doe".to_string(),
                age: 30
            })
        );
    }

    #[test]
    fn test_deserialize_record_csv_error() {
        let line = "John Doe,thirty"; // "thirty" cannot be parsed into u8
        let record: Result<TestRecord, _> = deserialize_record(line);
        assert!(matches!(record, Err(DeserializeError::Csv(_))));
    }

    #[test]
    fn test_deserialize_record_no_record_found() {
        let line = ""; // Empty line
        let record: Result<TestRecord, _> = deserialize_record(line);
        assert_eq!(record, Err(DeserializeError::NoRecordFound));
    }
}
