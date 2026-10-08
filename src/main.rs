use reqwest::blocking::Client;
use std::error::Error;
use std::thread;
use web_scraper::{ScrapedPage, fetch_html, parse_page};

fn main() -> Result<(), Box<dyn Error>> {
    //let url = "https://www.chiark.greenend.org.uk/~sgtatham/coroutines.html";
    let urls = [
        "https://docs.rs/reqwest/latest/reqwest/blocking/struct.Client.html",
        "https://doc.rust-lang.org/book/ch11-01-writing-tests.html",
    ];

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()?;
    let mut handles = Vec::new();

    for url in urls {
        let wrk_url = url.to_owned();
        let wrk_client = client.clone();

        let handle = thread::spawn(move || -> Result<ScrapedPage, reqwest::Error> {
            let html = fetch_html(&wrk_url, &wrk_client)?;

            let page = parse_page(&wrk_url, &html);

            Ok(page)
        });

        handles.push((url, handle));
    }

    let mut pages: Vec<ScrapedPage> = Vec::new();

    for (url, handle) in handles {
        match handle.join() {
            Ok(Ok(page)) => {
                pages.push(page);
            }
            Ok(Err(error)) => {
                eprintln!("Failed to fetch {url}: {error}");
            }

            Err(_) => {
                eprintln!("Thrad panicked while preoicessing {url}");
            }
        }
    }

    let json = serde_json::to_string_pretty(&pages)?;

    println!("{json}");

    Ok(())
}
