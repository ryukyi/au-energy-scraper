use std::fs::File;
use zip::read::ZipArchive;

mod models;
mod parsers;

use crate::parsers::rooftop_pv_parser::parse_csv_no_dependencies;

fn main() {
    // Open the ZIP file
    let file = File::open(
        "./src/fixtures/PUBLIC_ROOFTOP_PV_ACTUAL_MEASUREMENT_20240303200000_0000000412707330.zip",
    )
    .expect("Failed to open ZIP file");
    let mut archive = ZipArchive::new(file).expect("Failed to read ZipArchive");

    // Check if the archive contains exactly one file
    if archive.len() == 1 {
        // Since there's only one file, we can directly access it without iterating
        let file = archive.by_index(0).expect("Failed to get file by index");
        // Create a BufReader for the file
        let _ = parse_csv_no_dependencies(file);
    } else {
        // Handle archives with more than one file or no files
        todo!("Handle archives with multiple files or no files");
    }
}
