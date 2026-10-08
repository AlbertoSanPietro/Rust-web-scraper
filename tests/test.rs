use web_scraper::parse_description;

#[ignore]
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
#[test]
pub fn test_missing_title() {
    let fake_html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
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
        "The parser should return None for an missing title"
    );
}

#[test]
fn test_valid_description() {
    let html = r#"
        <html>
            <head>
                <meta name="description" content="Hello">
            </head>
        </html>
    "#;

    let description = parse_description(html);

    assert_eq!(description.as_deref(), Some("Hello"));
}

#[test]
fn test_missing_description() {
    let html = r#"
        <html>
            <head>
                <title>Test Page</title>
            </head>
        </html>
    "#;

    let description = parse_description(html);

    assert_eq!(description, None);
}

#[test]
fn test_description_without_content() {
    let html = r#"
        <html>
            <head>
                <meta name="description">
            </head>
        </html>
    "#;

    let description = parse_description(html);

    assert_eq!(description, None);
}

#[test]
fn test_empty_description() {
    let html = r#"
        <html>
            <head>
                <meta name="description" content="">
            </head>
        </html>
    "#;

    let description = parse_description(html);

    assert_eq!(description, None);
}

#[test]
fn test_whitespace_description() {
    let html = r#"
        <html>
            <head>
                <meta name="description" content="   ">
            </head>
        </html>
    "#;

    let description = parse_description(html);

    assert_eq!(description, None);
}

#[test]
fn test_unrelated_meta_tag() {
    let html = r#"
        <html>
            <head>
                <meta name="viewport" content="width=device-width">
            </head>
        </html>
    "#;

    let description = parse_description(html);

    assert_eq!(description, None);
}
use web_scraper::{Heading, parse_headings};

#[test]
fn test_parse_headings() {
    let html = r#"
        <html>
            <body>
                <h1>Introduction</h1>
                <h2>Installation</h2>
                <h3>Dependencies</h3>
                <p>Not a heading</p>
                <h2>Usage</h2>
                <h4>   </h4>
            </body>
        </html>
    "#;

    let headings = parse_headings(html);

    let expected = vec![
        Heading {
            level: 1,
            text: "Introduction".to_string(),
        },
        Heading {
            level: 2,
            text: "Installation".to_string(),
        },
        Heading {
            level: 3,
            text: "Dependencies".to_string(),
        },
        Heading {
            level: 2,
            text: "Usage".to_string(),
        },
    ];

    assert_eq!(headings, expected);
}
