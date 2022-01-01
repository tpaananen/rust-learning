use crate::frame::{Frame, Drawable};

pub struct Score {
    count: usize,
    invaders_count: usize
}

impl Score {
    pub fn new(invaders_count: usize) -> Self {
        Score { count: 0, invaders_count }
    }

    pub fn increment(&mut self, count_hits: usize) {
        self.count += count_hits;
    }
}

impl Drawable for Score {
    fn draw(&self, frame: &mut Frame) {
        let formatted = format!("SCORE: {:0>2} / {:0>2}", self.count, self.invaders_count);
        frame.update_row(0, &formatted);
    }
}
