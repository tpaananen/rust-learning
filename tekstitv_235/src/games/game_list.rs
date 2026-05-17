use super::game::{Game, GameStatus};
use crate::MESSAGE;
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

    pub fn get_next_game_to_go(&self) -> &str {
        if self.games.is_empty() {
            return MESSAGE;
        }

        let mut rng = rand::rng();
        self
            .games
            .iter()
            .filter(|game| game.get_status() == GameStatus::Started)
            .choose(&mut rng)
            .map_or(MESSAGE, |game| game.get_home_team_name())
    }

    pub(crate) fn all_games_completed(&self) -> bool {
        self.games.is_empty() || self.games.iter().all(|game| game.is_completed())
    }
}
