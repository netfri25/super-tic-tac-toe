use std::collections::VecDeque;

use macroquad::prelude::*;

use crate::constants::PAD;
use crate::draw::pad_rect;
use crate::grid::{Cell, GeneralCell, Grid, Player, AllowedStatus};

pub type GameGrid = Grid<Grid<Cell>>;

pub struct Game {
    pub grid: GameGrid,
    pub turn: Player,
    pub history: Vec<VecDeque<AllowedStatus>>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            grid: Grid::new(),
            turn: Player::X,
            history: Vec::new(),
        }
    }

    pub fn mouse_press(&mut self, bounds: Rect) {
        let indices = IndicesGenerator::new(mouse_position().into(), bounds);
        self.play(indices);
    }

    pub fn play(&mut self, indices: impl Iterator<Item = usize> + Clone) -> bool {
        let Some(history) = self.grid.get_history(indices.clone()) else { return false };
        let placed = self.grid.play(self.turn, indices.clone());

        let valid = placed.is_valid();
        if valid {
            self.turn.switch();
            self.history.push(history);
        }

        valid
    }

    pub fn rewind_step(&mut self) {
        if let Some(last) = self.history.pop() {
            self.grid.unplay(last.into_iter());
            self.turn.switch();
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
struct IndicesGenerator {
    position: Vec2,
    bounds: Rect,
}

impl IndicesGenerator {
    pub fn new(position: Vec2, bounds: Rect) -> Self {
        Self { position, bounds }
    }
}

impl Iterator for IndicesGenerator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let norm_pos = (self.position - self.bounds.point()) / self.bounds.size();
        let index = grid_index(norm_pos)?;

        let row = (index / 3) as f32;
        let col = (index % 3) as f32;
        let w = self.bounds.w / 3.;
        let h = self.bounds.h / 3.;
        let x = col * w + self.bounds.x;
        let y = row * h + self.bounds.y;
        self.bounds = pad_rect(Rect::new(x, y, w, h), PAD);

        Some(index)
    }
}

fn grid_index(Vec2 { x, y }: Vec2) -> Option<usize> {
    if !(0f32..=1f32).contains(&x) || !(0f32..=1f32).contains(&y) {
        return None;
    }
    let col = x * 3.;
    let row = y * 3.;
    let index = row as usize * 3 + col as usize;
    Some(index)
}
