use super::game::{Game, GameStatus};
use crate::MESSAGE;
use rand::RngExt;

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

        let on_going_games = self
            .games
            .iter()
            .filter(|game| game.get_status() == GameStatus::Started)
            .collect::<Vec<_>>();

        if on_going_games.is_empty() {
            return MESSAGE;
        }

        let len = on_going_games.len();
        let mut rng = rand::rng();
        let game = &on_going_games[rng.random_range(0..len)];
        game.get_home_team_name()
    }

    pub(crate) fn all_games_completed(&self) -> bool {
        self.games.len() == 0 || self.games.iter().all(|game| game.is_completed())
    }
}
