use macroquad::prelude::*;

use crate::constants;
use crate::game::Game;
use crate::grid::{GeneralCell, Grid, Player, Cell};

pub trait Drawable {
    fn draw(&self, bounds: Rect, highlight_mouse: bool);
}

// pad is a value between 0..1
pub fn pad_rect(mut rect: Rect, pad: f32) -> Rect {
    let pad_w_px = pad * rect.w;
    rect.w -= 2. * pad_w_px;
    rect.x += pad_w_px;

    let pad_h_px = pad * rect.h;
    rect.h -= 2. * pad_h_px;
    rect.y += pad_h_px;

    rect
}

impl Drawable for Player {
    fn draw(&self, bounds: Rect, _highlight_mouse: bool) {
        let bounds = pad_rect(bounds, constants::PAD);
        match self {
            Player::X => draw_x(bounds),
            Player::O => draw_o(bounds),
        }
    }
}

impl Drawable for Cell {
    fn draw(&self, bounds: Rect, highlight_mouse: bool) {
        if let Some(inner) = self {
            inner.draw(bounds, false);
        } else if highlight_mouse {
            let mouse_pos = mouse_position();
            if bounds.contains(mouse_pos.into()) {
                let mut color = WHITE;
                color.a /= 2.;
                draw_rectangle(bounds.x, bounds.y, bounds.w, bounds.h, color);
            }
        }
    }
}

fn draw_x(bounds: Rect) {
    let x1 = bounds.x;
    let y1 = bounds.y;
    let x2 = bounds.x + bounds.w;
    let y2 = bounds.y + bounds.h;
    let size = bounds.w;
    draw_line(
        x1,
        y1,
        x2,
        y2,
        size * constants::CELL_THICK,
        constants::X_COLOR,
    );

    let x1 = bounds.x + bounds.w;
    let y1 = bounds.y;
    let x2 = bounds.x;
    let y2 = bounds.y + bounds.h;
    let size = bounds.w;
    draw_line(
        x1,
        y1,
        x2,
        y2,
        size * constants::CELL_THICK,
        constants::X_COLOR,
    );
}

fn draw_o(bounds: Rect) {
    let center = bounds.center();
    let radius = bounds.w / 2.;
    let size = bounds.w;
    draw_circle_lines(
        center.x,
        center.y,
        radius,
        size * constants::CELL_THICK,
        constants::O_COLOR,
    );
}

impl<C> Drawable for Grid<C>
where
    C: Drawable + GeneralCell,
{
    fn draw(&self, bounds: Rect, highlight_mouse: bool) {
        if let Some(player) = self.winner() {
            player.draw(bounds, false);
            return;
        }

        let size = bounds.w;
        let grid_thick = size * constants::GRID_THICK;
        let w = bounds.w / 3.;
        let h = bounds.h / 3.;
        let pad_x = w * constants::PAD / 2.;
        let pad_y = h * constants::PAD / 2.;

        for i in 1..3 {
            let i = i as f32;

            let x = bounds.x + i * w;
            let y1 = bounds.y + pad_y;
            let y2 = bounds.y + bounds.h - pad_y;
            draw_line(x, y1, x, y2, grid_thick, constants::GRID_COLOR);

            let y = bounds.y + i * h;
            let x1 = bounds.x + pad_x;
            let x2 = bounds.x + bounds.w - pad_x;
            draw_line(x1, y, x2, y, grid_thick, constants::GRID_COLOR);
        }

        for (i, cell) in self.cells().iter().enumerate() {
            let (row, col) = (i / 3, i % 3);
            let (row, col) = (row as f32, col as f32);
            let rect = Rect::new(bounds.x + w * col, bounds.y + h * row, w, h);
            cell.draw(pad_rect(rect, constants::PAD), highlight_mouse && self.allowed(i));
        }

        for i in 0..9 {
            let allowed = self.allowed(i);
            let won = self.cells()[i].value().is_some();
            if allowed || won {
                continue;
            }

            let row = i / 3;
            let col = i % 3;
            let w = bounds.w / 3.;
            let h = bounds.h / 3.;
            let x = bounds.x + w * col as f32;
            let y = bounds.y + h * row as f32;
            let Rect { x, y, w, h } = pad_rect(Rect::new(x, y, w, h), constants::PAD / 2.);
            draw_rectangle(x, y, w, h, constants::BLOCKED_COLOR);
        }
    }
}

impl Drawable for Game {
    fn draw(&self, bounds: Rect, highlight_mouse: bool) {
        self.grid.draw(bounds, highlight_mouse);
        if let Some(suggestion) = self.suggestion.as_ref().filter(|_| self.suggest) {
            for (i, (suggestion, eval)) in suggestion.moves.iter().zip(suggestion.evals.iter()).enumerate() {
                draw_best(suggestion.iter().cloned(), bounds);
                let suggestion_text = format!("{:?}: {:.5}", suggestion, eval);
                let font_size = screen_height() / 20.;
                let y = (i + 1) as f32 * font_size;
                draw_text(&suggestion_text, 0., y, font_size, WHITE);
            }
        }

        if self.grid.winner().is_none() {
            let w = screen_width() / 7.;
            let turn_rect = Rect::new(screen_width() - w - constants::PAD, screen_height() - w - constants::PAD, w, w);
            self.turn.draw(pad_rect(turn_rect, constants::PAD), false);
        }
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
