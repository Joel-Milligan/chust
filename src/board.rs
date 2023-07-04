use crate::piece::*;
use std::fmt::Display;

pub struct Board {
    position: [Option<Piece>; 64],
}

impl Default for Board {
    fn default() -> Self {
        let position = [
            WHITE_ROOK,
            WHITE_KNIGHT,
            WHITE_BISHOP,
            WHITE_QUEEN,
            WHITE_KING,
            WHITE_BISHOP,
            WHITE_KNIGHT,
            WHITE_ROOK,
            WHITE_PAWN,
            WHITE_PAWN,
            WHITE_PAWN,
            WHITE_PAWN,
            WHITE_PAWN,
            WHITE_PAWN,
            WHITE_PAWN,
            WHITE_PAWN,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            BLACK_PAWN,
            BLACK_PAWN,
            BLACK_PAWN,
            BLACK_PAWN,
            BLACK_PAWN,
            BLACK_PAWN,
            BLACK_PAWN,
            BLACK_PAWN,
            BLACK_ROOK,
            BLACK_KNIGHT,
            BLACK_BISHOP,
            BLACK_QUEEN,
            BLACK_KING,
            BLACK_BISHOP,
            BLACK_KNIGHT,
            BLACK_ROOK,
        ];

        Board { position }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in (0..8).rev() {
            for col in 0..8 {
                if let Some(piece) = self.position[row * 8 + col] {
                    write!(f, " {piece} ")?;
                } else {
                    write!(f, " - ")?;
                }
            }
            write!(f, "\n")?;
        }
        write!(f, "")
    }
}

impl Board {
    pub fn move_coordinate(&mut self, ply: &str) {
        if ply.len() != 4 {
            panic!("expected ply of length 4, got {}", ply.len());
        }

        let chars: Vec<char> = ply.chars().collect();

        let src_col = Board::get_column(&chars[0]);
        let src_row = Board::get_row(&chars[1]);
        let dst_col = Board::get_column(&chars[2]);
        let dst_row = Board::get_row(&chars[3]);

        self.move_piece(src_row, src_col, dst_row, dst_col);
    }

    fn get_column(c: &char) -> usize {
        match c {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => panic!("expected column from a to h, got {c}"),
        }
    }

    fn get_row(c: &char) -> usize {
        let row = (c.to_digit(10).unwrap() - 1) as usize;

        if row > 7 {
            panic!("expected row from 0 to 7, got {row}");
        }

        row
    }

    fn move_piece(&mut self, src_row: usize, src_col: usize, dst_row: usize, dst_col: usize) {
        let piece = self.position[src_row * 8 + src_col].clone();
        self.position[src_row * 8 + src_col] = None;
        self.position[dst_row * 8 + dst_col] = piece;
    }
}
