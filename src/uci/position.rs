use std::collections::VecDeque;

use crate::board::Board;
use crate::engine::Engine;
use crate::piece_move::Move;

pub fn invoke(engine: &mut Engine, tokens: VecDeque<String>) {
    let tokens: Vec<String> = tokens.into();
    if !tokens.is_empty() && tokens[0] == "startpos" {
        engine.board = Board::default();

        if tokens.get(1) == Some(&"moves".to_string()) {
            let moves = tokens[2..].iter();
            for mv in moves {
                engine.board.make_move(&Move::coordinate(mv));
            }
        }
    } else if tokens.len() >= 7 && tokens[0] == "fen" {
        let fen = tokens[1..7].join(" ");
        engine.board = Board::from_fen(&fen).unwrap();

        if tokens.get(7) == Some(&"moves".to_string()) {
            let moves = tokens[8..].iter();
            for mv in moves {
                engine.board.make_move(&Move::coordinate(mv));
            }
        }
    }
}
