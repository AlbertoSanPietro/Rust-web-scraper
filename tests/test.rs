use scraper::Html;
use std::time::Duration;

use web_scraper::{
    Heading, ScrapedPage, fetch_html, parse_description, parse_headings, parse_links, parse_page,
    parse_title,
};

#[test]
#[ignore = "Requires an external network connection"]
fn test_connect() {
    let url = "https://docs.rs/reqwest/latest/reqwest/blocking/struct.Client.html";

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(15))
        .build()
        .expect("Failed to build HTTP client");

    let body = fetch_html(url, &client).expect("Failed to fetch HTML");

    assert!(!body.is_empty(), "The server returned an empty response");
}

#[test]
fn test_title_parser() {
    let html = r#"
        <!DOCTYPE html>
        <html>
            <head>
                <title>Rust Web Scraper Test</title>
            </head>
            <body>
                <h1>Welcome to the Test Page</h1>
            </body>
        </html>
    "#;

    let document = Html::parse_document(html);
    let title = parse_title(&document);

    assert_eq!(title.as_deref(), Some("Rust Web Scraper Test"));
}

#[test]
fn test_missing_title() {
    let html = r#"
        <html>
            <head></head>
            <body>
                <h1>This is NOT a title element</h1>
            </body>
        </html>
    "#;

    let document = Html::parse_document(html);
    let title = parse_title(&document);

    assert_eq!(title, None);
}

#[test]
fn test_empty_title() {
    let html = r#"
        <html>
            <head>
                <title></title>
            </head>
        </html>
    "#;

    let document = Html::parse_document(html);
    let title = parse_title(&document);

    assert_eq!(title, None);
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

    let document = Html::parse_document(html);
    let description = parse_description(&document);

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

    let document = Html::parse_document(html);
    let description = parse_description(&document);

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

    let document = Html::parse_document(html);
    let description = parse_description(&document);

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

    let document = Html::parse_document(html);
    let description = parse_description(&document);

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

    let document = Html::parse_document(html);
    let description = parse_description(&document);

    assert_eq!(description, None);
}

#[test]
fn test_unrelated_meta_tag() {
    let html = r#"
        <html>
            <head>
                <meta name="viewport"
                      content="width=device-width">
            </head>
        </html>
    "#;

    let document = Html::parse_document(html);
    let description = parse_description(&document);

    assert_eq!(description, None);
}

#[test]
fn test_description_trim() {
    let html = r#"
        <meta name="description"
              content="   Hello Rust!   ">
    "#;

    let document = Html::parse_document(html);
    let description = parse_description(&document);

    assert_eq!(description.as_deref(), Some("Hello Rust!"));
}

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

    let document = Html::parse_document(html);
    let headings = parse_headings(&document);

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

#[test]
fn test_no_headings() {
    let html = r#"
        <html>
            <body>
                <p>This is a paragraph.</p>
                <div>This is a div.</div>
                <span>This is a span.</span>
            </body>
        </html>
    "#;

    let document = Html::parse_document(html);
    let headings = parse_headings(&document);

    assert!(headings.is_empty());
}

#[test]
fn test_nested_heading_elements() {
    let html = r#"
        <html>
            <body>
                <h2>Learning <em>Rust</em> properly</h2>
            </body>
        </html>
    "#;

    let document = Html::parse_document(html);
    let headings = parse_headings(&document);

    let expected = vec![Heading {
        level: 2,
        text: "Learning Rust properly".to_string(),
    }];

    assert_eq!(headings, expected);
}

#[test]
fn test_heading_boundary_levels() {
    let html = r#"
        <html>
            <body>
                <h6>This is a valid heading</h6>
                <h7>This is NOT a valid heading</h7>
            </body>
        </html>
    "#;

    let document = Html::parse_document(html);
    let headings = parse_headings(&document);

    let expected = vec![Heading {
        level: 6,
        text: "This is a valid heading".to_string(),
    }];

    assert_eq!(headings, expected);
}

#[test]
fn test_parse_links() {
    let html = r#"
        <html>
            <body>
                <a href="https://www.rust-lang.org">Rust</a>
                <a href="/learn">Learning</a>
                <a href="">Empty</a>
                <a>No href attribute</a>
                <a href="  /about  ">About</a>
            </body>
        </html>
    "#;

    let document = Html::parse_document(html);
    let links = parse_links(&document);

    assert_eq!(
        links,
        vec!["https://www.rust-lang.org", "/learn", "/about",]
    );
}

#[test]
fn test_parse_complete_page() {
    let html = r#"
        <!DOCTYPE html>
        <html>
            <head>
                <title>Rust Web Scraper</title>
                <meta name="description"
                      content="Learning web scraping with Rust">
            </head>
            <body>
                <h1>Introduction</h1>
                <h2>Learning <em>Rust</em> properly</h2>

                <a href="/docs">Documentation</a>
                <a href="https://www.rust-lang.org">Rust</a>
            </body>
        </html>
    "#;

    let page = parse_page("https://example.com", html);

    let expected = ScrapedPage {
        url: "https://example.com".to_string(),
        title: Some("Rust Web Scraper".to_string()),
        description: Some("Learning web scraping with Rust".to_string()),
        headings: vec![
            Heading {
                level: 1,
                text: "Introduction".to_string(),
            },
            Heading {
                level: 2,
                text: "Learning Rust properly".to_string(),
            },
        ],
        links: vec!["/docs".to_string(), "https://www.rust-lang.org".to_string()],
    };

    assert_eq!(page, expected);
}

#[test]
fn test_json_serialization() {
    let html = r#"
        <html>
            <head>
                <title>Test Page</title>
            </head>
            <body>
                <h1>Hello Rust</h1>
            </body>
        </html>
    "#;

    let page = parse_page("https://example.com", html);

    let json = serde_json::to_string_pretty(&page).expect("Failed to serialize ScrapedPage");

    let deserialized: serde_json::Value =
        serde_json::from_str(&json).expect("Generated JSON is invalid");

    let expected = serde_json::json!({
        "url": "https://example.com",
        "title": "Test Page",
        "description": null,
        "headings": [
            {
                "level": 1,
                "text": "Hello Rust"
            }
        ],
        "links": []
    });

    assert_eq!(deserialized, expected);
}
