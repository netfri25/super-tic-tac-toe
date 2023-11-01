use macroquad::prelude::*;

use crate::draw::{Drawable, Paddable};
use crate::layout::Layout;
use crate::player::Player;
use crate::utils::{PADDING, THICK_MULT, BLOCKED_COLOR, GRID_LINES_COLOR};

pub type Index = u8;

#[derive(Debug, Default, Clone)]
pub struct Grid {
    subgrids: [SubGrid; 9],
    only_allowed: Option<Index>,
}

impl Grid {
    pub fn play(&mut self, player: Player, outer_index: u8, inner_index: u8) -> bool {
        let can_interact = self.only_allowed.map(|i| outer_index == i).unwrap_or(true);
        if !can_interact || self.subgrids[outer_index as usize].is_done() {
            return false
        }

        let valid = self.subgrids[outer_index as usize].play(inner_index, player);
        if valid {
            if self.subgrids[inner_index as usize].is_done() {
                self.only_allowed = None
            } else {
                self.only_allowed = Some(inner_index);
            }
        }

        valid
    }

    pub fn unplay(&mut self, outer_index: u8, inner_index: u8, only_allowed: Option<Index>) {
        self.subgrids[outer_index as usize].unplay(inner_index);
        self.only_allowed = only_allowed;
    }

    pub fn to_subgrid(&self) -> SubGrid {
        let mut subgrid = SubGrid::default();

        for (i, grid) in self.subgrids.iter().enumerate() {
            if let Some(player) = grid.winner() {
                subgrid.play(i as u8, player);
            }
        }

        subgrid
    }

    pub fn subgrids(&self) -> &[SubGrid] {
        self.subgrids.as_slice()
    }

    pub fn winner(&self) -> Option<Player> {
        self.to_subgrid().winner()
    }

    pub fn only_allowed(&self) -> Option<Index> {
        self.only_allowed
    }

    pub fn is_filled(&self) -> bool {
        self.to_subgrid().is_filled()
    }
}

impl Drawable for Grid {
    fn draw(&self, bounds: Rect) {
        if let Some(winner) = self.winner() {
            winner.draw(bounds);
            return;
        }

        for (i, (subgrid, r)) in self.subgrids.iter().zip(padded_grid(bounds, PADDING)).enumerate() {
            if let Some(winner) = subgrid.winner() {
                winner.draw(r);
                continue;
            }

            subgrid.draw(r);

            let blocked = self.only_allowed.map(|index| index != i as u8).unwrap_or(false);
            if blocked {
                draw_rectangle(r.x, r.y, r.w, r.h, BLOCKED_COLOR);
            }
        }

        draw_grid_lines(bounds, bounds.w * THICK_MULT);
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct SubGrid {
    x_bits: u16,
    o_bits: u16,
}

impl SubGrid {
    pub fn at(&self, index: u8) -> Option<Player> {
        let mask = 1u16 << index;

        if (self.x_bits & mask) != 0 {
            Some(Player::X)
        } else if (self.o_bits & mask) != 0 {
            Some(Player::O)
        } else {
            None
        }
    }

    pub fn is_filled(&self) -> bool {
        (self.x_bits | self.o_bits) == 0b111111111
    }

    pub fn is_done(&self) -> bool {
        self.is_filled() || self.winner().is_some()
    }

    pub fn winner(&self) -> Option<Player> {
        const WIN_MASKS: [u16; 8] = [
            0b000000111,
            0b000111000,
            0b111000000,
            0b001001001,
            0b010010010,
            0b100100100,
            0b100010001,
            0b001010100,
        ];

        for mask in WIN_MASKS {
            if (self.x_bits & mask) == mask {
                return Some(Player::X);
            }

            if (self.o_bits & mask) == mask {
                return Some(Player::O);
            }
        }

        None
    }

    pub fn play(&mut self, index: u8, player: Player) -> bool {
        if !self.empty(index) {
            return false;
        }

        let mask = 1u16 << index;
        match player {
            Player::X => self.x_bits |= mask,
            Player::O => self.o_bits |= mask,
        }

        true
    }

    pub fn unplay(&mut self, index: u8) {
        let mask = 1u16 << index;
        self.x_bits &= !mask;
        self.o_bits &= !mask;
    }

    pub fn empty(&self, index: u8) -> bool {
        let mask = 1u16 << index;
        let cells = self.x_bits | self.o_bits;
        (cells & mask) == 0
    }
}

impl Drawable for SubGrid {
    fn draw(&self, bounds: Rect) {
        let bounds = bounds.pad(PADDING);
        draw_grid_lines(bounds, bounds.w * THICK_MULT);

        for (i, cell_bounds) in padded_grid(bounds, PADDING).enumerate() {
            if let Some(player) = self.at(i as u8) {
                player.draw(cell_bounds.pad(PADDING))
            }
        }
    }
}

fn draw_grid_lines(bounds: Rect, thick: f32) {
    for r in Layout::new(bounds, 3, 1).iter().take(2) {
        let x = r.x + r.w;
        let y1 = r.y;
        let y2 = y1 + r.h;
        draw_line(x, y1, x, y2, thick, GRID_LINES_COLOR);
    }

    for r in Layout::new(bounds, 1, 3).iter().take(2) {
        let y = r.y + r.h;
        let x1 = r.x;
        let x2 = x1 + r.w;
        draw_line(x1, y, x2, y, thick, GRID_LINES_COLOR);
    }
}

pub fn padded_grid(bounds: Rect, pad: f32) -> impl Iterator<Item = Rect> {
    Layout::new(bounds, 3, 3).into_iter().map(move |r| r.pad(pad))
}
