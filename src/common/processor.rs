use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::io::Cursor;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use reqwest;
use serde::Deserialize;
use zip::ZipArchive;

use crate::common::record_deserializer::{deserialize_record, DeserializeError};
use crate::models::{
    nem_current_rooftop_pv::{RooftopPvActual, RooftopPvForecast},
    nem_current_tradingis_reports::{Interconnector, Price},
};

pub trait RecordTypeStartsWith {
    fn matches(line: &str) -> bool;
}

// Define the enum for record types
#[derive(Debug, Deserialize)]
pub enum RecordType {
    Interconnector(Interconnector),
    Price(Box<Price>),
    RooftopPvActual(RooftopPvActual),
    RooftopPvForecast(RooftopPvForecast),
}

// Define the State enum based on the first 4 values of "I" rows
#[derive(Debug, PartialEq, Eq, Hash)]
enum State {
    Interconnector,
    Price,
    RooftopPvActual,
    RooftopPvForecast,
}

// Function to initialize and return the HashMap
fn initialize_deserializers() -> HashMap<State, DeserializerFn> {
    let mut deserializers: HashMap<State, DeserializerFn> = HashMap::new();

    deserializers.insert(State::Interconnector, |line| {
        deserialize_record::<Interconnector>(line).map(RecordType::Interconnector)
    });
    deserializers.insert(State::Price, |line| {
        deserialize_record::<Price>(line).map(|price| RecordType::Price(Box::new(price)))
    });
    deserializers.insert(State::RooftopPvActual, |line| {
        deserialize_record::<RooftopPvActual>(line).map(RecordType::RooftopPvActual)
    });
    deserializers.insert(State::RooftopPvForecast, |line| {
        deserialize_record::<RooftopPvForecast>(line).map(RecordType::RooftopPvForecast)
    });

    deserializers
}

// Define a type alias for a deserialization function
type DeserializerFn = fn(&str) -> Result<RecordType, DeserializeError>;

impl fmt::Display for RecordType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RecordType::Interconnector(data) => write!(f, "{}", data),
            RecordType::Price(data) => write!(f, "{}", data),
            RecordType::RooftopPvActual(data) => write!(f, "{}", data),
            RecordType::RooftopPvForecast(data) => write!(f, "{}", data),
        }
    }
}

fn extract_identifier(line: &str) -> &str {
    let mut commas = 0;
    let mut end = 0;

    for (index, char) in line.char_indices() {
        if char == ',' {
            commas += 1;
            if commas == 4 {
                end = index;
                break;
            }
        }
    }

    // If less than 4 commas are found, return the whole line
    // otherwise, return up to the 4th comma
    if commas < 4 {
        line
    } else {
        &line[..end]
    }
}

// Accepts an iterator so file is only read once
fn process<I>(lines: I) -> Result<Vec<RecordType>, Box<dyn Error>>
where
    I: Iterator<Item = Result<String, std::io::Error>>,
{
    let deserializers = initialize_deserializers();
    let mut current_state: Option<State> = None;
    let mut records = Vec::new();

    for line_result in lines {
        let line = line_result?; // Handle the Result here
        if line.starts_with('I') {
            let identifier = extract_identifier(&line);
            current_state = match identifier {
                "I,TRADING,INTERCONNECTORRES,2" => Some(State::Interconnector),
                "I,TRADING,PRICE,3" => Some(State::Price),
                "I,ROOFTOP,ACTUAL,2" => Some(State::RooftopPvActual),
                "I,ROOFTOP,FORECAST,1" => Some(State::RooftopPvForecast),
                _ => None,
            };
        } else if line.starts_with('D') && current_state.is_some() {
            if let Some(ref state) = current_state {
                if let Some(deserializer) = deserializers.get(state) {
                    let record = deserializer(&line)?;
                    records.push(record);
                }
            }
        }
    }

    Ok(records)
}

pub fn unzip_and_process(zip_bytes: &[u8]) -> Result<CsvRecordCollection, Box<dyn Error>> {
    let start_time = Instant::now();
    let reader = Cursor::new(zip_bytes);
    let mut archive = ZipArchive::new(reader)?;

    let number_of_files = archive.len();
    let mut collection = CsvRecordCollection::new();
    collection.set_zipfile_size(zip_bytes.len() as u64);
    collection.set_number_of_files(number_of_files);

    for i in 0..number_of_files {
        let file = archive.by_index(i)?;
        let file_name = file.name().to_string();

        println!("Processing file: {}", file_name);

        let file_reader = BufReader::new(file);
        let lines_iter = file_reader.lines();

        // Pass the iterator directly to process
        let processed_records = process(lines_iter)?;
        collection.add_records(processed_records);
    }

    let processing_time = start_time.elapsed().as_millis();
    collection.set_processing_time(processing_time);

    Ok(collection)
}

/// A generic collection of records with metadata.
#[derive(Debug)]
pub struct CsvRecordCollection {
    pub records: Vec<RecordType>,
    pub source_file: Option<String>,
    pub processing_time_ms: Option<u128>,
    pub zipfile_size_bytes: Option<u64>,
    pub number_of_files: Option<usize>,
}

impl CsvRecordCollection {
    pub fn new() -> Self {
        CsvRecordCollection {
            records: Vec::new(),
            source_file: None,
            processing_time_ms: None,
            zipfile_size_bytes: None,
            number_of_files: None,
        }
    }

    /// Extends the collection with a vector of records.
    pub fn add_records(&mut self, new_records: Vec<RecordType>) {
        self.records.extend(new_records);
    }

    /// Sets the processing time metadata.
    pub fn set_processing_time(&mut self, time_ms: u128) {
        self.processing_time_ms = Some(time_ms);
    }

    /// Sets the zipfile size metadata.
    pub fn set_zipfile_size(&mut self, size_bytes: u64) {
        self.zipfile_size_bytes = Some(size_bytes);
    }

    /// Sets the number of files metadata.
    pub fn set_number_of_files(&mut self, count: usize) {
        self.number_of_files = Some(count);
    }
}

impl fmt::Display for CsvRecordCollection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Records Collection:")?;
        writeln!(
            f,
            "Source File: {:?}",
            self.source_file.as_ref().unwrap_or(&"None".to_string())
        )?;
        writeln!(f, "Processing Time (ms): {:?}", self.processing_time_ms)?;
        writeln!(f, "Zipfile Size (bytes): {:?}", self.zipfile_size_bytes)?;
        writeln!(f, "Number of Files: {:?}", self.number_of_files)?;
        writeln!(f, "Records:")?;
        for record in &self.records {
            writeln!(f, "{}", record)?;
        }

        Ok(())
    }
}

// Assuming RecordsCollection and unzip_and_process are defined elsewhere in your code.

/// Fetches a zip file from a URL and processes its contents.
///
/// # Arguments
///
/// * `base_url` - The base URL to fetch the zip file from.
/// * `path` - The specific path to the zip file on the server.
///
/// # Returns
///
/// A result containing a `RecordsCollection` if successful, or an error if not.
pub async fn unzip_and_process_from_url(
    base_url: &str,
    path: &str,
) -> Result<CsvRecordCollection, Box<dyn Error>> {
    let url = format!("{}{}", base_url, path);

    // Fetch the zip file from the URL
    let response = reqwest::get(&url).await?;
    let bytes = response.bytes().await?.to_vec(); // Convert Bytes to Vec<u8>

    // Directly pass the bytes to the adjusted unzip_and_process function
    let result = unzip_and_process(&bytes)?;

    Ok(result)
}
