use colored::*;
use games::game_list::GameList;
use games::game_parser::fetch_games;
use utils::print_tonight;

use crate::utils::print_line;

pub mod games;
pub mod regex_factory;
pub mod utils;
pub mod constants;

const MESSAGE: &'static str = "Jäämiehet varmaan hommissa...";

#[tokio::main]
async fn main() {
    let use_mock_data = false;
    let games = &fetch_games(use_mock_data).await;
    print(games);
}

fn print(games: &GameList) {
    print_tonight();
    println!();
    games.print();
    print_line();
    println!();
    println!("{} {}", "> Lennän seuraavaksi:".bright_blue(), &games.get_next_game_to_go());
    println!();
    print_line();
    println!();
}
