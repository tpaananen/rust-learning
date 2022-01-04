use colored::*;
use games::game_list::GameList;
use games::game_parser::fetch_games;

pub mod games;
pub mod regex_factory;
pub mod utils;
pub mod constants;

const MESSAGE: &'static str = "Jäämiehet varmaan hommissa...";

#[tokio::main]
async fn main() {
    let use_mock_data = false;
    print(&fetch_games(use_mock_data).await);
}

fn print(games: &GameList) {
    println!();
    games.print();
    println!("{}", "================================================================".bright_blue());
    println!();
    println!("{} {}", "> Lennän seuraavaksi:".bright_blue(), &games.get_next_game_to_go());
    println!();
    println!("{}", "================================================================".bright_blue());
    println!();
}
