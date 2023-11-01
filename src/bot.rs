use crate::grid::{Grid, SubGrid};
use crate::player::Player;


pub type Eval = i32;
pub const EVAL_WIN_WEIGHT: Eval = i16::MAX as i32;
pub const EVAL_NEUTRAL: Eval = 0;

pub trait Evaluate {
    fn eval(&self, player: Player) -> Eval;
}

impl Evaluate for Player {
    fn eval(&self, player: Player) -> Eval {
        if *self == player {
            1
        } else {
            -1
        }
    }
}

const WEIGHTS_SUM: i32 = 24;
const WEIGHTS: [i32; 9] = [
    3, 2, 3,
    2, 4, 2,
    3, 2, 3
];

impl Evaluate for Grid {
    fn eval(&self, player: Player) -> Eval {
        if let Some(winner) = self.winner() {
            return winner.eval(player) * EVAL_WIN_WEIGHT
        }

        if self.is_filled() {
            return EVAL_NEUTRAL;
        }

        self.subgrids().iter().zip(WEIGHTS).map(|(subgrid, w)| subgrid.eval(player) * w).sum()
    }
}

impl Evaluate for SubGrid {
    fn eval(&self, player: Player) -> Eval {
        if let Some(winner) = self.winner() {
            return winner.eval(player) * WEIGHTS_SUM;
        }

        if self.is_filled() {
            return EVAL_NEUTRAL;
        }

        (0..9).zip(WEIGHTS).map(|(i, w)| self.at(i).map(|p| p.eval(player) * w).unwrap_or(0)).sum()
    }
}
