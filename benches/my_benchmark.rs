use chess_alpha_beta::alpha_beta::{EvaluatedMove, EvaluationContext, ValueType};
use chess_alpha_beta::board_value::board_value;

use criterion::{criterion_group, criterion_main, Criterion};

mod minimax {
    use super::*;

    fn orig_minimax(board: &pleco::Board, context: EvaluationContext) -> ValueType {
        if context.depth == 0 || board.checkmate() || board.stalemate() {
            return board_value(&board, context.depth);
        }

        let mut experiment_board = board.clone();

        // If maximizer (white) player
        if board.turn() == pleco::Player::White {
            let mut best_value = -10_000_000;
            for m in board.generate_moves() {
                experiment_board.apply_move(m);
                best_value = best_value.max(orig_minimax(
                    &experiment_board,
                    EvaluationContext {
                        depth: context.depth - 1,
                        ..context
                    },
                ));
                experiment_board.undo_move();
            }
            return best_value;
        } else {
            let mut best_value = 10_000_000;
            for m in board.generate_moves() {
                experiment_board.apply_move(m);
                best_value = best_value.min(orig_minimax(
                    &experiment_board,
                    EvaluationContext {
                        depth: context.depth - 1,
                        ..context
                    },
                ));
                experiment_board.undo_move();
            }
            return best_value;
        }
    }

    fn alpha_beta_minimax(
        board: &pleco::Board,
        mut context: EvaluationContext,
    ) -> ValueType {
        if context.depth == 0 || board.checkmate() || board.stalemate() {
            return board_value(&board, context.depth);
        }

        let mut experiment_board = board.clone();

        if board.turn() == pleco::Player::White {
            let mut best_value = -10_000_000;
            for m in board.generate_moves() {
                experiment_board.apply_move(m);
                best_value = best_value.max(alpha_beta_minimax(
                    &experiment_board,
                    EvaluationContext {
                        depth: context.depth - 1,
                        alpha: context.alpha,
                        beta: context.beta,
                        ..context
                    },
                ));
                experiment_board.undo_move();
                context.alpha = context.alpha.max(best_value);
                if context.alpha >= context.beta {
                    break;
                }
            }
            return best_value;
        } else {
            let mut best_value = 10_000_000;
            for m in board.generate_moves() {
                experiment_board.apply_move(m);
                best_value = best_value.min(alpha_beta_minimax(
                    &experiment_board,
                    EvaluationContext {
                        depth: context.depth - 1,
                        alpha: context.alpha,
                        beta: context.beta,
                        ..context
                    },
                ));
                experiment_board.undo_move();
                context.beta = context.beta.min(best_value);
                if context.alpha >= context.beta {
                    break;
                }
            }
            return best_value;
        }
    }

    fn find_best_move<F: Fn(&pleco::Board, EvaluationContext) -> ValueType>(
        board: &pleco::Board,
        depth: std::num::NonZeroU32,
        minimax_cb: F,
    ) -> EvaluatedMove {
        let mut experiment_board = board.clone();
        let mut best_move = None;
        let mut best_value;
        let context = EvaluationContext {
            depth: depth.get() - 1,
            alpha: ValueType::MIN,
            beta: ValueType::MAX,
            ..Default::default()
        };
        if board.turn() == pleco::Player::White {
            best_value = -10_000_000;
            for m in board.generate_moves() {
                experiment_board.apply_move(m);
                let value = minimax_cb(&experiment_board, context.clone());
                if value > best_value {
                    best_value = value;
                    best_move = Some(m);
                }
                experiment_board.undo_move();
            }
        } else {
            best_value = 10_000_000;
            for m in board.generate_moves() {
                experiment_board.apply_move(m);
                let value = minimax_cb(&experiment_board, context.clone());
                if value < best_value {
                    best_value = value;
                    best_move = Some(m);
                }
                experiment_board.undo_move();
            }
        }
        return EvaluatedMove {
            m: best_move.unwrap(),
            value: best_value,
        };
    }

    pub fn find_best_move_minimax(
        board: &pleco::Board,
        depth: std::num::NonZeroU32,
    ) -> EvaluatedMove {
        return find_best_move(board, depth, &orig_minimax);
    }

    pub fn find_best_move_minimax_alpha_beta(
        board: &pleco::Board,
        depth: std::num::NonZeroU32,
    ) -> EvaluatedMove {
        return find_best_move(board, depth, &alpha_beta_minimax);
    }
}

mod negamax {
    use super::*;

    fn orig_negamax(board: &pleco::Board, context: EvaluationContext) -> ValueType {
        if context.depth == 0 || board.checkmate() || board.stalemate() {
            let color = match board.turn() {
                pleco::Player::White => 1,
                pleco::Player::Black => -1,
            };
            return color * board_value(&board, context.depth);
        }

        let mut experiment_board = board.clone();
        let mut best_value = -10_000_000;

        for m in board.generate_moves() {
            experiment_board.apply_move(m);
            best_value = best_value.max(-orig_negamax(
                &experiment_board,
                EvaluationContext {
                    depth: context.depth - 1,
                    ..context
                },
            ));
            experiment_board.undo_move();
        }
        return best_value;
    }

    fn negamax_alpha_beta(
        board: &pleco::Board,
        mut context: EvaluationContext,
    ) -> ValueType {
        if context.depth == 0 || board.checkmate() || board.stalemate() {
            let color = match board.turn() {
                pleco::Player::White => 1,
                pleco::Player::Black => -1,
            };
            return color * board_value(&board, context.depth);
        }

        let mut experiment_board = board.clone();
        let mut best_value = -10_000_000;

        for m in board.generate_moves() {
            experiment_board.apply_move(m);
            best_value = best_value.max(-negamax_alpha_beta(
                &experiment_board,
                EvaluationContext {
                    depth: context.depth - 1,
                    alpha: -context.beta,
                    beta: -context.alpha,
                    ..context
                },
            ));
            experiment_board.undo_move();
            context.alpha = context.alpha.max(best_value);
            if context.alpha >= context.beta {
                break;
            }
        }
        return best_value;
    }

    fn find_best_move<F: Fn(&pleco::Board, EvaluationContext) -> ValueType>(
        board: &pleco::Board,
        depth: std::num::NonZeroU32,
        negamax_cb: F,
    ) -> EvaluatedMove {
        let mut experiment_board = board.clone();
        let mut best_move = None;
        let mut best_value = -10_000_000;
        let context = EvaluationContext {
            depth: depth.get() - 1,
            alpha: -10_000_000,
            beta: 10_000_000,
            ..Default::default()
        };
        for m in board.generate_moves() {
            experiment_board.apply_move(m);
            let value = -negamax_cb(&experiment_board, context.clone());
            if value > best_value {
                best_value = value;
                best_move = Some(m);
            }
            experiment_board.undo_move();
        }
        if board.turn() == pleco::Player::Black {
            best_value = -best_value;
        }
        return EvaluatedMove {
            m: best_move.unwrap(),
            value: best_value,
        };
    }

    pub fn find_best_move_negamax(
        board: &pleco::Board,
        depth: std::num::NonZeroU32,
    ) -> EvaluatedMove {
        return find_best_move(board, depth, orig_negamax);
    }

    pub fn find_best_move_negamax_alpha_beta(
        board: &pleco::Board,
        depth: std::num::NonZeroU32,
    ) -> EvaluatedMove {
        return find_best_move(board, depth, negamax_alpha_beta);
    }
}

const DEPTH: u32 = 5;
const N: u32 = 10;
const CORRECT_BOARD: &str =
    "r1bqkbnr/1pp2ppp/p1n5/3p4/4p3/2NP1N2/PPP1PPPP/R1BQKB1R w KQkq - 0 6";

fn bench_orig_minimax() {
    let mut board = pleco::Board::start_pos();
    for _ in 0..N {
        let best_move =
            minimax::find_best_move_minimax(&board, DEPTH.try_into().unwrap());
        board.apply_move(best_move.m);
    }
    assert_eq!(board.fen(), CORRECT_BOARD);
}

fn bench_minimax_alpha_beta() {
    let mut board = pleco::Board::start_pos();
    for _ in 0..N {
        let best_move = minimax::find_best_move_minimax_alpha_beta(
            &board,
            DEPTH.try_into().unwrap(),
        );
        board.apply_move(best_move.m);
    }
    assert_eq!(board.fen(), CORRECT_BOARD);
}

fn bench_orig_negamax() {
    let mut board = pleco::Board::start_pos();
    for _ in 0..N {
        let best_move =
            negamax::find_best_move_negamax(&board, DEPTH.try_into().unwrap());
        board.apply_move(best_move.m);
    }
    assert_eq!(board.fen(), CORRECT_BOARD);
}

fn bench_negamax_alpha_beta() {
    let mut board = pleco::Board::start_pos();
    for _ in 0..N {
        let best_move = negamax::find_best_move_negamax_alpha_beta(
            &board,
            DEPTH.try_into().unwrap(),
        );
        board.apply_move(best_move.m);
    }
    assert_eq!(board.fen(), CORRECT_BOARD);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("orig_minimax", |b| b.iter(bench_orig_minimax));
    c.bench_function("minimax_alpha_beta", |b| b.iter(bench_minimax_alpha_beta));
    c.bench_function("orig_negamax", |b| b.iter(bench_orig_negamax));
    c.bench_function("negamax_alpha_beta", |b| b.iter(bench_negamax_alpha_beta));
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = criterion_benchmark
}
criterion_main!(benches);
