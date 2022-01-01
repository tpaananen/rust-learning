use std::{
    io,
    error::Error,
    sync::mpsc,
    thread,
};
use crossterm::{
    cursor::{Hide, Show},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use invaders::{sounds::GameAudio, render::render_screen, game::game_loop};

fn main() -> Result<(), Box<dyn Error>> {
    // Setup audio
    let mut audio = GameAudio::new();
    audio.play_startup();

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render loop
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        render_screen(render_rx);
    });

    // Game loop
    game_loop(&mut audio, &render_tx);

    // Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
