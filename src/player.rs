use macroquad::prelude::*;

use crate::utils;

use crate::{
    draw::{Drawable, Paddable},
    utils::PADDING,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn other(&self) -> Self {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

impl Drawable for Player {
    fn draw(&self, bounds: Rect) {
        let bounds = bounds.pad(PADDING);
        match self {
            Player::X => draw_x(bounds),
            Player::O => draw_o(bounds),
        }
    }
}

fn draw_x(bounds: Rect) {
    let x1 = bounds.x;
    let y1 = bounds.y;
    let x2 = bounds.x + bounds.w;
    let y2 = bounds.y + bounds.h;
    let size = bounds.w;
    draw_line(x1, y1, x2, y2, size * utils::SHAPE_THICK, utils::X_COLOR);

    let x1 = bounds.x + bounds.w;
    let y1 = bounds.y;
    let x2 = bounds.x;
    let y2 = bounds.y + bounds.h;
    let size = bounds.w;
    draw_line(x1, y1, x2, y2, size * utils::SHAPE_THICK, utils::X_COLOR);
}

fn draw_o(bounds: Rect) {
    let center = bounds.center();
    let radius = bounds.w / 2.;
    let size = bounds.w;
    draw_circle_lines(
        center.x,
        center.y,
        radius,
        size * utils::SHAPE_THICK,
        utils::O_COLOR,
    );
}
