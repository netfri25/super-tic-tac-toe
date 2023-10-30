pub mod iter;

use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct Layout {
    area: Rect, // the full area
    count_x: u32,
    count_y: u32,
}

impl Layout {
    pub fn new(area: Rect, count_x: u32, count_y: u32) -> Self {
        Self {
            area,
            count_x,
            count_y,
        }
    }

    pub fn box_size(&self) -> Vec2 {
        let w = self.area.w / self.count_x as f32;
        let h = self.area.h / self.count_y as f32;
        vec2(w, h)
    }

    pub fn at(&self, ix: u32, iy: u32) -> Rect {
        let size = self.box_size();
        let x = self.area.x + size.x * ix as f32;
        let y = self.area.y + size.y * iy as f32;
        Rect::new(x, y, size.x, size.y)
    }

    pub fn count_x(&self) -> u32 {
        self.count_x
    }

    pub fn count_y(&self) -> u32 {
        self.count_y
    }

    pub fn iter(&self) -> iter::LayoutIter {
        iter::LayoutIter::new(self)
    }
}
