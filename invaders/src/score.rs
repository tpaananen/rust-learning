use crate::frame::{Frame, Drawable};

pub struct Score {
    invaders_destroyed: usize,
    invaders_count: usize,
    total_shots_used: usize
}

impl Score {
    pub fn new(invaders_count: usize) -> Self {
        Score { invaders_destroyed: 0, invaders_count, total_shots_used: 0 }
    }

    pub fn increment_progress(&mut self, count_hits: usize) {
        self.invaders_destroyed += count_hits;
    }

    pub fn update_shots(&mut self, count_shots: usize) {
        self.total_shots_used = count_shots;
    }
}

impl Drawable for Score {
    fn draw(&self, frame: &mut Frame) {
        let percentage: f32 = self.invaders_destroyed as f32 / self.invaders_count as f32 * 100.0;
        let accuracy: f32 = if self.total_shots_used > 0 {
            self.invaders_destroyed as f32 / self.total_shots_used as f32 * 100.0
        } else {
            100.0
        };
        frame.update_top_row(&format!("PRGRESS: {:.2} % :: ACCURACY {:.2} %", percentage, accuracy));
    }
}
