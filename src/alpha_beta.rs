use crate::board_pretty_print::board_pretty_print;
use crate::board_value::board_value;
use crate::shuffled_move_list::shuffled_move_list;
use pleco;

pub fn alpha_beta(
    board: &pleco::Board,
    depth: i32,
    mut alpha: f64,
    mut beta: f64,
    maximize: bool,
    verbose: bool,
) -> (Option<pleco::BitMove>, f64) {
    if depth <= 0 || board.checkmate() {
        if verbose {
            println!("experiment_board:");
            board_pretty_print(&board);
            println!("experiment value = {}", board_value(&board));
        }
        return (None, board_value(&board));
    }
    if board.stalemate() {
        return (None, 0.);
    }
    let mut experiment_board = board.clone();
    let mut best_move: Option<pleco::BitMove> = None;
    if maximize {
        let mut best_value = f64::NEG_INFINITY;
        for m in shuffled_move_list(board.generate_moves()) {
            experiment_board.apply_move(m);
            let (_, value) = alpha_beta(
                &experiment_board,
                depth - 1,
                alpha,
                beta,
                !maximize,
                verbose,
            );
            if verbose {
                println!(
                "Max: depth = {}, experiment move = {}, value = {}, best_move = {}, best_value = {}",
                depth,
                m,
                value,
                if let Some(b) = best_move {b.to_string()}else {"None".to_owned()},
                best_value
            );
            }
            experiment_board.undo_move();
            if value > best_value {
                best_value = value;
                best_move = Some(m);
            }
            alpha = alpha.max(best_value);
            if alpha >= beta {
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
                depth - 1,
                alpha,
                beta,
                !maximize,
                verbose,
            );
            if verbose {
                println!(
                "Min: depth = {}, experiment move = {}, value = {}, best_move = {}, best_value = {}",
                depth,
                m,
                value,
                if let Some(b) = best_move {b.to_string()}else {"None".to_owned()},
                best_value
            );
            }
            experiment_board.undo_move();
            if value < best_value {
                best_value = value;
                best_move = Some(m);
            }
            beta = beta.min(best_value);
            if alpha >= beta {
                break;
            }
        }
        return (best_move, best_value);
    }
}
