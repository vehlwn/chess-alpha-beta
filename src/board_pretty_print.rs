use pleco;

pub fn board_pretty_print(board: &pleco::Board) {
    let int_to_file = |i| match i {
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
    let int_to_rank = |i| match i {
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

    println!("===== {}-th move:", board.ply());
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
