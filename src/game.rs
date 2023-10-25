use macroquad::prelude::*;

use crate::grid::{Cell, GeneralCell, Grid, Player};

pub struct Game {
    pub grid: Grid<Grid<Cell>>,
    pub turn: Player,
}

impl Game {
    pub fn new() -> Self {
        Game {
            grid: Grid::new(),
            turn: Player::X,
        }
    }

    pub fn mouse_press(&mut self, bounds: Rect) {
        let placed = self.grid.click(self.turn, bounds);

        if placed.is_valid() {
            self.turn.switch();
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
