use macroquad::prelude::*;

mod draw;
use draw::Drawable;

mod game;
use game::Game;

mod constants;
mod grid;

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();
    loop {
        let rect = calc_screen_rect();
        if is_mouse_button_pressed(MouseButton::Left) {
            game.mouse_press(rect)
        }

        game.draw(rect);
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
