use std::time::Duration;
use crate::{
    NUM_COLS, NUM_ROWS, NUM_SHOTS,
    frame::{Frame, Drawable, Position, Discoverable},
    shot::Shot, invaders::Invaders
};

pub struct Player {
    position: Position,
    shots: Vec<Shot>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            position: Position {
                col: NUM_COLS / 2,
                row: NUM_ROWS - 1,
            },
            shots: Vec::new(),
        }
    }

    pub fn move_left(&mut self) {
        if self.position.col > 0 {
            self.position.col -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.position.col < NUM_COLS - 1 {
            self.position.col += 1;
        }
    }

    pub fn shoot(&mut self) -> bool {
        if self.shots.len() < NUM_SHOTS {
            self.shots.push(Shot::new(self.position.col, self.position.row - 1));
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

    pub fn detect_hits(&mut self, invaders: &mut Invaders) -> bool {
        let mut hit = false;
        for shot in self.shots.iter_mut() {
            if !shot.is_exploding() {
                if invaders.kill_invader_at(shot) {
                    hit = true;
                    shot.explode();
                }
            }
        }
        hit
    }
}

impl Discoverable for Player {
    fn get_col(&self) -> usize { self.position.col }
    fn get_row(&self) -> usize { self.position.row }
    fn show(&self) -> &'static str { "A" }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame.update_item(self);
        for shot in self.shots.iter() {
            shot.draw(frame);
        }
    }
}
