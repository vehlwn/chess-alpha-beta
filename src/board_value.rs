use crate::alpha_beta::EvaluationContext;
use crate::alpha_beta::ValueType;
use pleco;

// Computer plays for black. It need to find move with minimal value.
// The more value - the more white wins.
pub fn board_value(board: &pleco::Board, context: &EvaluationContext) -> ValueType {
    return board_value_impl(board, context);
}

fn board_value_impl(board: &pleco::Board, context: &EvaluationContext) -> ValueType {
    let mut mate_score = ValueType::default();
    if board.checkmate() {
        // The less moves to mate - the more mate bonus.
        // current_depth at the start of an algorithm is equal to max_depth
        // and decreases to the leaf nodes of the solution tree.
        let moves_to_mate_bonus =
            context.current_depth as ValueType * 1.0e5 as ValueType;
        if board.turn() == pleco::Player::Black {
            mate_score = 1.0e6 as ValueType + moves_to_mate_bonus;
        } else {
            mate_score = -1.0e6 as ValueType - moves_to_mate_bonus;
        }
    }
    return if std::cmp::min(
        board.count_pieces_player(pleco::Player::White),
        board.count_pieces_player(pleco::Player::Black),
    ) < 9
    {
        board.psq().eg()
    } else {
        board.psq().mg()
    } as ValueType
        + mate_score;
}
