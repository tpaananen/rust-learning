use std::io::Stdout;

use crate::{NUM_COLS, NUM_ROWS, invaders::Invader, player::Player, shot::Shot};

pub trait Discoverable {
    fn get_col(&self) -> usize;
    fn get_row(&self) -> usize;
}

pub struct Position {
    pub col: usize,
    pub row: usize,
}

impl Discoverable for Position {
    fn get_col(&self) -> usize { self.col }
    fn get_row(&self) -> usize { self.row }
}

pub struct Frame {
    frame: Vec<Vec<&'static str>>
}

impl Frame {
    pub fn set_invader(&mut self, invader: &Invader, value: &'static str) {
        self.frame[invader.get_col()][invader.get_row()] = value;
    }

    pub fn set_player(&mut self, player: &Player) {
        self.frame[player.get_col()][player.get_row()] = "A";
    }

    pub fn set_shot(&mut self, shot: &Shot, value: &'static str) {
        self.frame[shot.col][shot.row] = value;
    }

    pub fn get_value_at(&self, col: usize, row: usize) -> &'static str {
        self.frame[col][row]
    }

    pub fn updade_each_cell<F>(&self, stdout: &mut Stdout, update: F) where F: Fn(usize, usize, &str, &mut Stdout) {
        for (col_index, col) in self.frame.iter().enumerate() {
            for (row_index, &current_value) in col.iter().enumerate() {
                update(col_index, row_index, current_value, stdout);
            }
        }
    }
}

pub fn new_frame() -> Frame {
    let mut frame = Vec::with_capacity(NUM_COLS);
    for _ in 0..NUM_COLS {
        let mut col = Vec::with_capacity(NUM_ROWS);
        for _ in 0..NUM_ROWS {
            col.push(" ");
        }
        frame.push(col);
    }

    Frame { frame }
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
