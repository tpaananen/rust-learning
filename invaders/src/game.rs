use std::{time::{Duration, Instant}, sync::mpsc::Sender, thread};
use crossterm::event::{self, Event};
use crate::{sounds::GameAudio, frame::{Frame, Drawable}, player::Player, invaders::Invaders};

pub fn game_loop(audio: &mut GameAudio, render_tx: &Sender<Frame>) {
    let mut instant = Instant::now();
    let mut player = Player::new();
    let mut invaders = Invaders::new();

    'gameloop: loop {

        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = Frame::new();

        // Input
        if process_player_input(&mut player, audio) {
            break 'gameloop;
        }

        // Updates
        player.update(delta);
        if invaders.update(delta) {
            audio.play_moving();
        }

        if player.detect_hits(&mut invaders) {
            audio.play_explode();
        }

        // Draw & render
        player.draw(&mut curr_frame);
        invaders.draw(&mut curr_frame);

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
