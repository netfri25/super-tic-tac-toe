use macroquad::prelude::*;

pub const PADDING: f32 = 0.08;
pub const THICK_MULT: f32 = 0.01;
pub const SHAPE_THICK: f32 = 0.1;

pub const X_COLOR: Color = RED;
pub const O_COLOR: Color = BLUE;
pub const GRID_LINES_COLOR: Color = WHITE;
pub const BLOCKED_COLOR: Color = {
    let mut color = WHITE;
    color.a /= 4.;
    color
};
