use colored::Colorize;
use games::{
    game_list::GameList,
    game_parser::{fetch_future_game_pages, fetch_games},
};
use std::error::Error;
use utils::print_tonight;

use crate::utils::{print_line, print_selection, print_tomorrow};

pub mod constants;
pub mod games;
pub mod regex_factory;
pub mod utils;

const MESSAGE: &str = "Jäämiehet hommissa...";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let use_mock_data = false;
    let games = fetch_games(use_mock_data).await?;

    print_games(&games);

    if games.all_games_completed() {
        let future_games = fetch_future_game_pages().await?;
        if !future_games.is_empty() {
            print_future_games(&future_games);
        }
    }
    print_next_target(&games);
    Ok(())
}

fn print_games(games: &GameList) {
    print_tonight();
    println!();
    games.print();
    println!();
}

fn print_next_target(games: &GameList) {
    print_selection();
    println!();
    println!(
        "{} {}",
        "> Seuraava kohde:".bright_blue(),
        &games.get_next_game_to_go()
    );
    println!();
    print_line();
}

fn print_future_games(future_games: &[String]) {
    print_tomorrow();
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
