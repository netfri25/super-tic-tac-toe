use itertools::Itertools;

use crate::grid::{Grid, SubGrid};
use crate::player::Player;

pub const BOT_DEPTH: usize = 7;

pub fn best_indices(grid: Grid, turn: Player) -> Vec<((u8, u8), Eval)> {
    let indices = generate_indices(&grid).collect_vec();
    let mut searched = 0;
    let best = indices
        .into_iter()
        .map(|indices| {
            let mut grid = grid.clone();
            grid.play(indices, turn);
            let eval = -search(
                &mut searched,
                grid,
                turn.other(),
                -EVAL_WIN_WEIGHT,
                EVAL_WIN_WEIGHT,
                BOT_DEPTH,
            );
            (indices, eval)
        })
        .max_set_by_key(|(_, eval)| *eval);

    println!("searched {} positions", searched);
    best
}

pub type Eval = i32;
const EVAL_WIN_WEIGHT: Eval = i16::MAX as i32;
const EVAL_NEUTRAL: Eval = 0;

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
const WEIGHTS: [i32; 9] = [3, 2, 3, 2, 4, 2, 3, 2, 3];

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
            return winner.eval(player) * WEIGHTS_SUM;
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

const INDICES_ORDER: [u8; 9] = [4, 0, 2, 6, 8, 1, 3, 5, 7];

fn generate_indices(grid: &Grid) -> impl Iterator<Item = (u8, u8)> + '_ {
    let only_allowed = grid.only_allowed();
    let allowed = INDICES_ORDER.into_iter().filter(move |i| only_allowed.map(|index| index == *i).unwrap_or(true));
    let empty = allowed.filter(|&i| !grid.subgrids()[i as usize].is_done());
    empty.flat_map(|i| {
        let subgrid = grid.subgrids()[i as usize];
        INDICES_ORDER
            .into_iter()
            .filter(move |&j| subgrid.empty(j))
            .map(move |j| (i, j))
    })
}

fn search(searched: &mut usize, grid: Grid, player: Player, mut alpha: Eval, beta: Eval, depth: usize) -> Eval {
    if let Some(winner) = grid.winner() {
        return winner.eval(player) * EVAL_WIN_WEIGHT;
    }

    if grid.is_filled() {
        return EVAL_NEUTRAL;
    }

    if depth == 0 {
        return grid.eval(player);
    }

    let all_indices = generate_indices(&grid).collect_vec();
    if all_indices.is_empty() {
        return EVAL_NEUTRAL;
    }

    for indices in all_indices {
        *searched += 1;
        let mut grid = grid.clone();
        grid.play(indices, player);
        let eval = -search(searched, grid, player.other(), -beta, -alpha, depth - 1);
        alpha = eval.max(alpha);
        if eval >= beta { return beta }
    }

    alpha
}
