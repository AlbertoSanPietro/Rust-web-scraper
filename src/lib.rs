use reqwest::blocking::Client;
use scraper::{Html, Selector};
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Heading {
    pub level: u8,
    pub text: String,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct ScrapedPage {
    pub url: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub headings: Vec<Heading>,
    pub links: Vec<String>,
}

pub fn parse_links(document: &Html) -> Vec<String> {
    let selector = Selector::parse("a[href]").expect("Harcoded links should be vaid");

    document
        .select(&selector)
        .filter_map(|elem| elem.value().attr("href"))
        .map(str::trim)
        .filter(|href| !href.is_empty())
        .map(str::to_owned)
        .collect()
}

pub fn parse_headings(document: &Html) -> Vec<Heading> {
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

pub fn parse_title(document: &Html) -> Option<String> {
    let selector = Selector::parse("title").expect("hardcoded title selector should be valid");

    document
        .select(&selector)
        .next()
        .map(|e| e.text().collect::<String>())
        .map(|title| title.trim().to_owned())
        .filter(|title| !title.is_empty())
}

pub fn parse_description(document: &Html) -> Option<String> {
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
pub fn fetch_html(url: &str, client: &Client) -> Result<String, reqwest::Error> {
    let response = client.get(url).send()?.error_for_status()?;

    response.text()
}

pub fn parse_page(url: &str, html: &str) -> ScrapedPage {
    let document = Html::parse_document(html);

    ScrapedPage {
        url: url.to_owned(),
        title: parse_title(&document),
        description: parse_description(&document),
        headings: parse_headings(&document),
        links: parse_links(&document),
    }
}
