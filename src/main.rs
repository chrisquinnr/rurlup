use reqwest::StatusCode;
use std::collections::HashMap;
use std::time::Duration;
use tokio;
use url::Url;

fn validate_url(url: &str) -> bool {
    match Url::parse(url) {
        Ok(_) => true,
        Err(_) => false,
    }
}

async fn check_url(url: &str) -> bool {
    match reqwest::get(url).await {
        Ok(response) => response.status() == StatusCode::OK,
        Err(_) => false,
    }
}

#[tokio::main]
async fn main() {
    let mut urls = std::env::args().skip(1).collect::<Vec<_>>();

    // Validate the URLs and remove any invalid ones
    urls.retain(|url| validate_url(url));

    // Create a HashMap to store the URLs and their status
    let mut url_status_map = urls
        .iter()
        .map(|url| (url.clone(), true))
        .collect::<HashMap<_, _>>();

    loop {
        // Check each URL
        for url in &urls {
            let is_up = check_url(url).await;

            // Update the URL status in the HashMap
            url_status_map.insert(url.clone(), is_up);

            // Notify the user if the URL is down
            if !is_up {
                println!("{} is down!", url);
            } else {
                println!("{} is up!", url);
            }
        }

        // Wait for one minute before checking again
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_url_valid() {
        assert!(validate_url("https://www.google.com"));
        assert!(validate_url("http://localhost:8000"));
    }

    #[test]
    fn test_validate_url_invalid() {
        assert!(!validate_url("invalid-url"));
    }

    #[tokio::test]
    async fn test_check_url_up() {
        assert!(check_url("https://www.google.com").await);
    }

    #[tokio::test]
    async fn test_check_url_down() {
        assert!(!check_url("https://www.google.com/404").await);
    }
}
