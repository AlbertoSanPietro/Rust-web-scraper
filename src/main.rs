use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::{
    io::Read,
    process::{self},
};

/*
#[derive(Debug)]
struct ScrapedPage {
    url: String,
    title: Option<String>,
    description: Option<String>,
    headings: Vec<Heading>,
    links: Vec<String>,
}

#[derive(Debug)]
struct Heading {
    level: u8,
    text: String,
}
*/
fn parse_title(html: &str) -> Option<String> {
    let document = Html::parse_document(html);
    //TODO fix unwrap
    let selector = Selector::parse("title").expect("hardcoded title selector should be valid");

    document
        .select(&selector)
        .next()
        .map(|e| e.text().collect::<String>().trim().to_string())
}

fn fetch_html(url: &str, client: &Client) -> Result<String, Box<dyn std::error::Error>> {
    let mut response = client.get(url).send()?;

    response.error_for_status_ref()?;

    let mut body = String::new();

    response.read_to_string(&mut body)?;

    Ok(body)
}

fn main() {
    //let url = "https://www.chiark.greenend.org.uk/~sgtatham/coroutines.html";
    let url = "https://docs.rs/reqwest/latest/reqwest/blocking/struct.Client.html";

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .unwrap();

    let body: String = match fetch_html(url, &client) {
        Ok(body) => body,
        Err(error) => {
            eprintln!("Error : {}", error);
            process::exit(1);
        } //fix unwrap, probably with unwrap_or_else
    };

    println!("Length: {}", body.len());

    let title = parse_title(&body);

    let title_string = match title {
        Some(s) => s,
        None => " ".to_string(),
    };

    println!("title: {}", title_string);

    //There is definetely a more elegant way
    //I dont know it

    let fake_host = "https://www.chiark.greenend.org.uk/gtatham/coroutines.html";

    let fake_body = fetch_html(fake_host, &client);
    let fake_count: usize = match fake_body {
        Ok(fake_body) => {
            println!("Everything cool for now");
            fake_body.len()
        }
        Err(fake_error) => {
            eprintln!("There was an error\n{}", fake_error);
            0
        }
    };
    println!("Fake count: {}", fake_count);
}
