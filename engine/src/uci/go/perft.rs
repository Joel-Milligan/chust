use crate::engine::Engine;
use crate::repr::board::Board;
use crate::repr::piece_move::Move;

pub fn invoke(engine: &mut Engine, tokens: &[&str]) {
    if let Some((depth, rest)) = tokens.split_first() {
        if let Some((fen, rest)) = rest.split_first() {
            engine.board = Board::from_fen(&fen).unwrap();

            if let Some((moves, _)) = rest.split_first() {
                for mv in moves.split_whitespace() {
                    engine.board.make_move(&Move::coordinate(mv))
                }
            }
        }

        if let Ok(depth) = depth.parse::<usize>() {
            engine.board.divide(depth);
        }
    }
}
