use rand::Rng;
use crate::MESSAGE;
use super::game::{Game, GameStatus};

pub struct GameList {
    games: Vec<Game>
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

        let on_going_games = self.games.iter().filter(|game| game.get_status() == GameStatus::Started).collect::<Vec<_>>();
        if on_going_games.is_empty() {
            return MESSAGE;
        }

        let mut rng = rand::thread_rng();
        let rand_value: f64 = rng.gen();
        let len = on_going_games.len();
        let game = &on_going_games[(rand_value * 1000.0) as usize % len];
        game.get_home_team_name()
    }
}
