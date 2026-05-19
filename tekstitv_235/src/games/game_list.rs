use super::game::{Game, GameStatus};
use rand::seq::IteratorRandom;

pub struct GameList {
    games: Vec<Game>,
}

impl GameList {
    pub(super) fn new() -> Self {
        GameList { games: Vec::new() }
    }

    pub(super) fn push(&mut self, game: Game) {
        self.games.push(game);
    }

    pub fn print(&self) {
        for game in &self.games {
            game.print();
            println!();
        }
    }

    pub fn get_next_game_to_go<'a>(&'a self, no_active_game_message: &'a str) -> &'a str {
        if self.games.is_empty() {
            return no_active_game_message;
        }

        let mut rng = rand::rng();
        self.games
            .iter()
            .filter(|game| game.get_status() == GameStatus::Started)
            .choose(&mut rng)
            .map_or(no_active_game_message, |game| game.get_home_team_name())
    }

    pub(crate) fn all_games_completed(&self) -> bool {
        self.games.is_empty() || self.games.iter().all(|game| game.is_completed())
    }
}
