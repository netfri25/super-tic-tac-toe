use macroquad::prelude::*;

use crate::draw::{Drawable, Paddable};
use crate::get_cells;
use crate::grid::Grid;
use crate::player::Player;
use crate::utils::{PADDING, BLOCKED_COLOR, Indices, Index};

struct History {
    only_allowed: Option<Index>,
    indices: Indices,
}

pub struct Game {
    grid: Grid,
    turn: Player,
    history: Vec<History>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            grid: Grid::default(),
            turn: Player::X,
            history: Vec::new(),
        }
    }

    pub fn clone_grid(&self) -> Grid {
        self.grid.clone()
    }

    pub fn turn(&self) -> Player {
        self.turn
    }

    pub fn play(&mut self, indices: (u8, u8)) -> bool {
        let only_allowed = self.grid.only_allowed();
        let played = self.grid.play(indices, self.turn);
        if played {
            self.turn = self.turn.other();
            self.history.push(History {
                only_allowed,
                indices,
            });
        }

        played
    }

    pub fn undo(&mut self) {
        let Some(History { only_allowed, indices }) = self.history.pop() else {
            return;
        };

        self.grid.unplay(indices, only_allowed);
        self.turn = self.turn.other();
    }

    pub fn finished(&self) -> bool {
        self.grid.is_filled() || self.grid.winner().is_some()
    }
}

impl Drawable for Game {
    fn draw(&self, bounds: Rect) {
        self.grid.draw(bounds);

        let mpos = mouse_position().into();
        let cell = get_cells().find(|(_, r)| r.contains(mpos));

        if let Some((indices, r)) = cell {
            if self.grid.is_valid(indices) {
                let r = r.pad(PADDING);
                draw_rectangle(r.x, r.y, r.w, r.h, BLOCKED_COLOR);
            }
        }
    }
}

