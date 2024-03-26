use chrono::NaiveDateTime;
use std::fmt;
use std::path::Path;
use std::str::FromStr;

// Define a custom error type for parsing errors
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UrlPathBadFormat(String),
    NotZipFileName(String),
    UniqueKeyNotNumber(String),
}

#[derive(Debug, PartialEq)]
pub struct UrlPath(String);

impl FromStr for UrlPath {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(ParseError::UrlPathBadFormat(
                "URL path is empty.".to_string(),
            ))
        } else if !s.starts_with('/') {
            Err(ParseError::UrlPathBadFormat(
                "URL path does not start with '/'. ".to_string() + s,
            ))
        } else if s.ends_with(".zip") {
            Err(ParseError::UrlPathBadFormat(
                "URL path ends with '.zip'. ".to_string() + s,
            ))
        } else {
            Ok(UrlPath(s.to_string()))
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ZipFileName(String);

impl FromStr for ZipFileName {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(ParseError::NotZipFileName(
                "ZipFileName is empty".to_string(),
            ))
        } else if !s.ends_with(".zip") {
            Err(ParseError::NotZipFileName(
                "ZipFileName doesn't end in .zip".to_string(),
            ))
        } else {
            Ok(ZipFileName(s.to_string()))
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct UniqueKey(String);

impl FromStr for UniqueKey {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let _ = s
            .parse::<i64>()
            .map_err(|_| ParseError::UniqueKeyNotNumber("Unique Key is not a number".to_string()));
        Ok(UniqueKey(s.to_string()))
    }
}

#[derive(Debug)]
pub enum ZipReportUrlParseError {
    UrlPathBadFormat(String),
    NotZipFileName(String),
    DateTimeBadFormat,
    UniqueKeyNotNumber(String),
}

impl From<ParseError> for ZipReportUrlParseError {
    fn from(err: ParseError) -> Self {
        match err {
            ParseError::UrlPathBadFormat(msg) => ZipReportUrlParseError::UrlPathBadFormat(msg),
            ParseError::NotZipFileName(msg) => ZipReportUrlParseError::NotZipFileName(msg),
            ParseError::UniqueKeyNotNumber(msg) => ZipReportUrlParseError::UniqueKeyNotNumber(msg),
        }
    }
}

impl From<chrono::ParseError> for ZipReportUrlParseError {
    fn from(_: chrono::ParseError) -> Self {
        ZipReportUrlParseError::DateTimeBadFormat
    }
}

#[derive(Debug)]
pub struct ZipReportUrlPath {
    pub url_path: UrlPath,
    pub file_name: ZipFileName,
    pub datetime: NaiveDateTime,
    pub unique_key: UniqueKey,
}

impl ZipReportUrlPath {
    pub fn parse_report_path(report_path: &str) -> Result<Self, ZipReportUrlParseError> {
        let path = Path::new(report_path);
        let file_name = path
            .file_name()
            .ok_or_else(|| {
                ZipReportUrlParseError::NotZipFileName("Expected a zip file name".to_string())
            })?
            .to_str()
            .ok_or_else(|| {
                ZipReportUrlParseError::NotZipFileName("Expected a zip file name".to_string())
            })?;

        let url_path_end = report_path.rfind(file_name).unwrap_or(0);
        let url_path = &report_path[..url_path_end];

        let without_extension = file_name.trim_end_matches(".zip");
        let parts: Vec<&str> = without_extension.rsplitn(3, '_').collect();
        let unique_key = parts.first().ok_or_else(|| {
            ZipReportUrlParseError::UniqueKeyNotNumber("Unique Key is not a number".to_string())
        })?;
        let datetime_str = parts
            .get(1)
            .ok_or(ZipReportUrlParseError::DateTimeBadFormat)?;

        let datetime = NaiveDateTime::parse_from_str(datetime_str, "%Y%m%d%H%M")?;

        Ok(Self {
            url_path: UrlPath::from_str(url_path)?,
            file_name: ZipFileName::from_str(file_name)?,
            datetime,
            unique_key: UniqueKey::from_str(unique_key)?,
        })
    }
}

impl fmt::Display for ZipReportUrlPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "URL Path: {}, File Name: {}, Datetime: {}, Unique Key: {}",
            self.url_path.0, self.file_name.0, self.datetime, self.unique_key.0
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_report_path() {
        let sample_href =
            "/Reports/Current/TradingIS_Reports/PUBLIC_TRADINGIS_202403120535_0000000413460134.zip";
        let expected = ZipReportUrlPath {
            url_path: UrlPath::from_str("/Reports/Current/TradingIS_Reports/").unwrap(),
            file_name: ZipFileName::from_str("PUBLIC_TRADINGIS_202403120535_0000000413460134.zip")
                .unwrap(),
            datetime: NaiveDateTime::parse_from_str("202403120535", "%Y%m%d%H%M").unwrap(),
            unique_key: UniqueKey::from_str("0000000413460134").unwrap(),
        };

        let result =
            ZipReportUrlPath::parse_report_path(sample_href).expect("Failed to parse report path");

        assert_eq!(result.url_path, expected.url_path);
        assert_eq!(result.file_name, expected.file_name);
        assert_eq!(result.datetime, expected.datetime);
        assert_eq!(result.unique_key, expected.unique_key);
    }
}
