use std::env;
use std::fs;
use std::io::Write;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant, SystemTime};

use reqwest::blocking::Client;

// info for each site
struct WebsiteStatus {
    url: String,
    status: String,
    time_ms: u128,
    timestamp: u64,
}

// Read URLs from file
fn read_urls() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    let mut urls = Vec::new();

    let mut i = 1;
    while i < args.len() {
        if args[i] == "--file" && i + 1 < args.len() {
            let file_content = fs::read_to_string(&args[i + 1]).expect("Failed to read file");
            for line in file_content.lines() {
                let trimmed = line.trim();
                if !trimmed.is_empty() && !trimmed.starts_with('#') {
                    urls.push(trimmed.to_string());
                }
            }
            i += 2;
        } 
        else {
            urls.push(args[i].clone());
            i += 1;
        }
    }

    if urls.is_empty() {
        eprintln!("Usage: website_checker [--file sites.txt] [URL ...]");
        std::process::exit(2);
    }

    urls
}

fn check_url(client: &Client, url: &str, timeout: u64, retries: u32) -> WebsiteStatus {
    let mut attempts = 0;
    let start = Instant::now();

    loop {
        let result = client.get(url).timeout(Duration::from_secs(timeout)).send();
        attempts += 1;

        let elapsed = start.elapsed().as_millis();
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        match result {
            Ok(response) => {
                return WebsiteStatus {
                    url: url.to_string(),
                    status: format!("{}", response.status().as_u16()),
                    time_ms: elapsed,
                    timestamp: now,
                };
            }
            Err(e) => {
                if attempts > retries {
                    return WebsiteStatus {
                        url: url.to_string(),
                        status: format!("Error: {}", e),
                        time_ms: elapsed,
                        timestamp: now,
                    };
                }
                thread::sleep(Duration::from_millis(100));
            }
        }
    }
}

fn main() {
    let urls = read_urls();

    let workers = 4; // fixed number of threads
    let timeout = 5; // seconds
    let retries = 0;

    let client = Client::builder()
        .timeout(Duration::from_secs(timeout))
        .build()
        .unwrap();

    let (tx, rx) = mpsc::channel();

    for chunk in urls.chunks(workers) {
        let mut handles = vec![];

        for url in chunk {
            let tx = tx.clone();
            let client = client.clone();
            let url = url.clone();

            let handle = thread::spawn(move || {
                let status = check_url(&client, &url, timeout, retries);
                tx.send(status).unwrap();
            });

            handles.push(handle);
        }

        for h in handles {
            h.join().unwrap();
        }
    }

    // results 
    let mut results = vec![];
    for _ in 0..urls.len() {
        let result = rx.recv().unwrap();
        println!(
            "{} - {} - {}ms",
            result.url, result.status, result.time_ms
        );
        results.push(result);
    }

    // Write to status.json 
    let mut json = String::from("[\n");
    for (i, res) in results.iter().enumerate() {
        json.push_str(&format!(
            "  {{ \"url\": \"{}\", \"status\": \"{}\", \"response_ms\": {}, \"timestamp\": {} }}",
            res.url, res.status, res.time_ms, res.timestamp
        ));
        if i < results.len() - 1 {
            json.push_str(",\n");
        } else {
            json.push_str("\n");
        }
    }
    json.push_str("]\n");

    let mut file = fs::File::create("status.json").expect("Could not create file");
    file.write_all(json.as_bytes())
        .expect("Failed to write JSON");
}
