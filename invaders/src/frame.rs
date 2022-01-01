use std::io::Stdout;
use array2d::Array2D;

pub trait Discoverable {
    fn get_col(&self) -> usize;
    fn get_row(&self) -> usize;
    fn show(&self) -> char;
    fn is_at_same_position_as(&self, other: &dyn Discoverable) -> bool {
        self.get_col() == other.get_col() && self.get_row() == other.get_row()
    }
}

#[derive(Debug)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

pub struct Frame {
    frame: Array2D<char>
}

impl Frame {
    pub fn new(num_rows: usize, num_columns: usize) -> Self {
        Frame { frame: Array2D::from_row_major(&vec![' '; num_rows * num_columns], num_rows, num_columns) }
    }

    pub fn update_item(&mut self, drawable: &dyn Discoverable) {
        self.frame.set(drawable.get_row(), drawable.get_col(), drawable.show()).unwrap();
    }

    pub fn update_row(&mut self, row: usize, val: &String) {
        for (col, c) in val.chars().enumerate() {
            self.frame.set(row, col, c).unwrap();
        }
    }

    pub fn get_value_at(&self, column: usize, row: usize) -> &char {
        match self.frame.get(row, column) {
            Some(val) => val,
            None => &' '
        }
    }

    pub fn update_each_cell<F>(&self, stdout: &mut Stdout, prev_frame: &Frame, renderer: F)
        where F: Fn(usize, usize, &char, &char, &mut Stdout) {
        for (row_index, row_iter) in self.frame.rows_iter().enumerate() {
            for (col_index, current_value) in row_iter.enumerate() {
                let previous_value = prev_frame.get_value_at(col_index, row_index);
                renderer(col_index, row_index, previous_value, current_value, stdout);
            }
        }
    }
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
