use itertools::Itertools;
use macroquad::prelude::*;

pub enum ClickResult {
    Invalid,
    SetCell,
    SetIndex(usize),
}
use ClickResult::*;

use crate::{constants::PAD, draw::pad_rect};

impl ClickResult {
    pub fn is_valid(&self) -> bool {
        !matches!(self, Invalid)
    }
}

pub trait GeneralCell {
    fn click(&mut self, player: Player, bounds: Rect) -> ClickResult;
    fn is_draw(&self) -> bool;
    fn cupdate(&mut self);
    fn cvalue(&self) -> Option<Player>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn switch(&mut self) {
        *self = match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

pub type Cell = Option<Player>;

impl GeneralCell for Cell {
    fn cupdate(&mut self) {}

    fn click(&mut self, player: Player, bounds: Rect) -> ClickResult {
        let mouse_pos = mouse_position().into();

        if bounds.contains(mouse_pos) && self.is_none() {
            *self = Some(player);
            SetCell
        } else {
            Invalid
        }
    }

    fn cvalue(&self) -> Option<Player> {
        *self
    }

    fn is_draw(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone)]
pub struct Grid<C> {
    cells: [C; 9],
    winner: Option<Player>,
    allowed: [bool; 9],
}

impl<C> Grid<C> {
    pub fn new() -> Self
    where
        C: Default,
    {
        Self {
            cells: Default::default(),
            winner: None,
            allowed: [true; 9],
        }
    }

    pub fn cells(&self) -> &[C; 9] {
        &self.cells
    }

    pub fn winner(&self) -> Option<Player> {
        self.winner
    }

    pub fn allowed(&self) -> &[bool; 9] {
        &self.allowed
    }

    pub fn update_with<T>(&mut self, index: usize, func: impl FnOnce(&mut C) -> T) -> Option<T> {
        self.cells.get_mut(index).map(func)
    }

    // TODO: there MUST be a cleaner way to do this
    pub fn update_winner(&mut self)
    where
        C: GeneralCell,
    {
        self.cells.iter_mut().for_each(C::cupdate);

        let tl = &self.cells[0];
        let tr = &self.cells[2];
        let mm = &self.cells[4];
        let bl = &self.cells[6];
        let br = &self.cells[8];

        for slope in [[tl, mm, br], [tr, mm, bl]] {
            if slope.iter().all(|c| c.cvalue().is_some())
                && slope.iter().map(|c| c.cvalue()).all_equal()
            {
                self.winner = slope[0].cvalue();
                return;
            }
        }

        for i in 0..3 {
            // check rows
            let mut iter = self.cells.iter().skip(i * 3).take(3).map(C::cvalue);
            if let Some(winner) = iter
                .clone()
                .next()
                .filter(|cell| cell.is_some() && iter.all_equal())
            {
                self.winner = winner.cvalue();
                break;
            }

            // check columns
            let mut iter = self.cells.iter().skip(i).step_by(3).map(C::cvalue);
            if let Some(winner) = iter
                .clone()
                .next()
                .filter(|cell| cell.is_some() && iter.all_equal())
            {
                self.winner = winner.cvalue();
                break;
            }
        }
    }
}

impl<C> Default for Grid<C>
where
    C: Default,
{
    fn default() -> Self {
        Self::new()
    }
}

// a Grid can act as a Cell
impl<C> GeneralCell for Grid<C>
where
    C: GeneralCell,
{
    fn cupdate(&mut self) {
        self.update_winner()
    }

    fn cvalue(&self) -> Option<Player> {
        self.winner()
    }

    fn click(&mut self, player: Player, bounds: Rect) -> ClickResult {
        let mouse_pos = mouse_position().into();
        if !bounds.contains(mouse_pos) {
            return Invalid;
        }

        let nx = (mouse_pos.x - bounds.x) / bounds.w;
        let ny = (mouse_pos.y - bounds.y) / bounds.h;

        let Some(index) = grid_index(nx, ny) else {
            return Invalid;
        };

        if !self.allowed[index] {
            return Invalid;
        }

        let place_result = self
            .update_with(index, |cell| {
                let row = (index / 3) as f32;
                let col = (index % 3) as f32;
                let w = bounds.w / 3.;
                let h = bounds.h / 3.;
                let x = col * w + bounds.x;
                let y = row * h + bounds.y;
                let new_rect = pad_rect(Rect::new(x, y, w, h), PAD);
                cell.click(player, new_rect)
            })
            .unwrap();

        self.update_winner();

        match place_result {
            Invalid => return Invalid,
            SetCell => {}
            SetIndex(inner_index) => {
                if self.cells[inner_index].cvalue().is_some() || self.cells[inner_index].is_draw() {
                    self.allowed = [true; 9];
                } else {
                    self.allowed = [false; 9];
                    self.allowed[inner_index] = true;
                }
            }
        };

        if self.winner.is_some() {
            self.allowed = [true; 9];
        }

        SetIndex(index)
    }

    fn is_draw(&self) -> bool {
        self.cells.iter().all(|c| c.is_draw() || c.cvalue().is_some())
    }
}

pub fn grid_index(x: f32, y: f32) -> Option<usize> {
    if !(0f32..=1f32).contains(&x) || !(0f32..=1f32).contains(&y) {
        return None;
    }
    let col = x * 3.;
    let row = y * 3.;
    let index = row as usize * 3 + col as usize;
    Some(index)
}
