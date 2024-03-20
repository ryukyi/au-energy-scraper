use std::error::Error;

mod common;
mod models;
mod time_utils;

use crate::models::{
    nem_current_tradingis_report::process_file_current_trading_is,
    nem_current_rooftop_pv_actual::process_file_current_rooftop_actual};
use crate::common::parser_types::unzip_and_process;

fn main() -> Result<(), Box<dyn Error>> {
    let path = "src/fixtures/PUBLIC_TRADINGIS_202403031335_0000000412683134.zip";
    // Ensure unzip_and_process correctly handles the Result type and returns a Result<Vec<Record>, Error>
    // Adjust the processor function to match the expected signature
    let result = unzip_and_process(path, |contents: &str| process_file_current_trading_is(contents.to_string()));
    match result {
        Ok(records) => {
            print!("{}", &records)
        },
        Err(e) => {
            println!("Error processing file: {:?}", e);
            return Err(e);
        }
    }

    let path = "src/fixtures/PUBLIC_ROOFTOP_PV_ACTUAL_MEASUREMENT_20240303200000_0000000412707330.zip";
    // Ensure unzip_and_process correctly handles the Result type and returns a Result<Vec<Record>, Error>
    // Adjust the processor function to match the expected signature
    let result = unzip_and_process(path, |contents: &str| process_file_current_rooftop_actual(contents.to_string()));
    match result {
        Ok(records) => {
            print!("{}", &records)
        },
        Err(e) => {
            println!("Error processing file: {:?}", e);
            return Err(e);
        }
    }

    Ok(())
}