use std::collections::VecDeque;

use itertools::Itertools;

use crate::constants::{self, EVALUATION_MAX, EVALUATION_MIN};
use crate::grid::{GeneralCell, Cell, Grid, Player};
use crate::game::Game;

pub struct Suggestion {
    pub moves: Vec<VecDeque<usize>>,
    pub evals: Vec<Evaluation>,
}

pub fn get_best_moves(game: &mut Game) -> Suggestion {
    let options = game.grid.generate_moves()
        .into_iter()
        .map(|indices| {
            game.play(indices.clone().into_iter());
            let eval = -search(game, constants::EVALUATION_MIN, constants::EVALUATION_MAX, constants::MAX_SEARCH_DEPTH);
            game.rewind_step();
            (indices, eval)
        })
        .collect_vec();

    let max_set = options.into_iter().max_set_by(|(_, a_eval), (_, b_eval)| a_eval.partial_cmp(b_eval).unwrap());
    let evals = max_set.iter().map(|(_, eval)| *eval).collect_vec();
    let moves = max_set.into_iter().map(|(indices, _)| indices).collect_vec();
    Suggestion { moves, evals }
}

trait MoveGeneration: GeneralCell {
    fn generate_moves(&self) -> Vec<VecDeque<usize>>;
}

impl MoveGeneration for Cell {
    fn generate_moves(&self) -> Vec<VecDeque<usize>> {
        if self.is_none() {
            vec![VecDeque::new()]
        } else {
            vec![]
        }
    }
}

impl<C> MoveGeneration for Grid<C>
where
    C: MoveGeneration
{
    fn generate_moves(&self) -> Vec<VecDeque<usize>> {
        if self.value().is_some() {
            return vec![];
        }

        let mut result = Vec::new();

        for i in self.allowed_indices() {
            let mut inner_moves = self.cells()[i].generate_moves();
            if inner_moves.is_empty() { continue }
            inner_moves.iter_mut().for_each(|v| v.push_front(i));
            result.extend(inner_moves);
        }

        result
    }
}

pub type Evaluation = i32;

trait Evaluate {
    fn evaluate(&self, player: Player) -> Evaluation;
}

impl Evaluate for Player {
    fn evaluate(&self, player: Player) -> Evaluation {
        if *self == player { EVALUATION_MAX } else { EVALUATION_MIN }
    }
}

impl Evaluate for Cell {
    fn evaluate(&self, player: Player) -> Evaluation {
        self.map(|p| p.evaluate(player)).unwrap_or(0)
    }
}

impl<C> Evaluate for Grid<C>
where
    C: Evaluate + GeneralCell
{
    fn evaluate(&self, player: Player) -> Evaluation {
        if self.is_draw() {
            return 0;
        }

        if let Some(winner) = self.winner() {
            winner.evaluate(player)
        } else {
            let weights = [
                3, 2, 3,
                2, 4, 2,
                3, 2, 3,
            ];

            let weights_sum = weights.iter().sum::<Evaluation>();

            self.cells()
                .iter()
                .zip(weights)
                .map(|(c, num)| c.evaluate(player) * num)
                .sum::<Evaluation>() / weights_sum
        }
    }
}

pub fn search(game: &mut Game, mut alpha: Evaluation, beta: Evaluation, depth: usize) -> Evaluation {
    if depth == 0 {
        return game.grid.evaluate(game.turn);
    }

    for indices in game.grid.generate_moves() {
        game.play(indices.clone().into_iter());
        let evaluation = -search(game, -beta, -alpha, depth - 1);
        game.rewind_step();
        if evaluation >= beta { return beta; }
        alpha = alpha.max(evaluation);
    }

    alpha
}
