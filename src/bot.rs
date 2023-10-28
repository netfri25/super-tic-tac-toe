use std::collections::VecDeque;

use itertools::Itertools;

use crate::constants;
use crate::grid::{GeneralCell, Cell, Grid, Player};
use crate::game::Game;

pub fn get_best_moves(game: &mut Game) -> (Vec<VecDeque<usize>>, Vec<f32>) {
    let options = game.grid.generate_moves()
        .into_iter()
        .map(|indices| {
            game.play(indices.clone().into_iter());
            let eval = -search(game, -1., 1., constants::MAX_SEARCH_DEPTH);
            game.rewind_step();
            (indices, eval)
        })
        .collect_vec();

    let max_set = options.into_iter().max_set_by(|(_, a_eval), (_, b_eval)| a_eval.partial_cmp(b_eval).unwrap());
    let evals = max_set.iter().map(|(_, eval)| *eval).collect_vec();
    let moves = max_set.into_iter().map(|(indices, _)| indices).collect_vec();
    (moves, evals)
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

        for i in self.allowed_range() {
            let mut inner_moves = self.cells()[i].generate_moves();
            if inner_moves.is_empty() { continue }
            inner_moves.iter_mut().for_each(|v| v.push_front(i));
            result.extend(inner_moves);
        }

        result
    }
}

trait Evaluate  {
    fn evaluate(&self, player: Player) -> f32;
}

impl Evaluate for Player {
    fn evaluate(&self, player: Player) -> f32 {
        if *self == player { 1. } else { -1. }
    }
}

impl Evaluate for Cell {
    fn evaluate(&self, player: Player) -> f32 {
        self.map(|p| p.evaluate(player)).unwrap_or(0.)
    }
}

impl<C> Evaluate for Grid<C>
where
    C: Evaluate
{
    fn evaluate(&self, player: Player) -> f32 {
        if let Some(winner) = self.winner() {
            winner.evaluate(player)
        } else {
            let weights = [
                3. / 8., 1. / 4., 3. / 8.,
                1. / 4., 1. / 2., 1. / 4.,
                3. / 8., 1. / 4., 3. / 8.,
            ];
            self.cells().iter().zip(weights).map(|(c, w)| c.evaluate(player) * w).sum::<f32>() / 9.
        }
    }
}

pub fn search(game: &mut Game, mut alpha: f32, beta: f32, depth: usize) -> f32 {
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
