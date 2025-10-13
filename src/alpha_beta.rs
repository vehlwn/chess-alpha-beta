use crate::board_value::board_value;
use anyhow::Context;
use pleco::Player;
use rayon::prelude::*;

pub type ValueType = i32;

#[derive(Debug, Clone, Default)]
pub struct EvaluationContext {
    pub depth: u32,
    pub alpha: ValueType,
    pub beta: ValueType,
}

#[derive(Debug, Clone)]
pub struct EvaluatedMove {
    pub m: pleco::BitMove,
    pub value: ValueType,
}

fn shuffled_move_list(it: pleco::MoveList) -> Vec<pleco::BitMove> {
    use rand::seq::SliceRandom;
    let mut rng = rand::rng();
    let mut y: Vec<pleco::BitMove> = it.iter().copied().collect();
    y.shuffle(&mut rng);
    y
}

pub fn get_best_move(
    board: &pleco::Board,
    depth: std::num::NonZeroU32,
) -> anyhow::Result<EvaluatedMove> {
    let color = match board.turn() {
        Player::White => 1,
        Player::Black => -1,
    };
    let context = EvaluationContext {
        depth: depth.get(),
        alpha: -10_000_000,
        beta: 10_000_000,
    };
    let possible_moves = shuffled_move_list(board.generate_moves());
    // Use par_iter for the first level of Negamax to remember moves
    let (best_move, mut best_value) = possible_moves
        .par_iter()
        .cloned()
        .map(|m| {
            let mut experiment_board = board.clone();
            experiment_board.apply_move(m);
            let value = -alpha_beta_impl(
                &experiment_board,
                EvaluationContext {
                    depth: context.depth - 1,
                    ..context
                },
            );
            experiment_board.undo_move();
            (m, value)
        })
        .max_by_key(|(_, value)| *value)
        .context("No available moves")?;
    // Invert color back if current player is minimizer
    best_value *= color;
    Ok(EvaluatedMove {
        m: best_move,
        value: best_value,
    })
}

fn alpha_beta_impl(
    board: &pleco::Board,
    mut context: EvaluationContext,
) -> ValueType {
    if context.depth == 0 || board.checkmate() {
        let color = match board.turn() {
            Player::White => 1,
            Player::Black => -1,
        };
        // Value of a minimizer player must be negated
        return color * board_value(board, context.depth);
    }
    if board.stalemate() {
        return 0;
    }
    let mut experiment_board = board.clone();
    let mut best_value = -10_000_000;
    for m in shuffled_move_list(board.generate_moves()) {
        experiment_board.apply_move(m);
        let value = -alpha_beta_impl(
            &experiment_board,
            EvaluationContext {
                depth: context.depth - 1,
                alpha: -context.beta,
                beta: -context.alpha,
            },
        );
        experiment_board.undo_move();
        best_value = best_value.max(value);
        context.alpha = context.alpha.max(best_value);
        if context.alpha >= context.beta {
            break;
        }
    }
    best_value
}
