use reqwest::blocking::Client;
use std::process::{self};
use web_scraper::fetch_html;
use web_scraper::parse_title;

fn main() {
    //let url = "https://www.chiark.greenend.org.uk/~sgtatham/coroutines.html";
    let url = "https://docs.rs/reqwest/latest/reqwest/blocking/struct.Client.html";

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .unwrap();
    let body = fetch_html(url, &client).unwrap_or_else(|error| {
        eprintln!("Error: {error}");
        process::exit(1);
    });

    println!("Length: {}", body.len());

    let title = parse_title(&body);

    let title_string = match title {
        Some(s) => s,
        None => "No title found...".to_string(),
    };

    println!("title: {}", title_string);
}
