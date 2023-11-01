use macroquad::prelude::*;

mod bot;
mod layout;

mod grid;
use grid::{Grid, padded_grid, Index};

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

struct History {
    only_allowed: Option<Index>,
    indices: (u8, u8),
}

struct App {
    grid: Grid,
    turn: Player,
    history: Vec<History>,
}

impl App {
    pub fn handle_input(&mut self) -> bool {
        if is_mouse_button_pressed(MouseButton::Left) {
            self.mouse_press();
        }

        if is_key_pressed(KeyCode::Escape) {
            return false;
        }

        if is_key_pressed(KeyCode::Z) {
            self.undo()
        }

        true
    }

    pub fn mouse_press(&mut self) {
        let Some(indices) = self.get_mouse_indices() else {
            return;
        };

        let only_allowed = self.grid.only_allowed();
        let played = self.grid.play(self.turn, indices.0, indices.1);
        if !played {
            return;
        }

        self.history.push(History {
            only_allowed,
            indices,
        });

        self.turn = self.turn.other();

        // TODO: abstract away the bot play
        let best_indices = bot::best_indices(self.grid.clone(), self.turn);
        let Some((indices, eval)) = best_indices else {
            eprintln!("no moves?");
            return
        };

        println!("best eval: {}", eval);

        let played = self.grid.play(self.turn, indices.0, indices.1);
        if !played {
            return
        }

        self.history.push(History {
            only_allowed,
            indices,
        });

        self.turn = self.turn.other();
    }

    pub fn undo(&mut self) {
        let Some(History { only_allowed, indices }) = self.history.pop() else {
            return;
        };

        self.grid.unplay(indices.0, indices.1, only_allowed);
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
            history: Vec::new(),
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
