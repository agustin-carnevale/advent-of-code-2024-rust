use dotenv::dotenv;
use reqwest::blocking::Client;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <day_number>", args[0]);
        process::exit(1);
    }

    // Validate day_number
    let day_number: u8 = match args[1].parse() {
        Ok(num) if num >= 1 && num <= 25 => num,
        _ => {
            eprintln!("Error: day_number must be a number between 1 and 25.");
            process::exit(1);
        }
    };

    // Construct URL and file path
    let url = format!("https://adventofcode.com/2024/day/{}/input", day_number);
    let file_path = format!(
        "/Users/agustincarnevale/advent-of-code/2024/aoc-2024-d{}/src/bin/input.txt",
        day_number
    );

    // Make the HTTP request
    let client = Client::new();
    let session_cookie = match env::var("AOC_SESSION_ID_COOKIE") {
        Ok(cookie) => cookie,
        Err(_) => {
            eprintln!("Error: SESSION_COOKIE environment variable not set.");
            process::exit(1);
        }
    };

    let response = client
        .get(&url)
        .header("Cookie", format!("session={}", session_cookie))
        .send();

    match response {
        Ok(resp) if resp.status().is_success() => {
            let content = resp.text().unwrap();

            // Save content to the specified file path
            let path = Path::new(&file_path);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            fs::write(path, content).unwrap();
            println!("File saved to {}", file_path);
        }
        Ok(resp) => {
            eprintln!(
                "Failed to download file: HTTP {}
{}",
                resp.status(),
                resp.text().unwrap_or_default()
            );
            process::exit(1);
        }
        Err(err) => {
            eprintln!("Failed to make request: {}", err);
            process::exit(1);
        }
    }
}
