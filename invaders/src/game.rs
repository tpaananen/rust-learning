use std::{time::{Duration, Instant}, sync::mpsc::Sender, thread};
use crossterm::event::{self, Event};
use crate::{sounds::GameAudio, frame::{Frame, Drawable}, player::Player, invaders::Invaders, score::Score};

pub fn game_loop(
    audio: &mut GameAudio,
    render_tx: &Sender<Frame>,
    num_rows: usize,
    num_columns: usize,
    num_shots: usize) {

    let mut instant = Instant::now();
    let mut player = Player::new(num_rows, num_columns, num_shots);
    let mut invaders = Invaders::new(num_rows, num_columns);
    let mut score = Score::new(invaders.count());

    'gameloop: loop {

        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = Frame::new(num_rows, num_columns);

        // Input
        if process_player_input(&mut player, audio) {
            break 'gameloop;
        }

        // Updates
        player.update(delta);
        if invaders.update(delta) {
            audio.play_moving();
        }

        let count_hits = player.detect_hits(&mut invaders);
        if count_hits > 0 {
            score.increment(count_hits);
            audio.play_explode();
        }

        // Draw & render
        player.draw(&mut curr_frame);
        invaders.draw(&mut curr_frame);
        score.draw(&mut curr_frame);

        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));

        // Win or lose?
        if invaders.all_killed() {
            audio.play_win();
            println!("Player wins!");
            break 'gameloop;
        } else if invaders.reached_bottom() {
            audio.play_lose();
            println!("Player lose!");
            break 'gameloop;
        }
    }
}

fn process_player_input(player: &mut Player, audio: &mut GameAudio) -> bool {
    while event::poll(Duration::default()).unwrap() {
        if let Event::Key(key_event) = event::read().unwrap() {
            match key_event.code {
                event::KeyCode::Left => player.move_left(),
                event::KeyCode::Right => player.move_right(),
                event::KeyCode::Up => {
                    if player.shoot() {
                        audio.play_pew();
                    }
                },
                event::KeyCode::Esc => {
                    audio.play_lose();
                    return true;
                },
                _ => {}
            }
        }
    }

    false
}
