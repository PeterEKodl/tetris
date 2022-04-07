use game::actions::Rotation;
use game::actions::Shift;
use render::Render;
use rustbox::*;
use std::thread::sleep;
use std::time::{Duration, Instant};

mod game;
mod render;

const FPS: f32 = 30.0;
const CONTROLS:&str = "Left/Right: Shift tetromino | Down: Hard drop | Z/X: Rotate tetromino | ESC: Pause/Continue | Q: Quit";

fn main() {
    let rustbox = RustBox::init(InitOptions::default()).expect("Failed to initalize terminal.");
    rustbox.set_input_mode(InputMode::EscMouse);
    let mut game = game::Game::new(10, 24);

    let mut now = Instant::now();

    #[cfg(debug_assertions)]
    let mut fps: f32 = 0.0;

    loop {
        rustbox.clear();
        #[cfg(debug_assertions)]
        draw(&rustbox, &game, fps);

        #[cfg(not(debug_assertions))]
        draw(&rustbox, &game);

        rustbox.present();

        if let Event::KeyEvent(key) = rustbox
            .peek_event(Duration::new(0, 1_000_000), false)
            .unwrap()
        {
            match key {
                Key::Esc => {
                    if pause_and_check_quit(&rustbox, &game) {
                        break;
                    } else {
                        now = Instant::now();
                        continue;
                    }
                }
                Key::Char('q') => break,
                Key::Char('z') => game.rotate(Rotation::CounterClockwise),
                Key::Char('x') => game.rotate(Rotation::Clockwise),
                Key::Left => game.shift(Shift::Left),
                Key::Right => game.shift(Shift::Right),
                Key::Down => game.shift(Shift::Down),
                _ => {}
            }
        }
        let new_now = Instant::now();
        let delta = new_now - now;
        now = new_now;
        #[cfg(debug_assertions)]
        {
            fps = 1.0 / delta.as_secs_f32();
        }
        sleep(Duration::from_millis(((1.0 / FPS) * 1000.0) as u64).saturating_sub(delta));
        game.update(delta);
        if game.game_over {
            break;
        }
    }
    // Dropping rustbox sets the terminal back into its original state.
    std::mem::drop(rustbox);
    println!("Score: {}", game.score);
}

#[cfg(debug_assertions)]
fn draw(rustbox: &RustBox, game: &game::Game, fps: f32) {
    game.render(rustbox);
    rustbox.print(
        0,
        rustbox.height() - 1,
        RB_REVERSE,
        Color::White,
        Color::Default,
        CONTROLS,
    );
    rustbox.print(
        0,
        0,
        RB_NORMAL,
        Color::White,
        Color::Default,
        &format!("FPS: {}", fps as u32),
    );
}

#[cfg(not(debug_assertions))]
fn draw(rustbox: &RustBox, game: &game::Game) {
    game.render(&rustbox);
    rustbox.print(
        0,
        rustbox.height() - 1,
        RB_REVERSE,
        Color::White,
        Color::Default,
        CONTROLS,
    );
}

fn pause_and_check_quit(rustbox: &RustBox, game: &game::Game) -> bool {
    loop {
        rustbox.clear();

        #[cfg(debug_assertions)]
        draw(rustbox, game, 0.0);

        #[cfg(not(debug_assertions))]
        draw(rustbox, game);

        rustbox.print(
            (rustbox.width() - 5) / 2,
            0,
            RB_BOLD,
            Color::White,
            Color::Red,
            "PAUSE",
        );
        rustbox.present();

        match rustbox.poll_event(false).unwrap() {
            Event::KeyEvent(Key::Char('q')) => return true,
            Event::KeyEvent(Key::Esc) => return false,
            _ => {}
        }
    }
}
