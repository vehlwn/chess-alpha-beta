use pleco;

// Computer plays for black. It need to find move with minimal value.
// The more value - the more white wins.
pub fn board_value(board: &pleco::Board) -> f64 {
    return board_value_impl(board);
}

fn board_value_impl(board: &pleco::Board) -> f64 {
    let mut mate_score = 0.;
    if board.checkmate() {
        if board.turn() == pleco::Player::Black {
            mate_score = 1.0e6;
        } else {
            mate_score = -1.0e6;
        }
    }
    return if std::cmp::min(
        board.count_pieces_player(pleco::Player::White),
        board.count_pieces_player(pleco::Player::Black),
    ) < 9
    {
        board.psq().mg()
    } else {
        board.psq().eg()
    } as f64
        + mate_score;
}
