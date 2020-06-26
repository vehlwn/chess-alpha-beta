use pleco;
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
                pleco::Piece::BlackKing => print!("♚"),
                pleco::Piece::BlackQueen => print!("♛"),
                pleco::Piece::BlackRook => print!("♜"),
                pleco::Piece::BlackBishop => print!("♝"),
                pleco::Piece::BlackKnight => print!("♞"),
                pleco::Piece::BlackPawn => print!("♟︎"),
                pleco::Piece::WhiteKing => print!("♔"),
                pleco::Piece::WhiteQueen => print!("♕"),
                pleco::Piece::WhiteRook => print!("♖"),
                pleco::Piece::WhiteBishop => print!("♗"),
                pleco::Piece::WhiteKnight => print!("♘"),
                pleco::Piece::WhitePawn => print!("♙"),
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

fn alpha_beta(
    board: &pleco::Board,
    depth: i32,
    mut alpha: f64,
    mut beta: f64,
    maximize: bool,
) -> f64 {
    //println!("experiment_board:");
    //board_pretty_print(&board);
    if depth <= 0 {
        return board_value(&board);
    }
    if board.checkmate() {
        if maximize {
            return 100.;
        } else {
            return -100.;
        }
    }
    if board.stalemate() {
        return 0.;
    }
    let mut experiment_board = board.clone();
    if maximize {
        let mut best_value = f64::NEG_INFINITY;
        for m in board.generate_moves() {
            experiment_board.apply_move(m);
            let value = alpha_beta(&experiment_board, depth - 1, alpha, beta, !maximize);
            experiment_board.undo_move();
            best_value = best_value.max(value);
            alpha = alpha.max(best_value);
            if alpha >= beta {
                break;
            }
        }
        return best_value;
    } else {
        let mut best_value = f64::INFINITY;
        for m in board.generate_moves() {
            experiment_board.apply_move(m);
            let value = alpha_beta(&experiment_board, depth - 1, alpha, beta, !maximize);
            experiment_board.undo_move();
            best_value = best_value.min(value);
            beta = beta.min(best_value);
            if alpha >= beta {
                break;
            }
        }
        return best_value;
    }
}

// The more value - the more white wins.
fn board_value(board: &pleco::Board) -> f64 {
    let fen_str2 = board.fen();
    let fen_str = fen_str2.split_whitespace().next().unwrap();
    let pawn_diff = fen_str.matches("P").count() as f64 - fen_str.matches("p").count() as f64;
    let rook_diff = fen_str.matches("R").count() as f64 - fen_str.matches("r").count() as f64;
    let knight_diff = fen_str.matches("N").count() as f64 - fen_str.matches("n").count() as f64;
    let bishop_diff = fen_str.matches("B").count() as f64 - fen_str.matches("b").count() as f64;
    let queen_diff = fen_str.matches("Q").count() as f64 - fen_str.matches("q").count() as f64;
    return 1. * pawn_diff + 3. * bishop_diff + 3. * knight_diff + 5. * rook_diff + 9. * queen_diff;
}

fn get_best_move(board: &pleco::Board, depth: i32) -> pleco::BitMove {
    let mut min_value = f64::INFINITY;
    let mut experiment_board = board.clone();
    let mut min_move = pleco::BitMove::null();
    for m in board.generate_moves() {
        experiment_board.apply_move(m);
        let value = alpha_beta(
            &experiment_board,
            depth,
            f64::NEG_INFINITY,
            f64::INFINITY,
            false,
        );
        println!("experiment move = {}, value = {}", m, value);
        experiment_board.undo_move();
        if value < min_value {
            min_value = value;
            println!("min_value = {}", min_value);
            min_move = m;
        }
    }
    return min_move;
}

fn main() {
    let mut game_board = pleco::Board::default();
    let mut move_counter = 1;
    loop {
        println!("===== {}-th move:", move_counter);
        board_pretty_print(&game_board);
        let mut legal_moves: Vec<String> = game_board
            .generate_moves()
            .iter()
            .map(|x| x.to_string())
            .collect();
        legal_moves.sort();
        println!("legal_moves = {:?}", legal_moves);
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

        let computer_move = get_best_move(&game_board, 6);
        println!("computer move = {}", computer_move);
        game_board.apply_move(computer_move);

        if game_board.checkmate() {
            println!("Chechmate! Computer won!");
            break;
        } else if game_board.stalemate() {
            println!("Stalemate! Game over.");
            break;
        }
        move_counter += 1;
    }
}
