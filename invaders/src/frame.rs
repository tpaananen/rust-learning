use std::io::Stdout;
use array2d::Array2D;

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
    frame: Array2D<&'static str>
}

impl Frame {
    pub fn new(num_rows: usize, num_columns: usize) -> Self {
        Frame { frame: Array2D::from_row_major(&vec![" "; num_rows * num_columns], num_rows, num_columns) }
    }

    pub fn update_item(&mut self, drawable: &dyn Discoverable) {
        self.frame.set(drawable.get_row(), drawable.get_col(), drawable.show()).unwrap();
    }

    pub fn get_value_at(&self, column: usize, row: usize) -> &'static str {
        match self.frame.get(row, column) {
            Some(val) => val,
            None => " "
        }
    }

    pub fn updade_each_cell<F>(&self, stdout: &mut Stdout, renderer: F) where F: Fn(usize, usize, &str, &mut Stdout) {
        for (row_index, row_iter) in self.frame.rows_iter().enumerate() {
            for (col_index, current_value) in row_iter.enumerate() {
                renderer(col_index, row_index, current_value, stdout);
            }
        }
    }
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
