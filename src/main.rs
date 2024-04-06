use std::error::Error;
use std::fs;

mod common;
mod http_requests;
mod models;
mod parsers;
mod time;

use crate::common::processor::{unzip_and_process, unzip_and_process_from_url};
use crate::http_requests::html::fetch_html_content;
use crate::parsers::html::ZipLinkExtractorFromHtml;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Example unzipping a filepath
    let file_path = "src/fixtures/PUBLIC_ROOFTOP_PV_FORECAST_20240321203000_0000000414322812.zip";
    let zip_bytes = fs::read(file_path)?;
    let collection = unzip_and_process(&zip_bytes)?;
    println!("{}", collection);

    // Example requesting from url and iterating over, parsing each zip file
    let base_url = "http://nemweb.com.au";
    let path = "/Reports/Current/ROOFTOP_PV/FORECAST/";
    let user_agent = "rooftop-app/0.1";
    let url_paths = match fetch_html_content(base_url, path, user_agent).await {
        Ok(html_content) => {
            let extractor = ZipLinkExtractorFromHtml::new();
            extractor.extract_links(&html_content)
        }
        Err(e) => {
            println!("Error fetching HTML content: {:?}", e);
            Vec::new()
        }
    };
    for path in url_paths {
        let url_collection = unzip_and_process_from_url(base_url, &path).await?;
        println!("{:?}", url_collection);
    }

    Ok(())
}
