use crate::games::game::{Game};
use crate::regex_factory::RegexFactory;
use crate::utils::is_empty_or_whitespace;
use super::game_list::GameList;
use reqwest::Url;
use scraper::{Html, Selector};

pub async fn fetch_games(use_mock_data: bool) -> GameList {
    let game_lines = fetch_game_pages(use_mock_data).await;
    let finnish_players = fetch_finnish_players().await;
    parse_games(&game_lines, &finnish_players)
}

async fn fetch_game_pages(use_mock_data: bool) -> Vec<String> {
    let mut pages: Vec<String> = Vec::new();
    let mut index = 1;

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

    parse_lines(&pages)
}

async fn fetch_finnish_players() -> Vec<String> {
    let mut finnish_players: Vec<String> = Vec::new();
    let mut index = 2;
    loop {
        let url_str = format!("https://yle.fi/tekstitv/txt/238_{:0>4}.htm", index);
        let url = Url::parse(&url_str).unwrap();
        let res = reqwest::get(url).await.expect("Failed to load results from YLE web site");

        if !res.status().is_success() {
            break;
        }

        let html = res.text().await.unwrap_or("".to_string());
        if html.is_empty() {
            break;
        }

        finnish_players.push(html);
        index += 1;
    }

    parse_player_lines(&finnish_players)
}

fn parse_lines(pages: &Vec<String>) -> Vec<String> {
    let selector = Selector::parse(".boxbox > pre").unwrap();
    let mut lines: Vec<String> = Vec::new();

    for page in pages {
        let document = Html::parse_document(&page);
        document
            .select(&selector)
            .flat_map(|element| { element.text().flat_map(|text| { text.lines() })})
            .filter(|line| { !line.contains("NHL-") })
            .for_each(|line| { lines.push(line.trim().to_owned()) });
    }
    lines
}

fn parse_player_lines(pages: &Vec<String>) -> Vec<String> {
    let selector = Selector::parse(".boxbox > pre").unwrap();
    let mut lines: Vec<String> = Vec::new();

    for page in pages {
        // expect they do not score any time soon, Pekka is not playing anymore :)
        if page.contains("NHL-MAALIVAHDIT") {
            continue;
        }

        let document = Html::parse_document(&page);
        document
            .select(&selector)
            .flat_map(|element| { element.text().flat_map(|text| { text.lines() })})
            .filter(|line| { !is_empty_or_whitespace(line) && line.len() > 21 && !line.contains("NHL-") })
            .for_each(|line| { lines.push(line.trim().chars().take(18).collect::<String>().split(" ").nth(1).unwrap().to_owned()) });
    }
    lines
}

fn parse_games(game_lines: &Vec<String>, finnish_players: &Vec<String>) -> GameList {
    let mut games = GameList::new();
    let regex_factory = RegexFactory::new();

    let mut game: Vec<&str> = Vec::new();
    for line in game_lines {
        if game.len() > 0 && is_empty_or_whitespace(line) {
            games.push(Game::from_lines(game, finnish_players, &regex_factory));
            game = Vec::new();
        } else if !is_empty_or_whitespace(line) {
            game.push(line);
        }
    }

    games
}
