use macroquad::prelude::*;

mod bot;
mod layout;
mod player;

mod grid;
use grid::padded_grid;

mod draw;
use draw::{Paddable, Drawable};

mod utils;
use utils::PADDING;

mod game;
use game::Game;

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
    game: Game,
}


impl App {
    pub fn handle_input(&mut self) -> bool {
        if is_mouse_button_pressed(MouseButton::Left) {
            self.mouse_press()
        }

        if is_key_pressed(KeyCode::Escape) {
            return false;
        }

        if is_key_pressed(KeyCode::Z) {
            self.game.undo()
        }

        true
    }

    pub fn mouse_press(&mut self) {
        let Some(indices) = get_mouse_indices() else {
            return;
        };

        if !self.game.play(indices) {
            return;
        }

        if let Some((indices, eval)) = bot::best_indices(self.game.clone_grid(), self.game.turn()).pop() {
            println!("best eval: {}", eval);
            if !self.game.play(indices) {
                panic!("unable to play {:?}", indices);
            }
        } else {
            println!("no moves?");
        };
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            game: Game::new()
        }
    }
}

impl Drawable for App {
    fn draw(&self, bounds: Rect) {
        let bounds = bounds.pad(PADDING / 2.);
        self.game.draw(bounds);
    }
}

fn get_mouse_indices() -> Option<(u8, u8)> {
    let bounds = get_screen_rect().pad(PADDING);
    let mpos = mouse_position().into();

    let (outer_index, inner_grid) = padded_grid(bounds, PADDING).enumerate().find(|(_, r)| r.contains(mpos))?;
    let inner_index = padded_grid(inner_grid, PADDING).position(|r| r.contains(mpos))?;

    Some((outer_index as u8, inner_index as u8))
}

pub fn get_screen_rect() -> Rect {
    let size = screen_width().min(screen_height());
    let x = (screen_width() - size) / 2.;
    let y = (screen_height() - size) / 2.;
    Rect::new(x, y, size, size)
}
