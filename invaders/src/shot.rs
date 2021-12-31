use std::time::Duration;

use rusty_time::prelude::Timer;

use crate::frame::{Drawable, Frame};

pub struct Shot {
    pub col: usize,
    pub row: usize,
    pub exploding: bool,
    timer: Timer
}

impl Shot {
    pub fn new(col: usize, row: usize) -> Self {
        Self { col, row, exploding: false, timer: Timer::from_millis(16) }
    }

    pub fn update(&mut self, delta: Duration) {
        self.timer.update(delta);
        if self.timer.ready && !self.exploding {
            if self.row > 0 {
                self.row -= 1;
            }
            self.timer.reset();
        }
    }

    pub fn explode(&mut self) {
        if !self.exploding {
            self.exploding = true;
            self.timer = Timer::from_millis(250);
        }
    }

    pub fn dead(&self) -> bool {
        (self.exploding && self.timer.ready) || self.row == 0
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut Frame) {
        frame[self.col][self.row] = if self.exploding { "*" } else { "|" };
    }
}
