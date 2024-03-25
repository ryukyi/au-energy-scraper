use chrono::{format::ParseError, NaiveDateTime};
use regex::Regex;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ReportMetadata {
    datetime: NaiveDateTime,
    url_path: String,
    unique_key: String,
}

fn build_url_lookup(urls: Vec<&str>) -> Result<BTreeMap<ReportMetadata, String>, ParseError> {
    let mut lookup = BTreeMap::new();

    for url in urls {
        let metadata = parse_url(url)?;
        lookup.insert(metadata, url.to_string());
    }

    Ok(lookup)
}

fn parse_url(url: &str) -> Result<ReportMetadata, ParseError> {
    let re = Regex::new(r"(.+)_\d{14}_(\d+)\.zip").unwrap();
    let caps = re.captures(url).unwrap();
    let url_path = caps.get(1).map_or("", |m| m.as_str()).to_string();
    let unique_key = caps.get(2).map_or("", |m| m.as_str()).to_string();

    // Extract datetime using a separate regex
    let datetime_re = Regex::new(r"\d{14}").unwrap();
    let datetime_str = datetime_re
        .find(url)
        .map_or("", |m| &url[m.start()..m.end()]);
    let datetime = NaiveDateTime::parse_from_str(datetime_str, "%Y%m%d%H%M%S")?;

    Ok(ReportMetadata {
        datetime,
        url_path,
        unique_key,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;

    #[test]
    fn test_parse_url_simple_pass() {
        let url = "http://nemweb.com.au/Reports/Current/TradingIS_Reports/PUBLIC_TRADINGIS_202403091025_0000000413211611.zip";
        let expected = ReportMetadata {
            datetime: NaiveDateTime::parse_from_str("20240309102500", "%Y%m%d%H%M%S").unwrap(),
            url_path: "Current/TradingIS_Reports/PUBLIC_TRADINGIS".to_string(),
            unique_key: "0000000413211611".to_string(),
        };
        let result = parse_url(url).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_build_url_lookup() {
        let urls = vec![
            "http://nemweb.com.au/Reports/Current/TradingIS_Reports/PUBLIC_TRADINGIS_202403091025_0000000413211611.zip",
        ];
        let lookup = build_url_lookup(urls).unwrap();
        let datetime_to_check =
            NaiveDateTime::parse_from_str("20240309102500", "%Y%m%d%H%M%S").unwrap();
        let metadata_to_check = ReportMetadata {
            datetime: datetime_to_check,
            url_path: "Current/TradingIS_Reports/PUBLIC_TRADINGIS".to_string(),
            unique_key: "0000000413211611".to_string(),
        };
        assert!(lookup.contains_key(&metadata_to_check));
    }
}
