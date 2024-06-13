use crate::alpha_beta::get_best_move;
use crate::board_pretty_print::board_pretty_print;
use crate::config::Config;
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

enum UserCommand {
    MakeMove(String),
    Undo,
    ChangeDepth(std::num::NonZeroU32),
    ChangeEvaluateUser(bool),
}

fn input_user_command(promt: &str) -> Option<UserCommand> {
    let s = input(promt);
    {
        let re = regex::Regex::new(r"d\s+(\d+)").unwrap();
        if let Some(caps) = re.captures(&s) {
            let depth =
                caps.get(1)?.as_str().parse::<std::num::NonZeroU32>().ok()?;
            return Some(UserCommand::ChangeDepth(depth));
        }
    }
    {
        let re = regex::Regex::new(r"e\s+(0|1)").unwrap();
        if let Some(caps) = re.captures(&s) {
            let e = caps.get(1)?.as_str().parse::<i32>().ok()? != 0;
            return Some(UserCommand::ChangeEvaluateUser(e));
        }
    }
    {
        if s == "u" {
            return Some(UserCommand::Undo);
        }
    }
    return Some(UserCommand::MakeMove(s));
}

fn handle_user_move(game_board: &mut pleco::Board, config: &mut Config) {
    loop {
        let user_move = input_user_command(&format!(
            "Type {} move: ",
            if game_board.turn() == pleco::Player::White {
                "white"
            } else {
                "black"
            }
        ));
        if user_move.is_none() {
            println!("Invalid command. Try again.");
            continue;
        }
        match user_move.unwrap() {
            UserCommand::Undo => {
                if game_board.ply() >= 2 {
                    println!("Undoing...");
                    game_board.undo_move();
                    game_board.undo_move();
                    board_pretty_print(&game_board);
                    continue;
                } else {
                    println!("Cannot be undone");
                    continue;
                }
            }
            UserCommand::MakeMove(user_move) => {
                let b = game_board.apply_uci_move(&user_move);
                if !b {
                    println!("Invalid move. try again.");
                    continue;
                }
                break;
            }
            UserCommand::ChangeDepth(d) => {
                println!("depth = {}", d);
                config.depth = d;
                continue;
            }
            UserCommand::ChangeEvaluateUser(e) => {
                println!("evaluate_user = {}", e);
                config.evaluate_user = e;
                continue;
            }
        }
    }
}

pub fn computer_with_computer(config: Config) {
    let mut game_board = pleco::Board::default();
    loop {
        board_pretty_print(&game_board);

        let white_best = get_best_move(&game_board, config.depth).unwrap();
        println!(
            "White move = {}, value = {}",
            white_best.m, white_best.value
        );
        game_board.apply_move(white_best.m);
        if game_board.checkmate() {
            println!("Chechmate! White won!");
            break;
        } else if game_board.stalemate() {
            println!("Stalemate! Game over.");
            break;
        }

        let black_best = get_best_move(&game_board, config.depth).unwrap();
        println!(
            "black move = {}, value = {}",
            black_best.m, black_best.value
        );
        game_board.apply_move(black_best.m);
        if game_board.checkmate() {
            println!("Chechmate! Black won!");
            break;
        } else if game_board.stalemate() {
            println!("Stalemate! Game over.");
            break;
        }
    }
}

pub fn white_user_with_black_computer(mut config: Config) {
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
        if config.evaluate_user {
            let white_best = get_best_move(&game_board, config.depth).unwrap();
            println!(
                "White best move = {}, value = {}",
                white_best.m, white_best.value
            );
        }
        handle_user_move(&mut game_board, &mut config);
        if game_board.checkmate() {
            println!("Chechmate! White won!");
            break;
        } else if game_board.stalemate() {
            println!("Stalemate! Game over.");
            break;
        }

        let black_best = get_best_move(&game_board, config.depth).unwrap();
        println!(
            "black move = {}, value = {}",
            black_best.m, black_best.value
        );
        game_board.apply_move(black_best.m);
        if game_board.checkmate() {
            println!("Chechmate! Black won!");
            break;
        } else if game_board.stalemate() {
            println!("Stalemate! Game over.");
            break;
        }
    }
}

pub fn black_user_with_white_computer(mut config: Config) {
    let mut game_board = pleco::Board::default();
    loop {
        let white_best = get_best_move(&game_board, config.depth).unwrap();
        println!(
            "white move = {}, value = {}",
            white_best.m, white_best.value
        );
        game_board.apply_move(white_best.m);
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
        if config.evaluate_user {
            let black_best = get_best_move(&game_board, config.depth).unwrap();
            println!(
                "Black best move = {}, value = {}",
                black_best.m, black_best.value
            );
        }
        handle_user_move(&mut game_board, &mut config);
        if game_board.checkmate() {
            println!("Chechmate! Black won!");
            break;
        } else if game_board.stalemate() {
            println!("Stalemate! Game over.");
            break;
        }
    }
}
