use crate::board_value::board_value;
use crate::shuffled_move_list::shuffled_move_list;
use pleco;

pub struct AlphaBetaParameters {
    pub max_depth: i32,
    pub current_depth: i32,
    pub alpha: f64,
    pub beta: f64,
    pub maximize: bool,
}

pub fn get_best_move(
    board: &pleco::Board,
    depth: i32,
    maximize: bool,
) -> (pleco::BitMove, f64) {
    let (best_move, value) = alpha_beta(
        &board,
        &mut AlphaBetaParameters {
            max_depth: depth,
            current_depth: depth,
            alpha: f64::NEG_INFINITY,
            beta: f64::INFINITY,
            maximize,
        },
    );
    return (best_move.unwrap(), value);
}

fn alpha_beta(
    board: &pleco::Board,
    params: &mut AlphaBetaParameters,
) -> (Option<pleco::BitMove>, f64) {
    if params.current_depth <= 0 || board.checkmate() {
        return (None, board_value(&board, &params));
    }
    if board.stalemate() {
        return (None, 0.);
    }
    let mut experiment_board = board.clone();
    let mut best_move: Option<pleco::BitMove> = None;
    if params.maximize {
        let mut best_value = f64::NEG_INFINITY;
        for m in shuffled_move_list(board.generate_moves()) {
            experiment_board.apply_move(m);
            let (_, value) = alpha_beta(
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
                best_move = Some(m);
            }
            params.alpha = params.alpha.max(best_value);
            if params.alpha >= params.beta {
                break;
            }
        }
        return (best_move, best_value);
    } else {
        let mut best_value = f64::INFINITY;
        for m in shuffled_move_list(board.generate_moves()) {
            experiment_board.apply_move(m);
            let (_, value) = alpha_beta(
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
                best_move = Some(m);
            }
            params.beta = params.beta.min(best_value);
            if params.alpha >= params.beta {
                break;
            }
        }
        return (best_move, best_value);
    }
}
