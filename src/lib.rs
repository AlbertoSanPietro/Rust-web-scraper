use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::io::Read;

#[derive(Debug, PartialEq, Eq)]
pub struct Heading {
    pub level: u8,
    pub text: String,
}

pub fn parse_headings(html: &str) -> Vec<Heading> {
    let document = Html::parse_document(html);

    let selector = Selector::parse("h1, h2, h3, h4, h5, h6").expect("headings should be valid");

    //took 2 hours to do ts
    document
        .select(&selector)
        .filter_map(|elem| {
            let level = elem.value().name().strip_prefix('h')?.parse::<u8>().ok()?;

            let text = elem.text().collect::<String>().trim().to_owned();

            if text.is_empty() {
                return None;
            }

            Some(Heading { level, text })
        })
        .collect()
}

pub fn parse_title(html: &str) -> Option<String> {
    let document = Html::parse_document(html);

    let selector = Selector::parse("title").expect("hardcoded title selector should be valid");

    document
        .select(&selector)
        .next()
        .map(|e| e.text().collect::<String>())
        .map(|title| title.trim().to_owned())
        .filter(|title| !title.is_empty())
}

pub fn parse_description(html: &str) -> Option<String> {
    let document = Html::parse_document(html);

    let selector = Selector::parse(r#"meta[name="description"]"#)
        .expect("hardcoded description selector should be valid");

    document
        .select(&selector)
        .next()
        .and_then(|elem| elem.value().attr("content"))
        .map(str::trim)
        .filter(|desc| !desc.is_empty())
        .map(str::to_owned)
}
pub fn fetch_html(url: &str, client: &Client) -> Result<String, Box<dyn std::error::Error>> {
    let mut response = client.get(url).send()?;

    response.error_for_status_ref()?;

    let mut body = String::new();

    response.read_to_string(&mut body)?;

    Ok(body)
}
