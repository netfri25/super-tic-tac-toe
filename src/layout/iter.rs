use macroquad::prelude::*;

use super::Layout;

impl IntoIterator for Layout {
    type Item = Rect;
    type IntoIter = LayoutIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        LayoutIntoIter::new(self)
    }
}

pub struct LayoutIter<'a> {
    grid_box: &'a Layout,
    x: u32,
    y: u32,
}

impl<'a> LayoutIter<'a> {
    pub fn new(grid_box: &'a Layout) -> Self {
        Self {
            grid_box,
            x: 0,
            y: 0,
        }
    }
}

impl<'a> Iterator for LayoutIter<'a> {
    type Item = Rect;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.grid_box.count_y() {
            return None;
        }

        if self.x >= self.grid_box.count_x() {
            self.x = 0;
            self.y += 1;
            return self.next();
        }

        let to_return = self.grid_box.at(self.x, self.y);
        self.x += 1;
        Some(to_return)
    }
}

pub struct LayoutIntoIter {
    grid_box: Layout,
    x: u32,
    y: u32,
}

impl LayoutIntoIter {
    pub fn new(grid_box: Layout) -> Self {
        Self {
            grid_box,
            x: 0,
            y: 0,
        }
    }
}

impl Iterator for LayoutIntoIter {
    type Item = Rect;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.grid_box.count_y() {
            return None;
        }

        if self.x >= self.grid_box.count_x() {
            self.x = 0;
            self.y += 1;
            return self.next();
        }

        let to_return = self.grid_box.at(self.x, self.y);
        self.x += 1;
        Some(to_return)
    }
}
