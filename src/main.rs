use reqwest::blocking::get;
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
fn send_html_get(hostname: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut response = get(hostname)?;

    response.error_for_status_ref()?;

    let mut body = String::new();

    response.read_to_string(&mut body)?;

    Ok(body)
}

fn main() {
    let hostname = "https://www.chiark.greenend.org.uk/~sgtatham/coroutines.html";

    println!("Here!");

    let body: String = match send_html_get(hostname) {
        Ok(body) => body,
        Err(error) => {
            eprintln!("Error : {}", error);
            process::exit(1);
        } //fix unwrap, probably with unwrap_or_else
    };

    println!("Length: {}", body.len());

    let html = r#"
<html>
<head>
</head>
<body></body>
</html>
"#;
    let title = parse_title(html);

    let title_string = match title {
        Some(s) => s,
        None => " ".to_string(),
    };

    println!("title: {}", title_string);

    //There is definetely a more elegant way
    //I dont know it
    /*
    let fake_host = "https://www.chiark.greenend.org.uk/gtatham/coroutines.html";

    let fake_body = send_html_get(fake_host);
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
    */
}
