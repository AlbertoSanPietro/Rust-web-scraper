use reqwest::blocking::Client;
use std::error::Error;
use web_scraper::{fetch_html, parse_page};

fn main() -> Result<(), Box<dyn Error>> {
    //let url = "https://www.chiark.greenend.org.uk/~sgtatham/coroutines.html";
    let url = "https://docs.rs/reqwest/latest/reqwest/blocking/struct.Client.html";

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()?;

    let html = fetch_html(url, &client)?;

    let page = parse_page(url, &html);

    let json = serde_json::to_string_pretty(&page)?;

    println!("{json}");

    Ok(())
}
