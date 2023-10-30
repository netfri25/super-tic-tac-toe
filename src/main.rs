use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(BLACK);

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
