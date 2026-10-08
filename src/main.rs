use reqwest::blocking::Client;
use std::sync::{Arc, Mutex};
use std::thread;
use std::{error::Error, sync::mpsc};
use web_scraper::{ScrapedPage, fetch_html, parse_page};

type JobResult = (String, Result<ScrapedPage, reqwest::Error>);

fn main() -> Result<(), Box<dyn Error>> {
    //let url = "https://www.chiark.greenend.org.uk/~sgtatham/coroutines.html";
    const WORKERS: usize = 3;
    let urls = [
        "https://bbw-chan.link/",
        "https://example.com",
        "https://docs.rs/reqwest/latest/reqwest/blocking/struct.Client.html",
        "https://doc.rust-lang.org/book/ch11-01-writing-tests.html",
    ];

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()?;

    let (job_sender, job_receiver) = mpsc::channel::<String>();

    let (result_sender, result_receiver) = mpsc::channel::<JobResult>();

    let job_receiver = Arc::new(Mutex::new(job_receiver));

    let mut handles = Vec::new();
    for worker_id in 0..WORKERS {
        let worker_receiver = Arc::clone(&job_receiver);
        let worker_sender = result_sender.clone();
        let worker_client = client.clone();

        let handle = thread::spawn(move || {
            loop {
                let job = {
                    let receiver = worker_receiver.lock().expect("Job receiver mutex poisoned");

                    receiver.recv()
                };

                let url = match job {
                    Ok(url) => url,
                    Err(_) => break,
                };
                let result = fetch_html(&url, &worker_client).map(|html| parse_page(&url, &html));

                if worker_sender.send((url, result)).is_err() {
                    break;
                }
            }
        });

        handles.push((worker_id, handle));
    }

    drop(result_sender);

    for url in urls {
        job_sender.send(url.to_owned())?;
    }

    drop(job_sender);

    let mut pages: Vec<ScrapedPage> = Vec::new();
    let mut failed = 0usize;

    for (url, result) in result_receiver {
        match result {
            Ok(page) => {
                pages.push(page);
            }

            Err(error) => {
                failed += 1;
                eprintln!("Failed to scrape {url}: {error}");
            }
        }
    }
    let mut panicked = 0usize;

    for (worker_id, handle) in handles {
        if handle.join().is_err() {
            panicked += 1;
            eprintln!("Worker {worker_id} panicked");
        }
    }

    let completed = pages.len() + failed;

    if panicked > 0 || completed != urls.len() {
        return Err(format!(
            "Incomplete run: {completed}/{} jobs returned; \
             {panicked} workers panicked",
            urls.len()
        )
        .into());
    }

    pages.sort_by(|a, b| a.url.cmp(&b.url));

    let json = serde_json::to_string_pretty(&pages)?;
    println!("{json}");

    Ok(())
}
