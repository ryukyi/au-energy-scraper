use chrono::{format::ParseError, NaiveDateTime};
use std::fmt;
use std::path::Path;
use std::str::FromStr;

// Generic wrapper struct for string types of path url
// For example url:
// "/Reports/Current/TradingIS_Reports/PUBLIC_TRADINGIS_202403120535_0000000413460134.zip"
#[derive(Debug, Clone, PartialEq)]
pub struct StringWrapper(String);

impl FromStr for StringWrapper {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(StringWrapper(s.to_string()))
    }
}

type UrlPath = StringWrapper;
type FileName = StringWrapper;
type UniqueKey = StringWrapper;

#[derive(Debug)]
pub struct ParsedReport {
    pub url_path: UrlPath,
    pub file_name: FileName,
    pub datetime: NaiveDateTime,
    pub unique_key: UniqueKey,
}

impl ParsedReport {
    pub fn parse_report_path(report_path: &str) -> Result<Self, ParseError> {
        let path = Path::new(report_path);
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let url_path = report_path.trim_end_matches(file_name);

        let without_extension = file_name.trim_end_matches(".zip");
        let parts: Vec<&str> = without_extension.rsplitn(3, '_').collect();
        let unique_key = parts[0];
        let datetime_str = parts[1];

        let datetime = NaiveDateTime::parse_from_str(datetime_str, "%Y%m%d%H%M")?;

        Ok(Self {
            url_path: UrlPath::from_str(url_path)?,
            file_name: FileName::from_str(file_name)?,
            datetime,
            unique_key: UniqueKey::from_str(unique_key)?,
        })
    }
}

impl fmt::Display for ParsedReport {
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
        let expected = ParsedReport {
            url_path: UrlPath::from_str("/Reports/Current/TradingIS_Reports/").unwrap(),
            file_name: FileName::from_str("PUBLIC_TRADINGIS_202403120535_0000000413460134.zip")
                .unwrap(),
            datetime: NaiveDateTime::parse_from_str("202403120535", "%Y%m%d%H%M").unwrap(),
            unique_key: UniqueKey::from_str("0000000413460134").unwrap(),
        };

        let result =
            ParsedReport::parse_report_path(sample_href).expect("Failed to parse report path");

        assert_eq!(result.url_path, expected.url_path);
        assert_eq!(result.file_name, expected.file_name);
        assert_eq!(result.datetime, expected.datetime);
        assert_eq!(result.unique_key, expected.unique_key);
    }
}
