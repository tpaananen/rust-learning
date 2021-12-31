use std::{time::{Duration, Instant}, sync::mpsc::Sender, thread};
use crossterm::event::{self, Event};
use crate::{sounds::GameAudio, frame::{Frame, new_frame, Drawable}, player::Player, invaders::Invaders};

pub fn game_loop(audio: &mut GameAudio, render_tx: &Sender<Frame>) {
    let mut instant = Instant::now();
    let mut player = Player::new();
    let mut invaders = Invaders::new();

    'gameloop: loop {

        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();

        // Input
        if process_player_input(&mut player, audio) {
            break 'gameloop;
        }

        // Updates
        player.update(delta);
        if invaders.update(delta) {
            audio.moving();
        }

        if player.detect_hits(&mut invaders) {
            audio.explode();
        }

        // Draw & render
        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawables.iter() {
            drawable.draw(&mut curr_frame);
        }

        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));

        // Win or lose?
        if invaders.all_killed() {
            audio.win();
            println!("Player wins!");
            break 'gameloop;
        } else if invaders.reached_bottom() {
            audio.lose();
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
                        audio.pew();
                    }
                },
                event::KeyCode::Esc => {
                    audio.lose();
                    return true;
                },
                _ => {}
            }
        }
    }

    false
}
