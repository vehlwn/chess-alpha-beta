use pleco;
use rand;
use rand::seq::SliceRandom;
use std::io::Write;

fn board_pretty_print(board: &pleco::Board) {
    let int_to_file = |i| {
        return match i {
            0 => pleco::File::A,
            1 => pleco::File::B,
            2 => pleco::File::C,
            3 => pleco::File::D,
            4 => pleco::File::E,
            5 => pleco::File::F,
            6 => pleco::File::G,
            7 => pleco::File::H,
            _ => panic!("Invalid file"),
        };
    };
    let int_to_rank = |i| {
        return match i {
            0 => pleco::Rank::R1,
            1 => pleco::Rank::R2,
            2 => pleco::Rank::R3,
            3 => pleco::Rank::R4,
            4 => pleco::Rank::R5,
            5 => pleco::Rank::R6,
            6 => pleco::Rank::R7,
            7 => pleco::Rank::R8,
            _ => panic!("Invalid rank"),
        };
    };

    for row in (0..8).rev() {
        print!("{}| ", row + 1);
        for col in 0..8 {
            let square = pleco::SQ::make(int_to_file(col), int_to_rank(row));
            match board.piece_at_sq(square) {
                pleco::Piece::None => print!("."),
                p => print!("{}", p),
            }
            print!(" ");
        }
        println!();
    }
    println!("------------------");
    println!(" | a b c d e f g h");
}

fn input(promt: &str) -> String {
    let mut ret = String::new();
    print!("{}", promt);
    std::io::stdout().flush().expect("flush failed");
    std::io::stdin()
        .read_line(&mut ret)
        .expect("Failed to read line");
    return ret.trim().to_string();
}

fn shuffled_move_list(it: pleco::MoveList) -> Vec<pleco::BitMove> {
    let mut rng = rand::thread_rng();
    let mut y: Vec<pleco::BitMove> = it.iter().map(|x| *x).collect();
    y.shuffle(&mut rng);
    return y;
}

fn alpha_beta(
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

// The more value - the more white wins.
fn board_value2(board: &pleco::Board) -> f64 {
    if board.checkmate() {
        if board.turn() == pleco::Player::Black {
            return 100.;
        } else {
            return -100.;
        }
    }
    let mut total_value = 0.;
    let fen_str2 = board.fen();
    let fen_str = fen_str2.split_whitespace().next().unwrap();
    total_value += 1.
        * (fen_str.matches("P").count() as f64
            - fen_str.matches("p").count() as f64);
    total_value += 5.
        * (fen_str.matches("R").count() as f64
            - fen_str.matches("r").count() as f64);
    total_value += 3.
        * (fen_str.matches("N").count() as f64
            - fen_str.matches("n").count() as f64);
    total_value += 3.
        * (fen_str.matches("B").count() as f64
            - fen_str.matches("b").count() as f64);
    total_value += 9.
        * (fen_str.matches("Q").count() as f64
            - fen_str.matches("q").count() as f64);
    if board.in_check() {
        if board.turn() == pleco::Player::Black {
            total_value += 50.;
        } else {
            total_value += -50.;
        }
    }
    return total_value;
}

fn board_value(board: &pleco::Board) -> f64 {
    return board.psq().mg() as f64;
}

fn get_best_move(board: &pleco::Board, depth: i32, verbose: bool) -> pleco::BitMove {
    // Computer plays for black. It need to find move with minimal value.
    let (best_move, value) = alpha_beta(
        &board,
        depth,
        f64::NEG_INFINITY,
        f64::INFINITY,
        false,
        verbose,
    );
    println!("computer value = {}", value);
    return best_move.unwrap();
}

fn main() {
    const DEPTH: i32 = 6;
    const VERBOSE: bool = false;

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
        loop {
            let user_move = input("Type your move: ");
            let b = game_board.apply_uci_move(&user_move);
            if !b {
                println!("Invalid move. try again.");
                continue;
            }
            break;
        }

        if game_board.checkmate() {
            println!("Chechmate! User won!");
            break;
        } else if game_board.stalemate() {
            println!("Stalemate! Game over.");
            break;
        }

        let computer_move = get_best_move(&game_board, DEPTH, VERBOSE);
        println!("computer move = {}", computer_move);
        game_board.apply_move(computer_move);
        if game_board.checkmate() {
            println!("Chechmate! Computer won!");
            break;
        } else if game_board.stalemate() {
            println!("Stalemate! Game over.");
            break;
        }
    }
}
