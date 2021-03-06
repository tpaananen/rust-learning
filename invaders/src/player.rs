use std::time::Duration;
use crate::shot::Shot;
use crate::invaders::Invaders;
use crate::frame::{Frame, Drawable, Position, Discoverable};

pub struct Player {
    position: Position,
    shots: Vec<Shot>,
    num_total_shots_taken: usize,
    num_columns: usize,
    num_shots: usize
}

impl Player {
    pub fn new(num_rows: usize, num_columns: usize, num_shots: usize) -> Self {
        Self {
            position: Position {
                col: num_columns / 2,
                row: num_rows - 1,
            },
            shots: Vec::new(),
            num_total_shots_taken: 0,
            num_columns,
            num_shots
        }
    }

    pub fn move_left(&mut self) {
        if self.position.col > 0 {
            self.position.col -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.position.col < self.num_columns - 1 {
            self.position.col += 1;
        }
    }

    pub fn shoot(&mut self) -> bool {
        if self.shots.len() < self.num_shots {
            self.shots.push(Shot::new(self.position.col, self.position.row - 1));
            self.num_total_shots_taken += 1;
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, delta: Duration) {
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }
        self.shots.retain(|shot| !shot.is_dead());
    }

    pub fn detect_hits(&mut self, invaders: &mut Invaders) -> usize {
        let mut count = 0;
        for shot in self.shots.iter_mut() {
            if !shot.is_exploding() {
                if invaders.kill_invader_at(shot) {
                    count += 1;
                    shot.explode();
                }
            }
        }
        count
    }

    pub fn count_shots(&self) -> usize {
        self.num_total_shots_taken
    }
}

impl Discoverable for Player {
    fn get_col(&self) -> usize { self.position.col }
    fn get_row(&self) -> usize { self.position.row }
    fn show(&self) -> char { 'A' }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame.update_item(self);
        for shot in self.shots.iter() {
            shot.draw(frame);
        }
    }
}
