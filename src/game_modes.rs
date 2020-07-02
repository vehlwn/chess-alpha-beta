use crate::alpha_beta::get_best_move;
use crate::board_pretty_print::board_pretty_print;
use crate::game_settings::GameSettings;
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

pub fn computer_with_computer(settings: &GameSettings) {
    let mut game_board = pleco::Board::default();
    loop {
        board_pretty_print(&game_board);

        let white_best = get_best_move(&game_board, settings.depth, true);
        println!("White move = {}, value = {}", white_best.0, white_best.1);
        game_board.apply_move(white_best.0);
        if game_board.checkmate() {
            println!("Chechmate! White won!");
            break;
        } else if game_board.stalemate() {
            println!("Stalemate! Game over.");
            break;
        }

        let black_best = get_best_move(&game_board, settings.depth, false);
        println!("black move = {}, value = {}", black_best.0, black_best.1);
        game_board.apply_move(black_best.0);
        if game_board.checkmate() {
            println!("Chechmate! Black won!");
            break;
        } else if game_board.stalemate() {
            println!("Stalemate! Game over.");
            break;
        }
    }
}

pub fn white_user_with_black_computer(settings: &GameSettings) {
    let mut game_board = pleco::Board::default();
    loop {
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
        if settings.evaluate_user {
            let white_best = get_best_move(&game_board, settings.depth, true);
            println!(
                "White best move = {}, value = {}",
                white_best.0, white_best.1
            );
        }
        loop {
            let user_move = input("Type white move: ");
            if user_move == "u" {
                if game_board.ply() >= 2 {
                    println!("Undoing...");
                    game_board.undo_move();
                    game_board.undo_move();
                    board_pretty_print(&game_board);
                } else {
                    println!("Cannot be undone");
                }
                continue;
            }
            let b = game_board.apply_uci_move(&user_move);
            if !b {
                println!("Invalid move. try again.");
                continue;
            }
            break;
        }
        if game_board.checkmate() {
            println!("Chechmate! White won!");
            break;
        } else if game_board.stalemate() {
            println!("Stalemate! Game over.");
            break;
        }

        let black_best = get_best_move(&game_board, settings.depth, false);
        println!("black move = {}, value = {}", black_best.0, black_best.1);
        game_board.apply_move(black_best.0);
        if game_board.checkmate() {
            println!("Chechmate! Black won!");
            break;
        } else if game_board.stalemate() {
            println!("Stalemate! Game over.");
            break;
        }
    }
}

pub fn black_user_with_white_computer(settings: &GameSettings) {
    let mut game_board = pleco::Board::default();
    loop {
        let white_best = get_best_move(&game_board, settings.depth, true);
        println!("white move = {}, value = {}", white_best.0, white_best.1);
        game_board.apply_move(white_best.0);
        board_pretty_print(&game_board);
        if game_board.checkmate() {
            println!("Chechmate! White won!");
            break;
        } else if game_board.stalemate() {
            println!("Stalemate! Game over.");
            break;
        }

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
        if settings.evaluate_user {
            let black_best = get_best_move(&game_board, settings.depth, false);
            println!(
                "Black best move = {}, value = {}",
                black_best.0, black_best.1
            );
        }
        loop {
            let user_move = input("Type black move: ");
            if user_move == "u" {
                if game_board.ply() >= 2 {
                    println!("Undoing...");
                    game_board.undo_move();
                    game_board.undo_move();
                    board_pretty_print(&game_board);
                } else {
                    println!("Cannot be undone");
                }
                continue;
            }
            let b = game_board.apply_uci_move(&user_move);
            if !b {
                println!("Invalid move. try again.");
                continue;
            }
            break;
        }
        if game_board.checkmate() {
            println!("Chechmate! Black won!");
            break;
        } else if game_board.stalemate() {
            println!("Stalemate! Game over.");
            break;
        }
    }
}
