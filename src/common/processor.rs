use std::error::Error;
use std::fmt;
use std::io::Cursor;
use std::io::Read;
use std::time::Instant;

use serde::Deserialize;
use reqwest;
use zip::ZipArchive;

use crate::models::{
    nem_current_rooftop_pv::RooftopPvActual,
    nem_current_tradingis_reports::{Price, Interconnector},
};
use crate::common::record_deserializer::deserialize_record;

pub trait RecordTypeStartsWith {
    fn matches(line: &str) -> bool;
}

// Define the enum for record types
#[derive(Debug, Deserialize)]
pub enum RecordType {
    Interconnector(Interconnector),
    Price(Price),
    RooftopPvActual(RooftopPvActual)
}

impl fmt::Display for RecordType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RecordType::Interconnector(data) => write!(f, "{}", data),
            RecordType::Price(data) => write!(f, "{}", data),
            RecordType::RooftopPvActual(data) => write!(f, "{}", data),
        }
    }
}

pub trait ProcessRecord {
    fn process(line: &str) -> Result<RecordType, Box<dyn Error>>;
}

// All zip file types must be listed here
impl ProcessRecord for RecordType {
    fn process(line: &str) -> Result<RecordType, Box<dyn Error>> {
        if Interconnector::matches(line) {
            let record = deserialize_record::<Interconnector>(line)?;
            Ok(RecordType::Interconnector(record))
        } else if Price::matches(line) {
            let record = deserialize_record::<Price>(line)?;
            Ok(RecordType::Price(record))
        } else if RooftopPvActual::matches(line) {
            let record = deserialize_record::<RooftopPvActual>(line)?;
            Ok(RecordType::RooftopPvActual(record))
        } else {
            // Add more cases as needed
            Err("Unknown record type".into())
        }
    }
}

/// A generic collection of records with metadata.
#[derive(Debug)]
pub struct RecordsCollection {
    pub records: Vec<RecordType>,
    pub source_file: Option<String>,
    pub processing_time_ms: Option<u128>,
    pub zipfile_size_bytes: Option<u64>,
    pub number_of_files: Option<usize>,
}

impl RecordsCollection {
    pub fn new() -> Self {
        RecordsCollection {
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

impl fmt::Display for RecordsCollection {
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

// Updated unzip_and_process function signature
// Adjusted unzip_and_process function to accept bytes
pub fn unzip_and_process<F>(
    zip_bytes: &[u8],
    processor: F,
) -> Result<RecordsCollection, Box<dyn Error>>
where
    F: Fn(&str) -> Result<Vec<RecordType>, Box<dyn Error>>,
{
    let start_time = Instant::now();
    let reader = Cursor::new(zip_bytes);
    let mut archive = ZipArchive::new(reader)?;

    let number_of_files = archive.len();
    let mut collection = RecordsCollection::new();
    // Since we don't have a file path, adjust how source_file is set if needed
    collection.set_zipfile_size(zip_bytes.len() as u64);
    collection.set_number_of_files(number_of_files);

    for i in 0..number_of_files {
        let mut file = archive.by_index(i)?;
        let file_name = file.name().to_string();

        println!("Processing file: {}", file_name);

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        match processor(&contents) {
            Ok(result) => {
                collection.add_records(result);
                println!("Successfully processed {}", file_name);
            }
            Err(e) => println!("Error processing {}: {}", file_name, e),
        }
    }

    let processing_time = start_time.elapsed().as_millis();
    collection.set_processing_time(processing_time);

    Ok(collection)
}

// Updated unzip_and_process_from_url function signature
// Updated unzip_and_process_from_url function to use unzip_and_process with bytes
pub async fn unzip_and_process_from_url<F>(
    base_url: &str,
    path: &str,
    processor: F,
) -> Result<RecordsCollection, Box<dyn Error>>
where
    F: Fn(&str) -> Result<Vec<RecordType>, Box<dyn Error>> + Send + Sync + 'static,
{
    let url = format!("{}{}", base_url, path);

    // Fetch the zip file from the URL
    let response = reqwest::get(&url).await?;
    let bytes = response.bytes().await?.to_vec(); // Convert Bytes to Vec<u8>

    // Directly pass the bytes to the adjusted unzip_and_process function
    let result = unzip_and_process(&bytes, processor)?;

    Ok(result)
}