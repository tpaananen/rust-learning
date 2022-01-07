use crate::constants::{COL_WIDTH_PLAYER_NAME, COL_WIDTH_PLAYER};
use crate::games::game::{Game};
use crate::regex_factory::RegexFactory;
use crate::utils::{is_empty_or_whitespace, print_loading};
use super::fetch::fetch_pages;
use super::game_list::GameList;
use scraper::{Html, Selector};

pub async fn fetch_games(use_mock_data: bool) -> GameList {
    print_loading();
    let regex_factory = RegexFactory::new();
    let game_lines = fetch_game_pages(use_mock_data).await;
    let finnish_players = fetch_finnish_players().await;
    let mut games = GameList::new();
    parse_games(&mut games, &game_lines, &finnish_players, &regex_factory);
    games
}

async fn fetch_game_pages(use_mock_data: bool) -> Vec<String> {
    let page_number = 235;
    let sub_index = 1;
    let error_message = "Failed to load results from YLE web site";
    let mut pages = fetch_pages(page_number, sub_index, error_message).await;
    if use_mock_data {
        // for some testing
        let contents = std::fs::read_to_string("./assets/sivu0001.htm")
            .expect("Something went wrong reading the file");
        pages.push(contents);
    }
    parse_lines(&pages)
}

async fn fetch_finnish_players() -> Vec<String> {
    let page_number = 238;
    let sub_index = 2;
    let error_message = "Failed to load finnish players from YLE web site";
    let pages = fetch_pages(page_number, sub_index, error_message).await;
    parse_player_lines(&pages)
}

pub async fn fetch_future_game_pages() -> Vec<String> {
    let page_number = 237;
    let sub_index = 3;
    let error_message = "Failed to load future games from YLE web site";
    let pages = fetch_pages(page_number, sub_index, error_message).await;
    parse_lines(&pages)
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
