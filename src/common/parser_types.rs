use std::error::Error;
use std::fs::{File, metadata};
use std::time::Instant;
use std::io::Read;
use std::path::Path;
use std::fmt;
use zip::ZipArchive;

pub trait ProcessRecord<T> {
    fn process(line: &str) -> Result<T, Box<dyn Error>>;
}

/// A generic collection of records with potential for additional metadata.
#[derive(Debug)]
pub struct RecordsCollection<T> {
    pub records: Vec<T>,
    pub source_file: Option<String>,
    pub processing_time_ms: Option<u128>,
    pub zipfile_size_bytes: Option<u64>, // New field for zipfile size in bytes
    pub number_of_files: Option<usize>, // New field for the number of files in the zipfile
}

impl<T> RecordsCollection<T> {
    /// Creates a new, empty `RecordsCollection`.
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
    pub fn add_records(&mut self, new_records: Vec<T>) {
        self.records.extend(new_records);
    }

    /// Sets the source file metadata.
    pub fn set_source_file(&mut self, file_name: String) {
        self.source_file = Some(file_name);
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

impl<T: fmt::Display> fmt::Display for RecordsCollection<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Records Collection:")?;
        writeln!(f, "Source File: {:?}", self.source_file.as_ref().unwrap_or(&"None".to_string()))?;
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

pub fn unzip_and_process<P, F, T>(file_path: P, processor: F) -> Result<RecordsCollection<T>, Box<dyn Error>>
where
    P: AsRef<Path>,
    F: Fn(&str) -> Result<Vec<T>, Box<dyn Error>>,
    T: 'static + std::fmt::Display,
{
    let start_time = Instant::now();
    let file = File::open(file_path.as_ref())?;
    let metadata = metadata(file_path.as_ref())?;
    let zipfile_size = metadata.len();
    let mut archive = ZipArchive::new(file)?;
    let number_of_files = archive.len();

    let mut collection = RecordsCollection::<T>::new();
    collection.set_source_file(file_path.as_ref().to_string_lossy().into_owned());
    collection.set_zipfile_size(zipfile_size);
    collection.set_number_of_files(number_of_files);

    for i in 0..number_of_files {
        let mut file = archive.by_index(i)?;
        let file_name = file.name().to_string();

        println!("Processing file: {}", file_name);

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        match processor(&contents) {
            Ok(result) => {
                println!("Successfully processed {}", file_name);
                collection.add_records(result);
            },
            Err(e) => println!("Error processing {}: {}", file_name, e),
        }
    }

    let processing_time = start_time.elapsed().as_millis();
    collection.set_processing_time(processing_time);

    Ok(collection)
}