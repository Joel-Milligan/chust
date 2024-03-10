use std::collections::VecDeque;

use crate::board::Board;
use crate::engine::Engine;
use crate::piece_move::Move;

pub fn invoke(engine: &mut Engine, mut tokens: VecDeque<String>) {
    if let Some(depth) = tokens.pop_front() {
        if let Some(fen) = tokens.pop_front() {
            engine.board = Board::from_fen(&fen).unwrap();
        }

        if let Some(moves) = tokens.pop_front() {
            for mv in moves.split_whitespace() {
                engine.board.make_move(&Move::coordinate(mv))
            }
        }

        if let Ok(depth) = depth.parse::<usize>() {
            engine.board.divide(depth);
        }
    }
}
