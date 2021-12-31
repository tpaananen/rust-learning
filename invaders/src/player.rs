use std::time::Duration;
use crate::{
    NUM_COLS, NUM_ROWS, NUM_SHOTS,
    frame::{Frame, Drawable},
    shot::Shot, invaders::Invaders
};

pub struct Player {
    col: usize,
    row: usize,
    shots: Vec<Shot>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            col: NUM_COLS / 2,
            row: NUM_ROWS - 1,
            shots: Vec::new(),
        }
    }

    pub fn move_left(&mut self) {
        if self.col > 0 {
            self.col -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.col < NUM_COLS - 1 {
            self.col += 1;
        }
    }

    pub fn shoot(&mut self) -> bool {
        if self.shots.len() < NUM_SHOTS {
            self.shots.push(Shot::new(self.col, self.row - 1));
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, delta: Duration) {
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }
        self.shots.retain(|shot| !shot.dead());
    }

    pub fn detect_hits(&mut self, invaders: &mut Invaders) -> bool {
        let mut hit = false;
        for shot in self.shots.iter_mut() {
            if !shot.exploding {
                if invaders.kill_invader_at(shot.row, shot.col) {
                    hit = true;
                    shot.explode();
                }
            }
        }
        hit
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.col][self.row] = "A";
        for shot in self.shots.iter() {
            shot.draw(frame);
        }
    }
}
