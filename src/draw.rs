use macroquad::prelude::*;

pub trait Drawable {
    fn draw(&self, bounds: Rect);
}

pub trait Paddable {
    // padding is a value between 0..1 that represents the precentage
    fn pad(self, padding: f32) -> Self;
}

impl Paddable for Rect {
    fn pad(mut self, padding: f32) -> Self {
        self.x += padding * self.w / 2.;
        self.y += padding * self.h / 2.;
        self.w *= 1. - padding;
        self.h *= 1. - padding;
        self
    }
}
