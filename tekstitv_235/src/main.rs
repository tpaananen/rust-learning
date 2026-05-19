use colored::Colorize;
use cli::{maybe_print_help, parse_args};
use games::{
    game_list::GameList,
    game_parser::{fetch_future_game_pages, fetch_games},
};
use localization::Locale;
use std::error::Error;
use utils::print_tonight;

use crate::utils::{print_line, print_selection, print_tomorrow};

pub mod constants;
pub mod cli;
pub mod games;
pub mod localization;
pub mod regex_factory;
pub mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let args: Vec<String> = std::env::args().collect();
    let cli = parse_args(&args[1..]);

    if maybe_print_help(cli) {
        return Ok(());
    }

    let locale = cli.locale;
    let use_mock_data = false;
    let games = fetch_games(use_mock_data, locale).await?;

    print_games(&games, locale);

    if games.all_games_completed() {
        let future_games = fetch_future_game_pages().await?;
        if !future_games.is_empty() {
            print_future_games(&future_games, locale);
        }
    }
    print_next_target(&games, locale);
    Ok(())
}

fn print_games(games: &GameList, locale: Locale) {
    print_tonight(locale);
    println!();
    games.print();
    println!();
}

fn print_next_target(games: &GameList, locale: Locale) {
    print_selection(locale);
    println!();
    println!(
        "{} {}",
        locale.next_target_label().bright_blue(),
        games.get_next_game_to_go(locale.no_active_game_message())
    );
    println!();
    print_line();
}

fn print_future_games(future_games: &[String], locale: Locale) {
    print_tomorrow(locale);
    for line in future_games {
        if line.contains("siir.") {
            println!("{}", line.trim_start().white().dimmed());
        } else if line.len() < 8 {
            println!("{}", line.trim_start().bright_yellow());
        } else {
            println!("{}", line.trim_start().bright_white());
        }
    }
}
