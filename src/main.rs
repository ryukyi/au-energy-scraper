use chrono::{NaiveDate, NaiveDateTime};
use std::error::Error;

mod common;
mod models;
mod utils;

use crate::common::parser_types::unzip_and_process;
use crate::models::{
    nem_current_rooftop_pv_actual::process_file_current_rooftop_actual,
    nem_current_tradingis_report::process_file_current_trading_is,
};
use crate::utils::time_ranges::{Interval, TimestampGenerator};

fn main() -> Result<(), Box<dyn Error>> {
    let path = "src/fixtures/PUBLIC_TRADINGIS_202403031335_0000000412683134.zip";
    // Ensure unzip_and_process correctly handles the Result type and returns a Result<Vec<Record>, Error>
    // Adjust the processor function to match the expected signature
    let result = unzip_and_process(path, |contents: &str| {
        process_file_current_trading_is(contents.to_string())
    });
    match result {
        Ok(records) => {
            print!("{}", &records)
        }
        Err(e) => {
            println!("Error processing file: {:?}", e);
            return Err(e);
        }
    }

    let path =
        "src/fixtures/PUBLIC_ROOFTOP_PV_ACTUAL_MEASUREMENT_20240303200000_0000000412707330.zip";
    // Ensure unzip_and_process correctly handles the Result type and returns a Result<Vec<Record>, Error>
    // Adjust the processor function to match the expected signature
    let result = unzip_and_process(path, |contents: &str| {
        process_file_current_rooftop_actual(contents.to_string())
    });
    match result {
        Ok(records) => {
            print!("{}", &records)
        }
        Err(e) => {
            println!("Error processing file: {:?}", e);
            return Err(e);
        }
    }

    let start_date: NaiveDateTime = NaiveDate::from_ymd_opt(2024, 3, 8)
        .expect("Start date is invalid")
        .and_hms_opt(7, 50, 0).expect("Start date hours minutes seconds not valid");
    let end_date: NaiveDateTime = NaiveDate::from_ymd_opt(2024, 3, 8)
        .expect("End date is invalid")
        .and_hms_opt(10, 31, 0).expect("End date hours minutes seconds not valid");

    // let start_date = NaiveDateTime::parse_from_str("20240309133000", "%Y%m%d%H%M%S").unwrap();
    // let end_date = NaiveDateTime::parse_from_str("20240309150000", "%Y%m%d%H%M%S").unwrap();
    let generator = TimestampGenerator::new(start_date, end_date, Interval::FiveMinutes);

    let timestamps = generator.generate();
    for timestamp in timestamps {
        println!("{}", timestamp);
    }

    Ok(())
}
