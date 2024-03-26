use chrono::{NaiveDate, NaiveDateTime};
use std::error::Error;

mod common;
mod http_requests;
mod models;
mod utils;

use crate::common::parser_types::{unzip_and_process, unzip_and_process_from_url};
use crate::http_requests::nemweb::fetch_html_content;
use crate::models::{
    nem_current_rooftop_pv_actual::process_file_current_rooftop_actual,
    nem_current_tradingis_report::process_file_current_trading_is,
};
use crate::utils::html_parse::ZipLinkExtractor;
use crate::utils::time_ranges::{Interval, TimestampGenerator};
use crate::utils::url_parse::ParsedReport;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let path = "src/fixtures/PUBLIC_TRADINGIS_202403031335_0000000412683134.zip";
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
    let result = unzip_and_process(path, |contents: &str| {
        process_file_current_rooftop_actual(contents.as_bytes())
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

    let path = "src/fixtures/PUBLIC_DVD_ROOFTOP_PV_ACTUAL_201912010000.zip";
    let result = unzip_and_process(path, |contents: &str| {
        process_file_current_rooftop_actual(contents.as_bytes())
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
        .and_hms_opt(7, 50, 0)
        .expect("Start date hours minutes seconds not valid");
    let end_date: NaiveDateTime = NaiveDate::from_ymd_opt(2024, 3, 8)
        .expect("End date is invalid")
        .and_hms_opt(10, 31, 0)
        .expect("End date hours minutes seconds not valid");

    let generator = TimestampGenerator::new(start_date, end_date, Interval::FiveMinutes);

    let timestamps = generator.generate();
    for timestamp in timestamps {
        println!("{}", timestamp);
    }

    let sample_href =
        "/Reports/Current/TradingIS_Reports/PUBLIC_TRADINGIS_202403120535_0000000413460134.zip";
    let result = ParsedReport::parse_report_path(sample_href).expect("Failed to parse report path");
    println!("{}", result);

    let path = "/Reports/Current/ROOFTOP_PV/ACTUAL/";
    let user_agent = "rooftop-app/0.1";
    let url_paths = match fetch_html_content(path, user_agent).await {
        Ok(html_content) => {
            let extractor = ZipLinkExtractor::new();
            extractor.extract_links(&html_content)
        }
        Err(e) => {
            println!("Error fetching HTML content: {:?}", e);
            Vec::new()
        }
    };
    for url_path in url_paths {
        let result = unzip_and_process_from_url(&url_path, |contents: &str| {
            process_file_current_rooftop_actual(contents.as_bytes())
        })
        .await?;
        println!("{}", result);
    }

    Ok(())
}
