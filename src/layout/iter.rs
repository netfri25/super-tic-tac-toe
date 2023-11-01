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
    layout: &'a Layout,
    i: u32,
}

impl<'a> LayoutIter<'a> {
    pub fn new(layout: &'a Layout) -> Self {
        Self { layout, i: 0 }
    }
}

impl<'a> Iterator for LayoutIter<'a> {
    type Item = Rect;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.layout.count_x() * self.layout.count_y() {
            return None
        }

        let width = self.layout.count_x();
        let to_return = self.layout.at(self.i % width, self.i / width);
        self.i += 1;
        Some(to_return)
    }
}

pub struct LayoutIntoIter {
    layout: Layout,
    i: u32,
}

impl LayoutIntoIter {
    pub fn new(layout: Layout) -> Self {
        Self { layout, i: 0 }
    }
}

impl Iterator for LayoutIntoIter {
    type Item = Rect;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.layout.count_x() * self.layout.count_y() {
            return None
        }

        let width = self.layout.count_x();
        let to_return = self.layout.at(self.i % width, self.i / width);
        self.i += 1;
        Some(to_return)
    }
}
