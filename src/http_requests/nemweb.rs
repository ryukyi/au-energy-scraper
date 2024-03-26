use bytes::Bytes;
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use reqwest::Error as ReqwestError;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};
use std::time::Duration;

// Assuming you have a custom error type in your application
#[derive(Debug)]
pub enum HttpError {
    Reqwest(ReqwestError),
    Io(IoError),
    MimeType(String),
}

impl From<ReqwestError> for HttpError {
    fn from(err: ReqwestError) -> HttpError {
        HttpError::Reqwest(err)
    }
}

impl From<IoError> for HttpError {
    fn from(err: IoError) -> HttpError {
        HttpError::Io(err)
    }
}

// Define the base URL
const BASE_URL: &str = "http://nemweb.com.au";

// Define an asynchronous function to fetch HTML content with additional parameters
pub async fn fetch_html_content(path: &str, user_agent: &str) -> Result<String, ReqwestError> {
    // Combine the base URL with the provided path
    let url = format!("{}{}", BASE_URL, path);

    // Create a client with a custom user agent and timeout
    let client = Client::builder()
        .user_agent(user_agent)
        .timeout(Duration::from_secs(10))
        .build()?;

    // Perform the GET request
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let html_content = response.text().await?;
        Ok(html_content)
    } else {
        Err(response.error_for_status().unwrap_err())
    }
}

pub async fn fetch_zip_file(path: &str, user_agent: &str) -> Result<Bytes, HttpError> {
    let url = format!("{}{}", BASE_URL, path);

    let client = Client::builder()
        .user_agent(user_agent)
        .timeout(Duration::from_secs(10))
        .build()?;

    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let content_type = response
            .headers()
            .get(CONTENT_TYPE)
            .ok_or_else(|| IoError::new(IoErrorKind::NotFound, "Content-Type header not found"))?
            .to_str()
            .map_err(|_| IoError::new(IoErrorKind::InvalidData, "Invalid Content-Type header"))?;

        if content_type.starts_with("application/zip")
            | content_type.starts_with("application/x-zip-compressed")
        {
            let bytes = response.bytes().await?;
            // Log the size of the downloaded file for debugging purposes
            println!("Downloaded ZIP file size: {} bytes", bytes.len());
            Ok(bytes)
        } else {
            // Log unexpected content type for debugging purposes
            println!("Unexpected Content-Type: {}", content_type);
            Err(HttpError::MimeType("The file is not a zip".into()))
        }
    } else {
        Err(HttpError::from(response.error_for_status().unwrap_err()))
    }
}
