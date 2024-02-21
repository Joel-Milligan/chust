use crate::constants::*;
use crate::piece_move::Move;

use std::fmt::Display;

pub struct Board {
    bitboards: [[u64; 6]; 2],
    squares: [Option<(usize, usize)>; 64],
    turn: usize,
}

impl Default for Board {
    fn default() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rank in (0..8).rev() {
            write!(f, "{} ", rank + 1)?;
            for file in 0..8 {
                if let Some(piece) = self.squares[rank * 8 + file] {
                    let kind = match piece.1 {
                        KING => "k",
                        QUEEN => "q",
                        ROOK => "r",
                        BISHOP => "b",
                        KNIGHT => "n",
                        PAWN => "p",
                        p => panic!("unexpected piece constant: {p}"),
                    };

                    if piece.0 == WHITE {
                        write!(f, "{} ", kind.to_uppercase())?;
                    } else {
                        write!(f, "{} ", kind)?;
                    }
                } else {
                    write!(f, ". ")?;
                }
            }
            write!(f, "\n")?;
        }
        write!(f, "  a b c d e f g h")
    }
}

impl Board {
    pub fn from_fen(fen: &str) -> Self {
        let mut squares = [None; 64];
        let mut bitboards = [[0; 6]; 2];

        let fields: Vec<&str> = fen.split_whitespace().collect();
        let mut idx = 0;
        for ch in fields[0].split('/').rev().flat_map(|rank| rank.chars()) {
            if ch.is_numeric() {
                let empty_squares = ch.to_digit(10).unwrap();
                for i in idx..(idx + empty_squares) {
                    squares[i as usize] = None;
                }
                idx += empty_squares;
                continue;
            }

            let (colour, piece) = match ch {
                'K' => (WHITE, KING),
                'Q' => (WHITE, QUEEN),
                'R' => (WHITE, ROOK),
                'B' => (WHITE, BISHOP),
                'N' => (WHITE, KNIGHT),
                'P' => (WHITE, PAWN),
                'k' => (BLACK, KING),
                'q' => (BLACK, QUEEN),
                'r' => (BLACK, ROOK),
                'b' => (BLACK, BISHOP),
                'n' => (BLACK, KNIGHT),
                'p' => (BLACK, PAWN),
                ch => panic!("invalid character: {ch}"),
            };

            bitboards[colour][piece] |= 1 << idx;
            squares[idx as usize] = Some((colour, piece));

            idx += 1;
        }

        let turn = match fields[1] {
            "w" => WHITE,
            "b" => BLACK,
            colour => panic!("unexpect colour: {colour}"),
        };

        // TODO: castling availability
        // TODO: en passant square
        // TODO: halfmove clock
        // TODO: fullmove clock

        Board {
            bitboards,
            squares,
            turn,
        }
    }

    pub fn apply_move(&mut self, m: Move) {
        let piece = self.squares[m.source().0];
        self.squares[m.source().0] = None;
        self.squares[m.destination().0] = piece;

        // TODO: Handle all possible moves
        // TODO: Handle captures
        // TODO: Check if move is legal
    }
}
