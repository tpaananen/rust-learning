use std::io::{Stdout, Write, stdout};
use std::sync::mpsc::Receiver;
use crossterm::QueueableCommand;
use crossterm::cursor::MoveTo;
use crossterm::style::{SetBackgroundColor, Color};
use crossterm::terminal::{Clear, ClearType};
use crate::frame::Frame;

fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) {
    if force {
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }

    curr_frame.update_each_cell(
        stdout,
        last_frame,
        |col_index, row_index, previous_value, current_value, stdout| {
        if current_value != previous_value || force {
            stdout.queue(MoveTo(col_index as u16, row_index as u16)).unwrap();
            println!("{}", current_value)
        }
    });

    stdout.flush().unwrap();
}

pub fn render_screen(render_rx: Receiver<Frame>, num_rows: usize, num_columns: usize) {
    let mut last_frame = Frame::new(num_rows, num_columns);
    let mut stdout = stdout();
    render(&mut stdout, &last_frame, &last_frame, true);
    while let Ok(curr_frame) = render_rx.recv() {
        render(&mut stdout, &last_frame, &curr_frame, false);
        last_frame = curr_frame;
    }
}
