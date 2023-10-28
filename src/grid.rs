use itertools::Itertools;
use macroquad::prelude::*;

#[derive(Debug)]
pub enum PlayResult {
    Invalid,
    SetCell,
    SetIndex(usize),
}
use PlayResult::*;

impl PlayResult {
    pub fn is_valid(&self) -> bool {
        !matches!(self, Invalid)
    }
}

pub trait GeneralCell {
    const DEPTH: usize;

    fn play(&mut self, player: Player, moves: impl Iterator<Item = usize>) -> PlayResult;
    fn is_draw(&self) -> bool;
    fn value(&self) -> Option<Player>;
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
    const DEPTH: usize = 0;

    fn play(&mut self, player: Player, _moves: impl Iterator<Item = usize>) -> PlayResult {
        if self.is_some() {
            return Invalid
        }

        *self = Some(player);
        SetCell
    }

    fn value(&self) -> Option<Player> {
        *self
    }

    fn is_draw(&self) -> bool {
        false
    }
}

pub type OnlyAllowed = Option<usize>;

#[derive(Debug)]
pub struct Grid<C> {
    cells: [C; 9],
    winner: Option<Player>,
    only_allowed: OnlyAllowed, // None when all of them are allowed
}

impl<C> Grid<C> {
    pub fn new() -> Self
    where
        C: Default,
    {
        Self {
            cells: Default::default(),
            winner: None,
            only_allowed: None,
        }
    }

    pub const fn cells(&self) -> &[C; 9] {
        &self.cells
    }

    pub const fn winner(&self) -> Option<Player> {
        self.winner
    }

    pub const fn allowed(&self, index: usize) -> bool {
        if let Some(allowed_index) = self.only_allowed {
            index == allowed_index
        } else {
            true
        }
    }

    // TODO: there MUST be a cleaner way to do this
    pub fn update_winner(&mut self)
    where
        C: GeneralCell,
    {
        self.winner = None;

        let tl = &self.cells[0];
        let tr = &self.cells[2];
        let mm = &self.cells[4];
        let bl = &self.cells[6];
        let br = &self.cells[8];

        for slope in [[tl, mm, br], [tr, mm, bl]] {
            if slope.iter().all(|c| c.value().is_some())
                && slope.iter().map(|c| c.value()).all_equal()
            {
                self.winner = slope[0].value();
                return;
            }
        }

        for i in 0..3 {
            // check rows
            let mut iter = self.cells.iter().skip(i * 3).take(3).map(C::value);
            if let Some(winner) = iter
                .clone()
                .next()
                .filter(|cell| cell.is_some() && iter.all_equal())
            {
                self.winner = winner.value();
                break;
            }

            // check columns
            let mut iter = self.cells.iter().skip(i).step_by(3).map(C::value);
            if let Some(winner) = iter
                .clone()
                .next()
                .filter(|cell| cell.is_some() && iter.all_equal())
            {
                self.winner = winner.value();
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
    const DEPTH: usize = 1 + C::DEPTH;

    fn value(&self) -> Option<Player> {
        self.winner()
    }

    fn play(&mut self, player: Player, mut moves: impl Iterator<Item = usize>) -> PlayResult {
        let Some(index) = moves.next() else {
            return Invalid;
        };

        if !self.allowed(index) {
            return Invalid;
        }

        let cell = &mut self.cells[index];
        let place_result = cell.play(player, moves);

        self.update_winner();

        match place_result {
            Invalid => return Invalid,
            SetCell => {}
            SetIndex(inner_index) => {
                if self.cells[inner_index].value().is_some() || self.cells[inner_index].is_draw() {
                    self.only_allowed = None;
                } else {
                    self.only_allowed = Some(inner_index);
                }
            }
        };

        SetIndex(index)
    }

    fn is_draw(&self) -> bool {
        self.cells.iter().all(|c| c.is_draw() || c.value().is_some())
    }
}
