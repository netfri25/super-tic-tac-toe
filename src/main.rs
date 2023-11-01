use bot::Eval;
use itertools::Itertools;
use macroquad::prelude::*;

mod bot;
mod layout;
mod player;

mod grid;
use grid::padded_grid;

mod draw;
use draw::{Drawable, Paddable};

mod utils;
use utils::{Indices, PADDING};

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
    suggest: Option<Vec<(Indices, Eval)>>,
}

impl App {
    pub fn handle_input(&mut self) -> bool {
        if is_mouse_button_pressed(MouseButton::Left) {
            self.mouse_press();
            self.new_suggestions();
        }

        if is_key_pressed(KeyCode::Escape) {
            return false;
        }

        if is_key_pressed(KeyCode::Z) {
            self.game.undo();
            self.new_suggestions();
        }

        if is_key_pressed(KeyCode::S) {
            self.suggest = match self.suggest {
                Some(_) => None,
                None => Some(Vec::new()),
            };
            self.new_suggestions();
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

        // if let Some((indices, eval)) =
        //     bot::best_indices(self.game.clone_grid(), self.game.turn()).pop()
        // {
        //     println!("best eval: {}", eval);
        //     if !self.game.play(indices) {
        //         panic!("unable to play {:?}", indices);
        //     }
        // } else {
        //     println!("no moves?");
        // };
    }

    fn new_suggestions(&mut self) {
        if let Some(ref mut v) = self.suggest {
            *v = bot::best_indices(self.game.clone_grid(), self.game.turn());
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self { game: Game::new(), suggest: None }
    }
}

impl Drawable for App {
    fn draw(&self, bounds: Rect) {
        let bounds = bounds.pad(PADDING / 2.);
        self.game.draw(bounds);

        if self.game.finished() {
            return;
        }

        const COLOR: Color = {
            let mut color = GREEN;
            color.a /= 1.5;
            color
        };

        let cells = get_cells().collect_vec();
        let font_size = screen_height() / 50.;
        if let Some(ref suggest) = self.suggest {
            for (i, &(indices, eval)) in suggest.iter().enumerate() {
                let text = format!("{:?}: {}", indices, eval);
                let row = i as f32 * font_size + font_size;
                draw_text(&text, 0., row, font_size, WHITE);

                let r = cells.iter().find(|(is, _)| *is == indices).map(|(_, r)| r).unwrap();
                let r = r.pad(3. * PADDING);
                draw_rectangle(r.x, r.y, r.w, r.h, COLOR);
            }
        }
    }
}

fn get_mouse_indices() -> Option<(u8, u8)> {
    let mpos = mouse_position().into();
    get_cells().find(|(_, r)| r.contains(mpos)).map(|(i, _)| i)
}

pub fn get_screen_rect() -> Rect {
    let size = screen_width().min(screen_height());
    let x = (screen_width() - size) / 2.;
    let y = (screen_height() - size) / 2.;
    Rect::new(x, y, size, size)
}

pub fn get_cells() -> impl Iterator<Item = (Indices, Rect)> {
    padded_grid(get_screen_rect().pad(PADDING / 2.), PADDING)
        .enumerate()
        .flat_map(|(i, outer)| {
            padded_grid(outer, PADDING)
                .enumerate()
                .map(move |(j, r)| ((i as u8, j as u8), r))
        })
}
