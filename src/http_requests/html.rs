use reqwest::Client;
use reqwest::Error as ReqwestError;
use std::io::Error as IoError;
use std::time::Duration;

// Assuming you have a custom error type in your application
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

// Define an asynchronous function to fetch HTML content with additional parameters
pub async fn fetch_html_content(
    base_url: &str,
    path: &str,
    user_agent: &str,
) -> Result<String, ReqwestError> {
    // Combine the base URL with the provided path
    let url = format!("{}{}", base_url, path);

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

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::StatusCode;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_fetch_html_content_success() {
        let mock_server = MockServer::start().await;
        let mock_response =
            ResponseTemplate::new(StatusCode::OK).set_body_string("<html>Success</html>");
        Mock::given(wiremock::matchers::method("GET"))
            .respond_with(mock_response)
            .mount(&mock_server)
            .await;

        let result = fetch_html_content(&mock_server.uri(), "", "TestAgent").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "<html>Success</html>");
    }
}
