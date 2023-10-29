use macroquad::prelude::*;

pub const PAD: f32 = 0.07;
pub const X_COLOR: Color = RED;
pub const O_COLOR: Color = BLUE;
pub const CELL_THICK: f32 = 0.1;
pub const GRID_THICK: f32 = 0.01;
pub const GRID_COLOR: Color = WHITE;
pub const BLOCKED_COLOR: Color = {
    let mut color = WHITE;
    color.a /= 4.;
    color
};

pub const MAX_SEARCH_DEPTH: usize = 8;

pub const INDICES_ORDER: [usize; 9] = [4, 0, 2, 6, 8, 1, 3, 5, 7];

pub const EVALUATION_MIN: i32 = i16::MIN as i32;
pub const EVALUATION_MAX: i32 = i16::MAX as i32;
