use crate::board_value::board_value;
use crate::shuffled_move_list::shuffled_move_list;
use pleco;

pub type ValueType = i32;

pub struct AlphaBetaParameters {
    pub max_depth: i32,
    pub current_depth: i32,
    pub alpha: ValueType,
    pub beta: ValueType,
    pub maximize: bool,
}

pub fn get_best_move(
    board: &pleco::Board,
    depth: i32,
    maximize: bool,
) -> (pleco::BitMove, ValueType) {
    // Perform separate step of alpha-beta pruning to get best value along with a move.
    let mut experiment_board = board.clone();
    let mut best_move = pleco::BitMove::null();
    let mut params = AlphaBetaParameters {
        max_depth: depth,
        current_depth: depth,
        alpha: ValueType::MIN,
        beta: ValueType::MAX,
        maximize,
    };
    if params.maximize {
        let mut best_value = ValueType::MIN;
        for m in shuffled_move_list(board.generate_moves()) {
            experiment_board.apply_move(m);
            let value = alpha_beta(
                &experiment_board,
                &mut AlphaBetaParameters {
                    max_depth: params.max_depth,
                    current_depth: params.current_depth - 1,
                    alpha: params.alpha,
                    beta: params.beta,
                    maximize: !params.maximize,
                },
            );
            experiment_board.undo_move();
            if value > best_value {
                best_value = value;
                best_move = m;
            }
            params.alpha = params.alpha.max(best_value);
            if params.alpha >= params.beta {
                break;
            }
        }
        return (best_move, best_value);
    } else {
        let mut best_value = ValueType::MAX;
        for m in shuffled_move_list(board.generate_moves()) {
            experiment_board.apply_move(m);
            let value = alpha_beta(
                &experiment_board,
                &mut AlphaBetaParameters {
                    max_depth: params.max_depth,
                    current_depth: params.current_depth - 1,
                    alpha: params.alpha,
                    beta: params.beta,
                    maximize: !params.maximize,
                },
            );
            experiment_board.undo_move();
            if value < best_value {
                best_value = value;
                best_move = m;
            }
            params.beta = params.beta.min(best_value);
            if params.alpha >= params.beta {
                break;
            }
        }
        return (best_move, best_value);
    }
}

fn alpha_beta(board: &pleco::Board, params: &mut AlphaBetaParameters) -> ValueType {
    if params.current_depth <= 0 || board.checkmate() {
        return board_value(&board, &params);
    }
    if board.stalemate() {
        return ValueType::default();
    }
    let mut experiment_board = board.clone();
    if params.maximize {
        let mut best_value = ValueType::MIN;
        for m in shuffled_move_list(board.generate_moves()) {
            experiment_board.apply_move(m);
            let value = alpha_beta(
                &experiment_board,
                &mut AlphaBetaParameters {
                    max_depth: params.max_depth,
                    current_depth: params.current_depth - 1,
                    alpha: params.alpha,
                    beta: params.beta,
                    maximize: !params.maximize,
                },
            );
            experiment_board.undo_move();
            if value > best_value {
                best_value = value;
            }
            params.alpha = params.alpha.max(best_value);
            if params.alpha >= params.beta {
                break;
            }
        }
        return best_value;
    } else {
        let mut best_value = ValueType::MAX;
        for m in shuffled_move_list(board.generate_moves()) {
            experiment_board.apply_move(m);
            let value = alpha_beta(
                &experiment_board,
                &mut AlphaBetaParameters {
                    max_depth: params.max_depth,
                    current_depth: params.current_depth - 1,
                    alpha: params.alpha,
                    beta: params.beta,
                    maximize: !params.maximize,
                },
            );
            experiment_board.undo_move();
            if value < best_value {
                best_value = value;
            }
            params.beta = params.beta.min(best_value);
            if params.alpha >= params.beta {
                break;
            }
        }
        return best_value;
    }
}
