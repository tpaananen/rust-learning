use crate::utils::print_line;
use reqwest::{Client, StatusCode};

pub async fn fetch_pages(
    client: &Client,
    page_number: usize,
    sub_index: usize,
    _error_message: &str,
) -> Result<Vec<String>, reqwest::Error> {
    let mut index = sub_index;
    let mut pages: Vec<String> = Vec::new();

    loop {
        let url_str = format!(
            "https://yle.fi/tekstitv/txt/{}_{:0>4}.htm",
            page_number, index
        );
        let response = client.get(&url_str).send().await?;

        if response.status() == StatusCode::NOT_FOUND {
            break;
        }

        let response = response.error_for_status()?;
        let html = response.text().await?;

        if html.is_empty() {
            break;
        }

        print_line();
        pages.push(html);
        index += 1;
    }

    Ok(pages)
}
