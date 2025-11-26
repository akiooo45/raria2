use std::error::Error;
use std::fs::File;
use std::io::copy;
use std::path::Path;

use reqwest::{Url, blocking::Response};

// Function to perform the GET request and return the Response
fn get_url(url: &str) -> Result<Response, Box<dyn Error>> {
    let res = reqwest::blocking::get(url)?;
    Ok(res)
}

// Function to validate and parse the URL string.
// It returns a custom error message if parsing fails.
fn parse_url(url: &str) -> Result<Url, Box<dyn Error>> {
    // Attempt to parse the URL. If it fails, use map_err to replace the error message.
    Url::parse(url).map_err(|_| {
        // Convert the custom error message string into Box<dyn Error>
        format!("This is not Url format: {}", url).into()
    })
}

// Function to extract a filename from the URL path segments, similar to wget's default behavior
fn get_filename_from_url(url: &Url) -> String {
    url.path_segments()
        // Try to get the last segment of the path (e.g., /path/to/file.html -> file.html)
        .and_then(|segments| segments.last())
        // Filter out empty strings, which usually means the URL points to the root directory (e.g., https://example.com/)
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        // If no valid filename is found, use a default
        .unwrap_or("index.html")
        .to_string()
}

// Function that handles URL validation, fetching, and saving to a file
fn fetch_and_save(url_str: &str) -> Result<(), Box<dyn Error>> {
    // 1. URL format validation
    let valid_url = parse_url(url_str)?;

    // 2. Determine the output filename
    let filename = get_filename_from_url(&valid_url);
    println!("URL validated: {}", valid_url);

    // 3. Execute the network request
    let mut res = get_url(valid_url.as_str())?;

    // Check if the HTTP status code indicates success
    if !res.status().is_success() {
        return Err(format!("Request failed with status: {}", res.status()).into());
    }

    let path = Path::new(&filename);

    // 4. Create the local file
    println!("Creating file: {}", path.display());
    let mut file = File::create(path)?;

    // 5. Stream the response body directly to the file (avoids loading large files into memory)
    let bytes_written = copy(&mut res, &mut file)?;

    println!(
        "Successfully saved to {} ({} bytes).",
        path.display(),
        bytes_written
    );

    Ok(())
}

fn main() {
    let url_str =
        "https://chirsemby.top:8000/video/The.Shadows.Edge.2025.1080p.TS.HC.EN.MY.CN-RGB/Info.txt";
    // You can also try to download a specific file, e.g.:
    // let url_str = "https://www.rust-lang.org/logos/rust-logo-512x512.png";

    // Handle the final Result from the entire operation
    if let Err(e) = fetch_and_save(url_str) {
        eprintln!("Operation failed: {}", e);
    }
}
