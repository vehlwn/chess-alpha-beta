mod alpha_beta;
mod board_pretty_print;
mod board_value;
mod shuffled_move_list;

use alpha_beta::alpha_beta;
use board_pretty_print::board_pretty_print;
use pleco;
use std::io::Write;

fn input(promt: &str) -> String {
    let mut ret = String::new();
    print!("{}", promt);
    std::io::stdout().flush().expect("flush failed");
    std::io::stdin()
        .read_line(&mut ret)
        .expect("Failed to read line");
    return ret.trim().to_string();
}

fn get_best_move(
    board: &pleco::Board,
    depth: i32,
    maximize: bool,
    verbose: bool,
) -> pleco::BitMove {
    let (best_move, value) = alpha_beta(
        &board,
        depth,
        f64::NEG_INFINITY,
        f64::INFINITY,
        maximize,
        verbose,
    );
    println!(
        "{} value = {}",
        if maximize { "White" } else { "Black" },
        value
    );
    return best_move.unwrap();
}

fn main() {
    const DEPTH: i32 = 6;
    const VERBOSE: bool = false;
    const USER_WITH_COMPUTER: bool = true;

    let mut game_board = pleco::Board::default();
    loop {
        println!("===== {}-th move:", game_board.ply());
        board_pretty_print(&game_board);
        let mut legal_moves: Vec<String> = game_board
            .generate_moves()
            .iter()
            .map(|x| x.to_string())
            .collect();
        legal_moves.sort();
        println!(
            "legal_moves = {:?}, len = {}",
            legal_moves,
            legal_moves.len()
        );
        if USER_WITH_COMPUTER {
            loop {
                let user_move = input("Type white move: ");
                let b = game_board.apply_uci_move(&user_move);
                if !b {
                    println!("Invalid move. try again.");
                    continue;
                }
                break;
            }
        } else {
            let user_move = get_best_move(&game_board, DEPTH, true, VERBOSE);
            println!("White move = {}", user_move.to_string());
            game_board.apply_move(user_move);
        }

        if game_board.checkmate() {
            println!("Chechmate! White won!");
            break;
        } else if game_board.stalemate() {
            println!("Stalemate! Game over.");
            break;
        }

        let black_move = get_best_move(&game_board, DEPTH, false, VERBOSE);
        println!("black move = {}", black_move);
        game_board.apply_move(black_move);
        if game_board.checkmate() {
            println!("Chechmate! Black won!");
            break;
        } else if game_board.stalemate() {
            println!("Stalemate! Game over.");
            break;
        }
    }
}
