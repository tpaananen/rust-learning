use std::io::Stdout;
use crate::{NUM_COLS, NUM_ROWS};

pub trait Discoverable {
    fn get_col(&self) -> usize;
    fn get_row(&self) -> usize;
    fn show(&self) -> &'static str;
}

#[derive(Debug)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

pub struct Frame {
    frame: Vec<Vec<&'static str>>
}

impl Frame {
    pub fn update_item(&mut self, drawable: &dyn Discoverable) {
        self.frame[drawable.get_col()][drawable.get_row()] = drawable.show();
    }

    pub fn get_value_at(&self, col: usize, row: usize) -> &'static str {
        self.frame[col][row]
    }

    pub fn updade_each_cell<F>(&self, stdout: &mut Stdout, renderer: F) where F: Fn(usize, usize, &str, &mut Stdout) {
        for (col_index, col) in self.frame.iter().enumerate() {
            for (row_index, &current_value) in col.iter().enumerate() {
                renderer(col_index, row_index, current_value, stdout);
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
