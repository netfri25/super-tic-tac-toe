use macroquad::prelude::*;

mod constants;
use constants::PAD;

mod grid;
use grid::{Cell, Grid, Player, GeneralCell};

mod draw;
use draw::{pad_rect, Drawable};

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();
    loop {
        if is_mouse_button_pressed(MouseButton::Left) {
            game.mouse_press()
        }

        let rect = game_rect();

        for i in 0..game.allowed_grids.len() {
            let allowed = game.allowed_grids[i];
            let won = game.grid.cells()[i].winner().is_some();
            if allowed || won { continue }

            let row = i / 3;
            let col = i % 3;
            let w = rect.w / 3.;
            let h = rect.h / 3.;
            let x = rect.x + w * col as f32;
            let y = rect.y + h * row as f32;
            let mut color = RED;
            color.a /= 2.;
            draw_rectangle(x, y, w, h, color);
        }

        game.grid.update_winner();
        if game.grid.winner().is_some() {
            game.allowed_grids = [true; 9]
        }
        game.grid.draw(game_rect());
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

pub struct Game {
    pub grid: Grid<Grid<Cell>>,
    pub allowed_grids: [bool; 9],
    pub turn: Player,
}

impl Game {
    pub fn new() -> Self {
        Game {
            grid: Grid::new(),
            allowed_grids: [true; 9],
            turn: Player::X,
        }
    }

    pub fn mouse_press(&mut self) {
        let rect = game_rect();
        let nw = rect.w;
        let nh = rect.h;
        let (mx, my) = mouse_position();
        let (nmx, nmy) = (mx - rect.x, my - rect.y);

        let index_x = nmx / nw * 3.;
        let index_y = nmy / nh * 3.;
        let grid_index = index_y.floor() * 3. + index_x.floor();

        // check if the user is allowed to press there
        if !self.allowed_grids[grid_index as usize] {
            return;
        }

        // draw_rectangle(
        //     index_x / 3. * nw + rect.x,
        //     index_y / 3. * nh + rect.y,
        //     nw / 3.,
        //     nh / 3.,
        //     GREEN
        // );

        let inner_index_x = nmx * 9. / nw % 3.;
        let inner_index_y = nmy * 9. / nh % 3.;

        // draw_rectangle(
        //     inner_index_x / 9. * nw + rect.x,
        //     inner_index_y / 9. * nh + rect.y,
        //     nw / 9.,
        //     nh / 9.,
        //     RED,
        // );

        let range = 0f32..3f32;
        let indices = [index_x, index_y, inner_index_x, inner_index_y];
        let all_valid = indices.into_iter().all(|i| range.contains(&i.floor()));
        let placed = if all_valid {
            self.grid
                .update_with(index_y as usize, index_x as usize, |inner| {
                    inner.update_with(inner_index_y as usize, inner_index_x as usize, |cell| {
                        let b = cell.is_none();
                        if cell.is_none() {
                            *cell = Some(self.turn)
                        }
                        b
                    })
                })
                .flatten()
                .filter(|b| *b)
                .is_some()
        } else {
            false
        };

        // set all the allowed grids
        if placed {
            self.turn.switch();

            let inner_grid_index = inner_index_y.floor() * 3. + inner_index_x.floor();
            if self.grid.cells()[inner_grid_index as usize].cvalue().is_some() {
                self.allowed_grids = [true; 9];
            } else {
                self.allowed_grids = [false; 9];
                self.allowed_grids[inner_grid_index as usize] = true;
            }
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

pub fn game_rect() -> Rect {
    pad_rect(calc_screen_rect(), PAD)
}
