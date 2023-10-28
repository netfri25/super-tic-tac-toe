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
    let mut best_moves = get_best_moves(&mut game);
    let mut suggest = false;
    loop {
        let bounds = calc_screen_rect();
        if is_mouse_button_pressed(MouseButton::Left) && game.mouse_press(bounds) {
            let bot_best_moves = get_best_moves(&mut game);
            if let Some(bot_best_move) = bot_best_moves.0.choose().cloned() {
                game.play(bot_best_move.into_iter());
            }
            best_moves = get_best_moves(&mut game)
        }

        if is_key_pressed(KeyCode::R) {
            game = Game::new();
        }

        if is_key_pressed(KeyCode::Z) {
            game.rewind_step();
            best_moves = get_best_moves(&mut game)
        }

        if is_key_pressed(KeyCode::S) {
            suggest = !suggest;
        }

        game.draw(bounds, true);
        if game.grid.winner().is_none() {
            let w = screen_width() / 7.;
            let turn_rect = Rect::new(screen_width() - w - constants::PAD, screen_height() - w - constants::PAD, w, w);
            game.turn.draw(pad_rect(turn_rect, constants::PAD), false);
        }

        if suggest {
            for (i, (suggestion, eval)) in best_moves.0.iter().zip(best_moves.1.iter()).enumerate() {
                draw_best(suggestion.iter().cloned(), bounds);
                let suggestion_text = format!("{:?}: {:.5}", suggestion, eval);
                let font_size = screen_height() / 20.;
                let y = (i + 1) as f32 * font_size;
                draw_text(&suggestion_text, 0., y, font_size, WHITE);
            }
        }
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

fn draw_best(mut best: impl Iterator<Item = usize>, bounds: Rect) {
    let w = bounds.w / 3.;
    let h = bounds.h / 3.;

    if let Some(i) = best.next() {
        let (row, col) = (i / 3, i % 3);
        let (row, col) = (row as f32, col as f32);
        let rect = Rect::new(bounds.x + w * col, bounds.y + h * row, w, h);
        draw_best(best, pad_rect(rect, constants::PAD));
    } else {
        // let bounds = pad_rect(bounds, constants::PAD);
        draw_rectangle(bounds.x, bounds.y, bounds.w, bounds.h, GREEN);
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
