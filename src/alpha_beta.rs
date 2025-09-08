use crate::board_value::board_value;
use anyhow::Context;

pub type ValueType = i32;

#[derive(Debug, Clone)]
pub struct EvaluationContext {
    pub max_depth: u32,
    pub current_depth: u32,
    pub alpha: ValueType,
    pub beta: ValueType,
    pub color: i32,
}

#[derive(Debug, Clone)]
pub struct EvaluatedMove {
    pub m: pleco::BitMove,
    pub value: ValueType,
}

fn shuffled_move_list(it: pleco::MoveList) -> Vec<pleco::BitMove> {
    use rand::seq::SliceRandom;
    let mut rng = rand::rng();
    let mut y: Vec<pleco::BitMove> = it.iter().map(|x| *x).collect();
    y.shuffle(&mut rng);
    return y;
}

pub fn get_best_move(
    board: &pleco::Board,
    depth: std::num::NonZeroU32,
) -> anyhow::Result<EvaluatedMove> {
    use pleco::Player;
    use rayon::prelude::*;

    let color = match board.turn() {
        Player::White => 1,
        Player::Black => -1,
    };
    let context = EvaluationContext {
        max_depth: depth.get(),
        current_depth: depth.get(),
        alpha: -10_000_000,
        beta: 10_000_000,
        color,
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
                    current_depth: context.current_depth - 1,
                    color: -context.color,
                    ..context
                },
            );
            experiment_board.undo_move();
            return (m, value);
        })
        .max_by_key(|(_, value)| *value)
        .context("No available moves")?;
    // Invert color back if current player is minimizer
    best_value *= color;
    return Ok(EvaluatedMove {
        m: best_move,
        value: best_value,
    });
}

fn alpha_beta_impl(
    board: &pleco::Board,
    mut context: EvaluationContext,
) -> ValueType {
    if context.current_depth == 0 || board.checkmate() {
        // Value of a minimizer player must be negated
        return context.color * board_value(&board, &context);
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
                current_depth: context.current_depth - 1,
                alpha: -context.beta,
                beta: -context.alpha,
                color: -context.color,
                ..context
            },
        );
        experiment_board.undo_move();
        best_value = best_value.max(value);
        context.alpha = context.alpha.max(best_value);
        if context.alpha >= context.beta {
            break;
        }
    }
    return best_value;
}
