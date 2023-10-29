use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;

mod draw;
use draw::{Drawable, pad_rect};

mod game;
use game::Game;

mod bot;
use bot::get_best_moves;

mod constants;
mod grid;

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();
    loop {
        let bounds = calc_screen_rect();
        if is_mouse_button_pressed(MouseButton::Left) && game.mouse_press(bounds) {
            let bot_suggestions = get_best_moves(&mut game);
            if let Some(bot_best_move) = bot_suggestions.moves.choose().cloned() {
                game.play(bot_best_move.into_iter());
            }
            game.new_suggestion();
        }

        if is_key_pressed(KeyCode::R) {
            game = Game::new();
            game.new_suggestion();
        }

        if is_key_pressed(KeyCode::Z) {
            game.rewind_step();
            game.new_suggestion();
        }

        if is_key_pressed(KeyCode::S) {
            game.suggest = !game.suggest;
            game.new_suggestion();
        }

        game.draw(bounds, true);
        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Super Tic-Tac-Toe".into(),
        window_width: 1600,
        window_height: 900,
        fullscreen: false,
        window_resizable: true,
        ..Default::default()
    }
}

fn calc_screen_rect() -> Rect {
    let w = screen_width();
    let h = screen_height();
    let size = w.min(h);
    let x = (w - size) / 2.;
    let y = (h - size) / 2.;
    Rect::new(x, y, size, size)
}
