use std::{time::Duration, cmp::max};
use rusty_time::prelude::Timer;
use crate::{NUM_COLS, NUM_ROWS, frame::{Frame, Drawable, Discoverable, Position}};

#[derive(PartialEq, Eq, Copy, Clone)]
enum Direction {
    Left = -1,
    Right = 1
}

pub struct Invader {
    position: Position
}

impl Discoverable for Invader {
    fn get_col(&self) -> usize { self.position.col }
    fn get_row(&self) -> usize { self.position.row }
}

pub struct Invaders {
    army: Vec<Invader>,
    move_timer: Timer,
    direction: Direction
}

impl Invaders {
    pub fn new() -> Self {
        let mut army = Vec::new();

        for col in 0..NUM_COLS {
            for row in 0..NUM_ROWS {
                if (col > 1) && (col < NUM_COLS - 2) && (row > 0) && (row < NUM_ROWS / 2) && (col % 2 == 0) && (row % 2 == 0) {
                    army.push(Invader { position: Position { col, row }});
                }
            }
        }

        Self { army, move_timer: Timer::from_millis(2000), direction: Direction::Right}
    }

    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.update(delta);
        if self.move_timer.ready {
            self.move_timer.reset();
            let should_move_downwards = self.set_horizontal_direction();
            self.update_invader_positions(should_move_downwards);
            true
        } else {
            false
        }
    }

    fn set_horizontal_direction(&mut self) -> bool {
        let mut downwards = false;
        if self.direction == Direction::Left {
            let min_col = self.army.iter().map(|invader| invader.get_col()).min().unwrap_or(0);
            if min_col == 0 {
                self.direction = Direction::Right;
                downwards = true;
            }
        } else {
            let max_col = self.army.iter().map(|invader| invader.get_col()).max().unwrap_or(0);
            if max_col == NUM_COLS - 2 {
                self.direction = Direction::Left;
                downwards = true
            }
        }
        downwards
    }

    fn update_invader_positions(&mut self, should_move_downwards: bool) {
        if should_move_downwards {
            let new_duration = max(self.move_timer.duration.as_millis() - 250, 250);
            self.move_timer = Timer::from_millis(new_duration as u64);
            for invader in self.army.iter_mut() {
                if invader.position.row < NUM_ROWS - 1 {
                    invader.position.row += 1;
                }
            }
        } else {
            for invader in self.army.iter_mut() {
                invader.position.col = ((invader.get_col() as i32) + self.direction as i32) as usize;
            }
        }
    }

    pub fn all_killed(&self) -> bool {
        self.army.is_empty()
    }

    pub fn reached_bottom(&self) -> bool {
        self.army.iter().map(|invader| invader.get_row()).max().unwrap_or(0) >= NUM_ROWS - 1
    }

    pub fn kill_invader_at(&mut self, row: usize, col: usize) -> bool {
        if let Some(idx) = self
            .army
            .iter()
            .position(|invader| (invader.get_col() == col) && (invader.get_row() == row)) {
                self.army.remove(idx);
                true
            } else {
                false
            }
    }
}

impl Drawable for Invaders {
    fn draw(&self, frame: &mut Frame) {
        for invader in self.army.iter() {
            let value = self.move_timer.time_left.as_secs_f32() / self.move_timer.duration.as_secs_f32();
            let look_and_feel = if value > 0.5 { "x" } else { "+" };
            frame.set_invader(invader, look_and_feel);
        }
    }
}
