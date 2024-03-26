use regex::Regex;

pub struct LinkExtractor {
    pattern: Regex,
}

impl LinkExtractor {
    pub fn new() -> Self {
        LinkExtractor {
            // Only keep links ending in zip
            pattern: Regex::new(r#"HREF="([^"]*\.zip)""#).unwrap(),
        }
    }

    pub fn extract_links(&self, html: &str) -> Vec<String> {
        self.pattern
            .captures_iter(html)
            .filter_map(|cap| cap.get(1))
            .map(|link| link.as_str().to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the outer module
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    #[test]
    fn test_extract_links_from_tradingis_html() {
        let extractor = LinkExtractor::new();
        let mut file =
            File::open(Path::new("src/fixtures/TradingIs.html")).expect("Failed to open HTML file");
        let mut html_content = String::new();
        file.read_to_string(&mut html_content)
            .expect("Failed to read HTML content");

        let links = extractor.extract_links(&html_content);

        let expected_links = vec![
            "/Reports/Current/TradingIS_Reports/PUBLIC_TRADINGIS_202403120535_0000000413460134.zip",
            "/Reports/Current/TradingIS_Reports/PUBLIC_TRADINGIS_202403120540_0000000413460407.zip",
            "/Reports/Current/TradingIS_Reports/PUBLIC_TRADINGIS_202403120545_0000000413460679.zip",
        ]
        .iter()
        .map(|&s| s.to_string())
        .collect::<Vec<String>>();

        assert_eq!(links, expected_links);
    }
}
