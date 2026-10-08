#[cfg(test)]
mod tests {
    #[test]
    pub fn test_connect() {
        let fake_host = "https://www.chiark.greenend.org.uk/gtatham/coroutines.html";
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(15))
            .build()
            .expect("Failed to build HTTP client");

        let body = web_scraper::fetch_html(fake_host, &client).expect("Failed to fetch html");

        assert!(!body.is_empty(), "The server returned an empty response");

        println!("Fetched {} bytes successfully", body.len());
    }

    #[test]
    pub fn test_title_parser() {
        let fake_html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Rust Web Scraper Test</title>
</head>
<body>
    <h1>Welcome to the Test Page</h1>
    <p>This is a hardcoded HTML document.</p>
    <h2>Another Heading</h2>
    <p>The scraper should extract the title from the head.</p>
</body>
</html>
"#;
        let title = web_scraper::parse_title(fake_html);

        assert_eq!(
            title.as_deref(),
            Some("Rust Web Scraper Test"),
            "The Parser returned an incorrect title"
        );
    }
    #[test]
    pub fn test_empty_title() {
        let fake_html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <title></title>
</head>
<body>
    <h1>Test Page</h1>
</body>
</html>
"#;

        let title = web_scraper::parse_title(fake_html);

        assert_eq!(
            title.as_deref(),
            None,
            "The parser should return None for an empty title"
        );
    }
}
