use macroquad::prelude::*;

mod layout;

mod grid;
use grid::{Grid, padded_grid};

mod draw;
use draw::{Paddable, Drawable};

mod player;
use player::Player;

mod utils;
use utils::PADDING;

#[macroquad::main(window_conf)]
async fn main() {
    let mut app = App::default();
    let mut keep_running = true;
    while keep_running {
        keep_running = app.handle_input();
        clear_background(BLACK);
        app.draw(get_screen_rect());
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

struct App {
    grid: Grid,
    turn: Player,
}

impl App {
    pub fn handle_input(&mut self) -> bool {
        if is_mouse_button_pressed(MouseButton::Left) {
            self.mouse_press();
        }

        if is_key_pressed(KeyCode::Escape) {
            return false;
        }

        true
    }

    pub fn mouse_press(&mut self) {
        let Some(indices) = self.get_mouse_indices() else {
            return;
        };

        let played = self.grid.play(self.turn, indices.0, indices.1);
        if !played {
            return;
        }

        self.turn = self.turn.other();
    }

    pub fn get_mouse_indices(&self) -> Option<(u8, u8)> {
        let bounds = get_screen_rect().pad(PADDING);
        let mpos = mouse_position().into();

        let (outer_index, inner_grid) = padded_grid(bounds, PADDING).enumerate().find(|(_, r)| r.contains(mpos))?;
        let inner_index = padded_grid(inner_grid, PADDING).position(|r| r.contains(mpos))?;

        Some((outer_index as u8, inner_index as u8))
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            grid: Default::default(),
            turn: Player::X,
        }
    }
}

impl Drawable for App {
    fn draw(&self, bounds: Rect) {
        self.grid.draw(bounds.pad(PADDING / 2.));
    }
}

pub fn get_screen_rect() -> Rect {
    let size = screen_width().min(screen_height());
    let x = (screen_width() - size) / 2.;
    let y = (screen_height() - size) / 2.;
    Rect::new(x, y, size, size)
}
