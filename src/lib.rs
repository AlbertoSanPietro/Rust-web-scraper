use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::io::Read;
pub fn parse_title(html: &str) -> Option<String> {
    let document = Html::parse_document(html);
    //TODO fix unwrap
    let selector = Selector::parse("title").expect("hardcoded title selector should be valid");

    document
        .select(&selector)
        .next()
        // .map(|e| e.text().collect::<String>().trim().to_string())
        .map(|e| e.text().collect::<String>())
        .map(|title| title.trim().to_owned())
        .filter(|title| !title.is_empty())
}

pub fn fetch_html(url: &str, client: &Client) -> Result<String, Box<dyn std::error::Error>> {
    let mut response = client.get(url).send()?;

    response.error_for_status_ref()?;

    let mut body = String::new();

    response.read_to_string(&mut body)?;

    Ok(body)
}
