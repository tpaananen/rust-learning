use colored::*;
use games::{game_list::GameList, game_parser::{fetch_games, fetch_future_game_pages}};
use utils::print_tonight;

use crate::utils::{print_line, print_selection, print_tomorrow};

pub mod games;
pub mod regex_factory;
pub mod utils;
pub mod constants;

const MESSAGE: &'static str = "Jäämiehet hommissa...";

#[tokio::main]
async fn main() {
    let use_mock_data = false;
    let games = fetch_games(use_mock_data).await;

    print_games(&games);

    if games.all_games_completed() {
        let future_games = fetch_future_game_pages().await;
        if !future_games.is_empty() {
            print_future_games(&future_games);
        }
    }
    print_next_target(&games)
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
    println!("{} {}", "> Seuraava kohde:".bright_blue(), &games.get_next_game_to_go());
    println!();
    print_line();
}

fn print_future_games(future_games: &Vec<String>) {
    print_tomorrow();
    for line in future_games {
        if line.contains("siir.") {
            println!("{}", line.trim_start().white().dimmed());
        } else {
            println!("{}", line.trim_start().bright_white());
        }
    }
}
