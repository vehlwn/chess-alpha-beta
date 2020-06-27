use pleco;

// Computer plays for black. It need to find move with minimal value.
// The more value - the more white wins.
pub fn board_value(board: &pleco::Board) -> f64 {
    return board_value_impl(board);
}

fn board_value_impl(board: &pleco::Board) -> f64 {
    if board.checkmate() {
        if board.turn() == pleco::Player::Black {
            return 1.0e6;
        } else {
            return -1.0e6;
        }
    }
    return board.psq().mg() as f64;
}

