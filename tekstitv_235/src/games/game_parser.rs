use crate::games::game::{Game};
use crate::regex_factory::RegexFactory;
use crate::utils::is_empty_or_whitespace;
use super::game_list::GameList;
use reqwest::Url;
use scraper::{Html, Selector};

pub async fn fetch_games(use_mock_data: bool) -> GameList {
    let mut index = 1;
    let mut pages: Vec<String> = Vec::new();

    if use_mock_data {
        // for some testing
        let contents = std::fs::read_to_string("./assets/sivu0001.htm")
            .expect("Something went wrong reading the file");
        pages.push(contents);
    }

    loop {
        let url_str = format!("https://yle.fi/tekstitv/txt/235_{:0>4}.htm", index);
        let url = Url::parse(&url_str).unwrap();
        let res = reqwest::get(url).await.expect("Failed to load results from YLE web site");

        if !res.status().is_success() {
            break;
        }

        let html = res.text().await.unwrap_or("".to_string());
        if html.is_empty() {
            break;
        }

        pages.push(html);
        index += 1;
    }

    parse_games(&pages)
}

fn parse_games(pages: &Vec<String>) -> GameList {
    let selector = Selector::parse(".boxbox > pre").unwrap();
    let mut games = GameList::new();
    let regex_factory = RegexFactory::new();

    for page in pages {
        let document = Html::parse_document(&page);
        let lines = document
            .select(&selector)
            .flat_map(|element| { element.text().flat_map(|text| { text.lines() })})
            .filter(|line| { !line.contains("NHL-") });

        let mut game_lines: Vec<&str> = Vec::new();
        for line in lines {
            let trimmed_line = line.trim();
            if game_lines.len() > 0 && is_empty_or_whitespace(trimmed_line) {
                games.push(Game::from_lines(game_lines, &regex_factory));
                game_lines = Vec::new();
            } else if !is_empty_or_whitespace(line) {
                game_lines.push(trimmed_line);
            }
        }
    }
    games
}
