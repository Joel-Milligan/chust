use bitvec::prelude::*;

use crate::calculated::king::KING_MOVES;
use crate::calculated::knight::KNIGHT_MOVES;
use crate::calculated::pawn::{PAWN_ATTACKS, PAWN_MOVES};
use crate::constants::*;
use crate::piece_move::Move;
use crate::square::Square;

use std::fmt::Display;

pub struct Board {
    bitboards: [[u64; 6]; 2],
    /// Each square contains Option<(COLOUR, PIECE)>
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
            colour => panic!("invalid colour: {colour}"),
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

    pub fn get_legal_moves(&self, square: Square) -> Vec<Move> {
        let square = square.0;
        let (colour, piece) = match self.squares[square] {
            Some(p) => p,
            None => return vec![],
        };

        let friendly = self.bitboards[colour]
            .into_iter()
            .reduce(|acc, e| acc | e)
            .unwrap();

        let opponent_colour = if colour == WHITE { BLACK } else { WHITE };

        let opponent_pieces = self.bitboards[opponent_colour]
            .into_iter()
            .reduce(|acc, e| acc | e)
            .unwrap();

        let moves = match piece {
            KING => KING_MOVES[square],
            KNIGHT => KNIGHT_MOVES[square],
            PAWN => (PAWN_ATTACKS[colour][square] & opponent_pieces) | PAWN_MOVES[colour][square],
            _ => unimplemented!("Moves not implemented for chosen piece"),
        };

        let moves = moves & !friendly;
        let moves = moves.view_bits::<Lsb0>();

        moves
            .into_iter()
            .enumerate()
            .filter(|(_, m)| **m)
            .map(|s| Move(Square(square), Square(s.0)))
            .collect()
    }

    pub fn apply_move(&mut self, mv: Move) {
        // TODO: Check if move is legal
        // TODO: Castling
        // TODO: En Passant
        // TODO: Promotion

        // Pieces
        let source_piece = self.squares[mv.source().0].unwrap();
        let captured_piece = self.squares[mv.destination().0];

        // Squares
        self.squares[mv.source().0] = None;
        self.squares[mv.destination().0] = Some(source_piece);

        // Bitboards
        self.bitboards[source_piece.0][source_piece.1] ^=
            1 << mv.destination().0 | 1 << mv.source().0;

        if let Some(piece) = captured_piece {
            self.bitboards[piece.0][piece.1] ^= 1 << mv.destination().0;
        };

        // Turn
        if self.turn == WHITE {
            self.turn = BLACK;
        } else {
            self.turn = WHITE;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_board() {
        let board = Board::default();

        assert_eq!(board.squares[E2], Some((WHITE, PAWN)));
        assert_eq!(board.squares[A8], Some((BLACK, ROOK)));
        assert_eq!(board.squares[F6], None);

        assert_eq!(board.bitboards[WHITE][PAWN], 0xff00);
        assert_eq!(board.bitboards[BLACK][KING], 0x1000000000000000);
    }

    #[test]
    fn simple_move() {
        let mut board = Board::default();
        board.apply_move(Move::coordinate("e2e4".to_string()));

        assert_eq!(board.squares[E2], None);
        assert_eq!(board.squares[E4], Some((WHITE, PAWN)));
        assert_eq!(board.bitboards[WHITE][PAWN], 0x1000ef00);
    }

    #[test]
    fn capture() {
        let mut board =
            Board::from_fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2");
        board.apply_move(Move::coordinate("e4d5".to_string()));

        assert_eq!(board.squares[E4], None);
        assert_eq!(board.squares[D5], Some((WHITE, PAWN)));

        assert_eq!(board.bitboards[WHITE][PAWN], 0x80000ef00);
        assert_eq!(board.bitboards[BLACK][PAWN], 0xf7000000000000);
    }

    #[test]
    fn castle() {}

    #[test]
    fn en_passant() {}

    #[test]
    fn promote() {}

    #[test]
    fn illegal_move() {}

    #[test]
    fn legal_knight_moves() {
        let board = Board::from_fen("8/8/8/8/8/8/8/N7 w - - 0 1");
        let moves = board.get_legal_moves(Square(A1));
        assert!(moves.contains(&Move(Square(A1), Square(B3))));
        assert!(moves.contains(&Move(Square(A1), Square(C2))));
        assert_eq!(moves.len(), 2);

        let board = Board::default();
        let moves = board.get_legal_moves(Square(B1));
        assert!(moves.contains(&Move(Square(B1), Square(A3))));
        assert!(moves.contains(&Move(Square(B1), Square(C3))));
        assert_eq!(moves.len(), 2);

        let board = Board::from_fen("8/8/2K1K3/1K3K2/3N4/1K3K2/2K1K3/8 w - - 0 1");
        let moves = board.get_legal_moves(Square(D4));
        assert!(moves.is_empty());
    }

    #[test]
    fn legal_king_moves() {
        let board = Board::from_fen("8/8/8/8/8/8/8/K7 w - - 0 1");
        let moves = board.get_legal_moves(Square(A1));
        assert!(moves.contains(&Move(Square(A1), Square(B1))));
        assert!(moves.contains(&Move(Square(A1), Square(A2))));
        assert!(moves.contains(&Move(Square(A1), Square(B2))));
        assert_eq!(moves.len(), 3);

        let board = Board::default();
        let moves = board.get_legal_moves(Square(E1));
        assert!(moves.is_empty());

        // TODO: Castling
    }

    #[test]
    fn legal_white_pawn_moves() {
        // Starting square
        let board = Board::default();
        let moves = board.get_legal_moves(Square(E2));
        assert!(moves.contains(&Move(Square(E2), Square(E3))));
        assert!(moves.contains(&Move(Square(E2), Square(E4))));
        assert_eq!(moves.len(), 2);

        // Normal move
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/3P4/PPP1PPPP/RNBQKBNR b KQkq - 0 1");
        let moves = board.get_legal_moves(Square(D3));
        assert!(moves.contains(&Move(Square(D3), Square(D4))));
        assert_eq!(moves.len(), 1);

        // Attack
        let board = Board::from_fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2");
        let moves = board.get_legal_moves(Square(E4));
        assert!(moves.contains(&Move(Square(E4), Square(E5))));
        assert!(moves.contains(&Move(Square(E4), Square(D5))));
        assert_eq!(moves.len(), 2);

        // TODO: En passant
        // TODO: Promotion
    }

    #[test]
    fn legal_black_pawn_moves() {
        // Starting square
        let board = Board::default();
        let moves = board.get_legal_moves(Square(E7));
        assert!(moves.contains(&Move(Square(E7), Square(E5))));
        assert!(moves.contains(&Move(Square(E7), Square(E6))));
        assert_eq!(moves.len(), 2);

        // Normal move
        let board = Board::from_fen("rnbqkbnr/pppp1ppp/4p3/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2");
        let moves = board.get_legal_moves(Square(E6));
        assert!(moves.contains(&Move(Square(E6), Square(E5))));
        assert_eq!(moves.len(), 1);

        // Attack
        let board = Board::from_fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2");
        let moves = board.get_legal_moves(Square(D5));
        assert!(moves.contains(&Move(Square(D5), Square(D4))));
        assert!(moves.contains(&Move(Square(D5), Square(E4))));
        assert_eq!(moves.len(), 2);

        // TODO: En passant
        // TODO: Promotion
    }
}
