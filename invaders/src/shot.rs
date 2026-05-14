use crate::frame::{Discoverable, Drawable, Frame, Position};
use rusty_time::Timer;
use std::time::Duration;

pub struct Shot {
    position: Position,
    exploding: bool,
    timer: Timer,
}

impl Discoverable for Shot {
    fn get_col(&self) -> usize {
        self.position.col
    }
    fn get_row(&self) -> usize {
        self.position.row
    }
    fn show(&self) -> char {
        if self.exploding {
            '*'
        } else {
            '|'
        }
    }
}

impl Shot {
    pub fn new(col: usize, row: usize) -> Self {
        Self {
            position: Position { col, row },
            exploding: false,
            timer: Timer::new(Duration::from_millis(16)),
        }
    }

    pub fn is_exploding(&self) -> bool {
        self.exploding
    }

    pub fn update(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if self.timer.just_finished() && !self.exploding {
            if self.position.row > 0 {
                self.position.row -= 1;
            }
            self.timer.reset();
        }
    }

    pub fn explode(&mut self) {
        if !self.exploding {
            self.exploding = true;
            self.timer = Timer::new(Duration::from_millis(250));
        }
    }

    pub fn is_dead(&self) -> bool {
        (self.exploding && self.timer.finished()) || self.position.row == 0
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut Frame) {
        frame.update_item(self);
    }
}
