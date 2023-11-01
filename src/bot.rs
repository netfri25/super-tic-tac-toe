use crate::grid::{Grid, SubGrid};
use crate::player::Player;

pub const BOT_DEPTH: usize = 6;

pub fn best_indices(grid: Grid, turn: Player) -> Option<((u8, u8), Eval)> {
    let indices = generate_indices(&grid);
    indices.map(|(outer, inner)| {
        let mut grid = grid.clone();
        grid.play(turn, outer, inner);
        let eval = -search(grid, turn.other(), -EVAL_WIN_WEIGHT, EVAL_WIN_WEIGHT, BOT_DEPTH);
        ((outer, inner), eval)
    }).max_by_key(|(_, eval)| *eval)
}

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
        self.subgrids()
            .iter()
            .zip(WEIGHTS)
            .map(|(subgrid, w)| subgrid.eval(player) * w)
            .sum()
    }
}

impl Evaluate for SubGrid {
    fn eval(&self, player: Player) -> Eval {
        if let Some(winner) = self.winner() {
            return winner.eval(player) * WEIGHTS_SUM
        }

        if self.is_filled() {
            return EVAL_NEUTRAL;
        }

        (0..9)
            .zip(WEIGHTS)
            .filter_map(|(i, w)| self.at(i).map(|p| p.eval(player) * w))
            .sum()
    }
}

pub const INDICES_ORDER: [u8; 9] = [4, 0, 2, 6, 8, 1, 3, 5, 7];

fn generate_indices(grid: &Grid) -> impl Iterator<Item = (u8, u8)> + '_ {
    let indices = INDICES_ORDER.into_iter();
    let allowed = indices.filter(|&i| grid.only_allowed().map(|index| i == index).unwrap_or(true));
    let non_full = allowed.filter(|&i| !grid.subgrids()[i as usize].is_filled());
    non_full.flat_map(|i| {
        let subgrid = &grid.subgrids()[i as usize];
        (0..9).filter_map(move |j| Some((i, j)).filter(|_| subgrid.empty(j)))
    })
}

fn search(grid: Grid, player: Player, mut alpha: Eval, beta: Eval, depth: usize) -> Eval {
    if let Some(winner) = grid.winner() {
        return winner.eval(player) * EVAL_WIN_WEIGHT;
    }

    if grid.is_filled() {
        return EVAL_NEUTRAL;
    }

    if depth == 0 {
        return grid.eval(player);
    }

    for indices in generate_indices(&grid) {
        let mut grid = grid.clone();
        if !grid.play(player, indices.0, indices.1) { continue }
        let eval = -search(grid, player.other(), -beta, -alpha, depth - 1);
        alpha = eval.max(alpha);
        if eval >= beta { return beta }
    }

    alpha
}
