use itertools::Itertools;

pub trait GeneralCell {
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

    fn cvalue(&self) -> Option<Player> {
        *self
    }
}

#[derive(Debug, Clone, Default)]
pub struct Grid<C> {
    cells: [C; 9],
    winner: Option<Player>,
}

impl<C> Grid<C> {
    pub fn new() -> Self
    where
        C: Default,
    {
        Self::default()
    }

    pub fn cells(&self) -> &[C; 9] {
        &self.cells
    }

    pub fn winner(&self) -> Option<Player> {
        self.winner
    }

    pub fn update_with<T>(&mut self, row: usize, col: usize, func: impl FnOnce(&mut C) -> T) -> Option<T> {
        self.cells.get_mut(row * 3 + col).map(func)
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
}
