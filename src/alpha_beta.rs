use crate::board_value::board_value;
use crate::shuffled_move_list::shuffled_move_list;
use rayon::prelude::*;

pub type ValueType = i32;

#[derive(Debug, Clone)]
pub struct EvaluationContext {
    pub max_depth: i32,
    pub current_depth: i32,
    pub alpha: ValueType,
    pub beta: ValueType,
    pub maximize: bool,
}

#[derive(Debug, Clone)]
pub struct EvaluatedMove {
    pub m: pleco::BitMove,
    pub value: ValueType,
}
impl PartialEq for EvaluatedMove {
    fn eq(&self, other: &Self) -> bool {
        return self.value == other.value;
    }
}
impl Eq for EvaluatedMove {}
impl PartialOrd for EvaluatedMove {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}
impl Ord for EvaluatedMove {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.value.cmp(&other.value);
    }
}

pub fn get_best_move(
    board: &pleco::Board,
    depth: i32,
    maximize: bool,
) -> EvaluatedMove {
    let context = EvaluationContext {
        max_depth: depth,
        current_depth: depth,
        alpha: ValueType::MIN,
        beta: ValueType::MAX,
        maximize,
    };
    let possible_moves = shuffled_move_list(board.generate_moves());
    if context.maximize {
        // Use par_iter for only the first level of minimax algorithm.
        let best_move = possible_moves
            .par_iter()
            .cloned()
            .map(|m| {
                let mut experiment_board = board.clone();
                experiment_board.apply_move(m);
                let evaluated_move = alpha_beta_impl(
                    &experiment_board,
                    EvaluationContext {
                        max_depth: context.max_depth,
                        current_depth: context.current_depth - 1,
                        alpha: context.alpha,
                        beta: context.beta,
                        maximize: !context.maximize,
                    },
                );
                experiment_board.undo_move();
                return EvaluatedMove {
                    m,
                    value: evaluated_move.value,
                };
            })
            .max()
            .unwrap();
        return best_move;
    } else {
        let best_move = possible_moves
            .par_iter()
            .cloned()
            .map(|m| {
                let mut experiment_board = board.clone();
                experiment_board.apply_move(m);
                let evaluated_move = alpha_beta_impl(
                    &experiment_board,
                    EvaluationContext {
                        max_depth: context.max_depth,
                        current_depth: context.current_depth - 1,
                        alpha: context.alpha,
                        beta: context.beta,
                        maximize: !context.maximize,
                    },
                );
                experiment_board.undo_move();
                return EvaluatedMove {
                    m,
                    value: evaluated_move.value,
                };
            })
            .min()
            .unwrap();
        return best_move;
    }
}

fn alpha_beta_impl(
    board: &pleco::Board,
    mut context: EvaluationContext,
) -> EvaluatedMove {
    if context.current_depth <= 0 || board.checkmate() {
        return EvaluatedMove {
            m: pleco::BitMove::null(),
            value: board_value(&board, &context),
        };
    }
    if board.stalemate() {
        return EvaluatedMove {
            m: pleco::BitMove::null(),
            value: ValueType::default(),
        };
    }
    let mut experiment_board = board.clone();
    if context.maximize {
        let mut best_move = EvaluatedMove {
            m: pleco::BitMove::null(),
            value: ValueType::MIN,
        };
        for m in shuffled_move_list(board.generate_moves()) {
            experiment_board.apply_move(m);
            let evaluated_move = alpha_beta_impl(
                &experiment_board,
                EvaluationContext {
                    max_depth: context.max_depth,
                    current_depth: context.current_depth - 1,
                    alpha: context.alpha,
                    beta: context.beta,
                    maximize: !context.maximize,
                },
            );
            experiment_board.undo_move();
            if evaluated_move.value > best_move.value {
                best_move.m = m;
                best_move.value = evaluated_move.value;
            }
            context.alpha = context.alpha.max(best_move.value);
            if context.alpha >= context.beta {
                break;
            }
        }
        return best_move;
    } else {
        let mut best_move = EvaluatedMove {
            m: pleco::BitMove::null(),
            value: ValueType::MAX,
        };
        for m in shuffled_move_list(board.generate_moves()) {
            experiment_board.apply_move(m);
            let evaluated_move = alpha_beta_impl(
                &experiment_board,
                EvaluationContext {
                    max_depth: context.max_depth,
                    current_depth: context.current_depth - 1,
                    alpha: context.alpha,
                    beta: context.beta,
                    maximize: !context.maximize,
                },
            );
            experiment_board.undo_move();
            if evaluated_move.value < best_move.value {
                best_move.m = m;
                best_move.value = evaluated_move.value;
            }
            context.beta = context.beta.min(best_move.value);
            if context.alpha >= context.beta {
                break;
            }
        }
        return best_move;
    }
}
