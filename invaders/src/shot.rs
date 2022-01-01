use std::time::Duration;

use rusty_time::prelude::Timer;

use crate::frame::{Drawable, Frame, Position, Discoverable};

pub struct Shot {
    position: Position,
    exploding: bool,
    timer: Timer
}

impl Discoverable for Shot {
    fn get_col(&self) -> usize { self.position.col }
    fn get_row(&self) -> usize { self.position.row }
}

impl Shot {
    pub fn new(col: usize, row: usize) -> Self {
        Self { position: Position { col, row }, exploding: false, timer: Timer::from_millis(16) }
    }

    pub fn is_exploding(&self) -> bool { self.exploding }

    pub fn update(&mut self, delta: Duration) {
        self.timer.update(delta);
        if self.timer.ready && !self.exploding {
            if self.position.row > 0 {
                self.position.row -= 1;
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

    pub fn is_dead(&self) -> bool {
        (self.exploding && self.timer.ready) || self.position.row == 0
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut Frame) {
        let value = if self.exploding { "*" } else { "|" };
        frame.set_shot(self, value);
    }
}
