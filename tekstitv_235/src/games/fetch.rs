use reqwest::Url;
use crate::utils::print_line;

pub async fn fetch_pages(page_number: usize, sub_index: usize, error_message: &str) -> Vec<String> {
    let mut index = sub_index;
    let mut pages: Vec<String> = Vec::new();
    loop {
        let url_str = format!("https://yle.fi/tekstitv/txt/{}_{:0>4}.htm", page_number, index);
        let url = Url::parse(&url_str).unwrap();
        let res = reqwest::get(url).await.expect(error_message);

        if !res.status().is_success() {
            break;
        }

        let html = res.text().await.unwrap_or("".to_string());
        if html.is_empty() {
            break;
        }

        print_line();
        pages.push(html);
        index += 1;
    }
    pages
}
