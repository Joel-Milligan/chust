use crate::bitboards;
use crate::piece::*;
use crate::piece_move::Move;
use std::fmt::Display;

pub struct Board {
    black_king: u64,
    black_queens: u64,
    black_rooks: u64,
    black_bishops: u64,
    black_knights: u64,
    black_pawns: u64,

    white_king: u64,
    white_queens: u64,
    white_rooks: u64,
    white_bishops: u64,
    white_knights: u64,
    white_pawns: u64,

    squares: [Option<Piece>; 64],
    turn: Colour,
}

impl Default for Board {
    fn default() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in (0..8).rev() {
            for col in 0..8 {
                if let Some(piece) = self.squares[row * 8 + col] {
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
            col => panic!("expected column from a to h, got {col}"),
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
        let piece = self.squares[src_row * 8 + src_col].clone();
        self.squares[src_row * 8 + src_col] = None;
        self.squares[dst_row * 8 + dst_col] = piece;
    }

    pub fn from_fen(fen: &str) -> Self {
        let mut squares = [None; 64];

        let mut white_king = 0u64;
        let mut white_queens = 0u64;
        let mut white_rooks = 0u64;
        let mut white_bishops = 0u64;
        let mut white_knights = 0u64;
        let mut white_pawns = 0u64;

        let mut black_king = 0u64;
        let mut black_queens = 0u64;
        let mut black_rooks = 0u64;
        let mut black_bishops = 0u64;
        let mut black_knights = 0u64;
        let mut black_pawns = 0u64;

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

            let piece = match ch {
                'K' => {
                    white_king |= 1 << idx;
                    WHITE_KING
                }
                'Q' => {
                    white_queens |= 1 << idx;
                    WHITE_QUEEN
                }
                'R' => {
                    white_rooks |= 1 << idx;
                    WHITE_ROOK
                }
                'B' => {
                    white_bishops |= 1 << idx;
                    WHITE_BISHOP
                }
                'N' => {
                    white_knights |= 1 << idx;
                    WHITE_KNIGHT
                }
                'P' => {
                    white_pawns |= 1 << idx;
                    WHITE_PAWN
                }
                'k' => {
                    black_king |= 1 << idx;
                    BLACK_KING
                }
                'q' => {
                    black_queens |= 1 << idx;
                    BLACK_QUEEN
                }
                'r' => {
                    black_rooks |= 1 << idx;
                    BLACK_ROOK
                }
                'b' => {
                    black_bishops |= 1 << idx;
                    BLACK_BISHOP
                }
                'n' => {
                    black_knights |= 1 << idx;
                    BLACK_KNIGHT
                }
                'p' => {
                    black_pawns |= 1 << idx;
                    BLACK_PAWN
                }
                ch => panic!("invalid character: {ch}"),
            };

            squares[idx as usize] = piece;

            idx += 1;
        }

        let turn = match fields[1] {
            "w" => Colour::White,
            "b" => Colour::Black,
            colour => panic!("unexpect colour: {colour}"),
        };

        // TODO: castling availability
        // TODO: en passant square
        // TODO: halfmove clock
        // TODO: fullmove clock

        Board {
            squares,
            turn,
            white_king,
            white_queens,
            white_rooks,
            white_bishops,
            white_knights,
            white_pawns,
            black_king,
            black_queens,
            black_rooks,
            black_bishops,
            black_knights,
            black_pawns,
        }
    }

    fn print_bitboard(bitboard: u64) {
        for rank in (0..8).rev() {
            print!("{} ", rank + 1);
            for file in 0..8 {
                let square = rank * 8 + file;
                let occupied = bitboard & (1u64 << square) != 0;
                print!(" {} ", if occupied { "o" } else { "." })
            }
            println!();
        }
        println!("   a  b  c  d  e  f  g  h");
    }

    pub fn generate_moves(&self) -> Vec<Move> {
        let mut moves = vec![];

        moves.append(&mut self.generate_pawn_moves());

        moves
    }

    fn generate_pawn_moves(&self) -> Vec<Move> {
        let mut moves = vec![];

        if self.turn == Colour::White {
            let pawns = bitboards::indicies(&self.white_pawns);
            for pawn in pawns.into_iter().filter(|p| p <= &55) {
                moves.push(Move(pawn, pawn + 8));
            }
        } else {
            let pawns = bitboards::indicies(&self.black_pawns);
            for pawn in pawns.into_iter().filter(|p| p >= &8) {
                moves.push(Move(pawn, pawn - 8));
            }
        }

        moves
    }
}
