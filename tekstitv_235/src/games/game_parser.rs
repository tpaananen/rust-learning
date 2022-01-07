use crate::constants::{COL_WIDTH_PLAYER_NAME, COL_WIDTH_PLAYER};
use crate::games::game::{Game};
use crate::regex_factory::RegexFactory;
use crate::utils::{is_empty_or_whitespace, print_line, print_loading};
use super::game_list::GameList;
use reqwest::Url;
use scraper::{Html, Selector};

pub async fn fetch_games(use_mock_data: bool) -> GameList {
    print_loading();
    let regex_factory = RegexFactory::new();
    let game_lines = fetch_game_pages(use_mock_data).await;
    let finnish_players = fetch_finnish_players().await;
    let mut games = GameList::new();
    parse_games(&mut games, &game_lines, &finnish_players, &regex_factory);

    if games.all_games_completed() {
        fetch_future_game_pages(&mut games, &regex_factory).await
    }

    games
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

        print_line();
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

        if html.contains("NHL-MAALIVAHDIT") {
            break;
        }

        print_line();
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
        let filtering = document
            .select(&selector)
            .flat_map(|element| element.text().flat_map(|text| text.lines()))
            .filter(|line| !line.contains("NHL"));

        let mut previous_was_empty = false;
        for line in filtering {
            let empty = is_empty_or_whitespace(line);

            if previous_was_empty && empty {
                break;
            }
            previous_was_empty = empty;
            lines.push(line.to_owned());
        }
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
            .flat_map(|element| element.text().flat_map(|text| text.lines()))
            .filter(|line| !is_empty_or_whitespace(line) && line.len() > COL_WIDTH_PLAYER && !line.contains("NHL"))
            .for_each(|line| lines.push(line.trim().chars().take(COL_WIDTH_PLAYER_NAME).collect::<String>().split(" ").nth(1).unwrap().to_owned()));
    }
    lines
}

async fn fetch_future_game_pages(games: &mut GameList, regex_factory: &RegexFactory) {
    let mut pages: Vec<String> = Vec::new();
    let mut index = 3;

    loop {
        let url_str = format!("https://yle.fi/tekstitv/txt/237_{:0>4}.htm", index);
        let url = Url::parse(&url_str).unwrap();
        let res = reqwest::get(url).await.expect("Failed to load future games from YLE web site");

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

    let game_lines = parse_lines(&pages);
    parse_future_games(games, &game_lines, regex_factory);
}

fn parse_games(games: &mut GameList, game_lines: &Vec<String>, finnish_players: &Vec<String>, regex_factory: &RegexFactory) {
    let mut lines: Vec<&str> = Vec::new();
    for line in game_lines {
        if !lines.is_empty() && is_empty_or_whitespace(line) {
            if let Some(game) = Game::from_lines(&lines, finnish_players, &regex_factory) {
                games.push(game);
            }
            lines.clear();
        } else if !is_empty_or_whitespace(line) {
            lines.push(line);
        }
    }
}

fn parse_future_games(games: &mut GameList, game_lines: &Vec<String>, regex_factory: &RegexFactory) {
    let finnish_players: Vec<String> = Vec::new();
    let mut lines: Vec<&str> = Vec::with_capacity(1);
    for line in game_lines {
        if line.len() > 7 && !is_empty_or_whitespace(line) {
            lines.push(line);
            if let Some(game) = Game::from_lines(&lines, &finnish_players, regex_factory) {
                games.push(game);
            }
            lines.clear();
        }
    }
}
