//! Auto-generated main module

#![allow(unused_imports)]
#![allow(dead_code)]

pub mod globals;
pub mod src_url;
pub mod types;

fn main() {
    // Example usage
    let url = "https://user:pass@example.com:8080/path/to/page?query=1#fragment";
    println!("Parsing: {}", url);
    match src_url::url_parse(url) {
        Some(data) => src_url::url_data_inspect(&data),
        None => println!("Failed to parse URL"),
    }
}