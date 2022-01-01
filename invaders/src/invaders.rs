use std::{time::Duration, cmp::max};
use rusty_time::prelude::Timer;
use crate::frame::{Frame, Drawable, Discoverable, Position};

#[derive(PartialEq, Eq, Copy, Clone)]
enum Direction {
    Left = -1,
    Right = 1
}

pub struct Invader {
    position: Position,
    how_i_look: char
}

impl Discoverable for Invader {
    fn get_col(&self) -> usize { self.position.col }
    fn get_row(&self) -> usize { self.position.row }
    fn show(&self) -> char { self.how_i_look }
}

pub struct Invaders {
    army: Vec<Invader>,
    move_timer: Timer,
    direction: Direction,
    num_rows: usize,
    num_columns: usize
}

impl Invaders {
    pub fn new(num_rows: usize, num_columns: usize) -> Self {
        let mut army = Vec::new();

        for col in 0..num_columns {
            for row in 0..num_rows {
                if (col > 1) && (col < num_columns - 2) && (row > 0) && (row < num_rows / 2) && (col % 2 == 0) && (row % 2 == 0) {
                    army.push(Invader { position: Position { col, row }, how_i_look: 'X' });
                }
            }
        }

        Self {
            army,
            move_timer: Timer::from_millis(2000),
            direction: Direction::Right,
            num_rows,
            num_columns
        }
    }

    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.update(delta);
        self.update_invader_look();
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
            if max_col == self.num_columns - 2 {
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
                if invader.position.row < self.num_rows - 1 {
                    invader.position.row += 1;
                }
            }
        } else {
            for invader in self.army.iter_mut() {
                invader.position.col = ((invader.get_col() as i32) + self.direction as i32) as usize;
            }
        }
    }

    fn update_invader_look(&mut self) {
        for invader in self.army.iter_mut() {
            let value = self.move_timer.time_left.as_secs_f32() / self.move_timer.duration.as_secs_f32();
            let look = if value > 0.5 { 'X' } else { '+' };
            invader.how_i_look = look;
        }
    }

    pub fn all_killed(&self) -> bool {
        self.army.is_empty()
    }

    pub fn reached_bottom(&self) -> bool {
        self.army.iter().map(|invader| invader.get_row()).max().unwrap_or(0) >= self.num_rows - 1
    }

    pub fn kill_invader_at(&mut self, item: &dyn Discoverable) -> bool {
        for (index, invader) in self.army.iter().enumerate() {
            if invader.is_at_same_position_as(item) {
                self.army.remove(index);
                return true;
            }
        }
        false
    }

    pub fn count(&self) -> usize {
        self.army.len()
    }
}

impl Drawable for Invaders {
    fn draw(&self, frame: &mut Frame) {
        for invader in self.army.iter() {
            frame.update_item(invader);
        }
    }
}
