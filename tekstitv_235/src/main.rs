use colored::*;
use crate::games::game_parser::fetch_games;

pub mod games;
pub mod regex_factory;
pub mod utils;
pub mod constants;

const MESSAGE: &'static str = "Jäämiehet varmaan hommissa...";

#[tokio::main]
async fn main() {
    println!();
    let games = fetch_games(true).await;
    games.print();

    //let games_on_going = read_and_print_pages(&pages);
    //print_selected_target(&find_target(&games_on_going));
    print_selected_target(MESSAGE);
}

fn print_selected_target(target: &str) {
    println!();
    println!("{}", "================================================================".bright_blue());
    println!();
    println!("{} {}", "> Lennän seuraavaksi:".bright_blue(), target);
    println!();
    println!("{}", "================================================================".bright_blue());
    println!();
}
