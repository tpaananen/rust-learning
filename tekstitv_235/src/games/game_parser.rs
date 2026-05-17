use super::fetch::fetch_pages;
use super::game_list::GameList;
use crate::constants::{COL_WIDTH_PLAYER, COL_WIDTH_PLAYER_NAME};
use crate::games::game::Game;
use crate::regex_factory::RegexFactory;
use crate::utils::{is_empty_or_whitespace, print_loading};
use reqwest::Client;
use scraper::{Html, Selector};
use std::error::Error;
use std::sync::LazyLock;

type AppResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

static BOXBOX_PRE_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(".boxbox > pre").expect("selector literal should parse"));

pub async fn fetch_games(use_mock_data: bool) -> AppResult<GameList> {
    print_loading();
    let client = create_http_client()?;
    let regex_factory = RegexFactory::new();
    let (game_lines, finnish_players) = tokio::try_join!(
        fetch_game_pages(&client, use_mock_data),
        fetch_finnish_players(&client)
    )?;

    let mut games = GameList::new();
    parse_games(&mut games, &game_lines, &finnish_players, &regex_factory);
    Ok(games)
}

fn create_http_client() -> Result<Client, reqwest::Error> {
    Client::builder()
        .connect_timeout(std::time::Duration::from_secs(10))
        .timeout(std::time::Duration::from_secs(20))
        .user_agent("tekstitv_235/1.0")
        .build()
}

async fn fetch_game_pages(client: &Client, use_mock_data: bool) -> AppResult<Vec<String>> {
    let page_number = 235;
    let sub_index = 1;
    let error_message = "Failed to load results from YLE web site";
    let mut pages = fetch_pages(client, page_number, sub_index, error_message).await?;

    if use_mock_data {
        // for some testing
        let contents = std::fs::read_to_string("./assets/sivu0001.htm")?;
        pages.push(contents);
    }

    Ok(parse_lines(&pages))
}

async fn fetch_finnish_players(client: &Client) -> AppResult<Vec<String>> {
    let page_number = 238;
    let sub_index = 2;
    let error_message = "Failed to load finnish players from YLE web site";
    let pages = fetch_pages(client, page_number, sub_index, error_message).await?;
    Ok(parse_player_lines(&pages))
}

pub async fn fetch_future_game_pages() -> AppResult<Vec<String>> {
    let client = create_http_client()?;
    let page_number = 237;
    let sub_index = 3;
    let error_message = "Failed to load future games from YLE web site";
    let pages = fetch_pages(&client, page_number, sub_index, error_message).await?;
    Ok(parse_lines(&pages))
}

fn parse_lines(pages: &[String]) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();

    for page in pages {
        let document = Html::parse_document(page);
        let filtering = document
            .select(&BOXBOX_PRE_SELECTOR)
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

fn parse_player_lines(pages: &[String]) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();

    for page in pages {
        // expect they do not score any time soon, Pekka is not playing anymore :)
        if page.contains("NHL-MAALIVAHDIT") {
            continue;
        }

        let document = Html::parse_document(page);
        document
            .select(&BOXBOX_PRE_SELECTOR)
            .flat_map(|element| element.text().flat_map(|text| text.lines()))
            .filter(|line| {
                !is_empty_or_whitespace(line)
                    && line.len() > COL_WIDTH_PLAYER
                    && !line.contains("NHL")
            })
            .for_each(|line| {
                let shortened = line
                    .trim()
                    .chars()
                    .take(COL_WIDTH_PLAYER_NAME)
                    .collect::<String>();
                if let Some(player_name) = shortened.split_whitespace().nth(1) {
                    lines.push(player_name.to_owned());
                }
            });
    }
    lines
}

fn parse_games(
    games: &mut GameList,
    game_lines: &[String],
    finnish_players: &[String],
    regex_factory: &RegexFactory,
) {
    let mut lines: Vec<&str> = Vec::new();
    for line in game_lines {
        if !lines.is_empty() && is_empty_or_whitespace(line) {
            if let Some(game) = Game::from_lines(&lines, finnish_players, regex_factory) {
                games.push(game);
            }
            lines.clear();
        } else if !is_empty_or_whitespace(line) {
            lines.push(line);
        }
    }

    if let Some(game) = Game::from_lines(&lines, finnish_players, regex_factory) {
        games.push(game);
    }
}
