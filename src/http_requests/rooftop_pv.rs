// Import necessary modules
use reqwest::Client;
use reqwest::Error as ReqwestError;
use std::time::Duration;

// Define the base URL
const BASE_URL: &str = "http://nemweb.com.au";

// Define an asynchronous function to fetch HTML content with additional parameters
pub async fn fetch_html_content_with_options(
    path: &str,
    user_agent: &str,
) -> Result<String, ReqwestError> {
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
        Err(ReqwestError::from(response.error_for_status().unwrap_err()))
    }
}
