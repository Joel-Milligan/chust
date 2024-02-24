use bitvec::prelude::*;

use crate::calculated::bishop::generate_bishop_moves;
use crate::calculated::king::KING_MOVES;
use crate::calculated::knight::KNIGHT_MOVES;
use crate::calculated::pawn::{PAWN_ATTACKS, PAWN_MOVES};
use crate::calculated::rook::generate_rook_moves;
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
        let source = square.0;
        let (colour, piece) = match self.squares[source] {
            Some(p) => p,
            None => return vec![],
        };

        let friendly_pieces = self.bitboards[colour]
            .into_iter()
            .reduce(|acc, e| acc | e)
            .unwrap();

        let opponent_colour = if colour == WHITE { BLACK } else { WHITE };
        let opponent_pieces = self.bitboards[opponent_colour]
            .into_iter()
            .reduce(|acc, e| acc | e)
            .unwrap();

        let blockers = friendly_pieces | opponent_pieces;

        let bishop_moves = generate_bishop_moves(source, blockers);
        let rook_moves = generate_rook_moves(source, blockers);

        let moves = match piece {
            BISHOP => bishop_moves,
            KING => KING_MOVES[source],
            KNIGHT => KNIGHT_MOVES[source],
            PAWN => {
                (PAWN_ATTACKS[colour][source] & opponent_pieces)
                    | (PAWN_MOVES[colour][source] & !opponent_pieces)
            }
            QUEEN => bishop_moves | rook_moves,
            ROOK => rook_moves,
            _ => panic!("Unknown piece"),
        };

        let moves = moves & !friendly_pieces;
        let moves = moves.view_bits::<Lsb0>();

        // Generate promotions
        if piece == PAWN
            && ((colour == WHITE && (A7..H7).contains(&source))
                || (colour == BLACK && (A2..H2).contains(&source)))
        {
            return moves
                .into_iter()
                .enumerate()
                .filter(|(_, m)| **m)
                .flat_map(|destination| {
                    vec![
                        Move::promotion(source, destination.0, BISHOP),
                        Move::promotion(source, destination.0, KNIGHT),
                        Move::promotion(source, destination.0, QUEEN),
                        Move::promotion(source, destination.0, ROOK),
                    ]
                })
                .collect();
        }

        moves
            .into_iter()
            .enumerate()
            .filter(|(_, m)| **m)
            .map(|destination| Move::new(source, destination.0))
            .collect()
    }

    pub fn apply_move(&mut self, mv: Move) {
        // TODO: Check if move is legal
        // TODO: Castling
        // TODO: En Passant
        // TODO: Promotion

        // Pieces
        let (colour, piece) = self.squares[mv.source.0].unwrap();
        let captured_piece = self.squares[mv.destination.0];

        // Squares
        self.squares[mv.source.0] = None;

        if let Some(promotion) = mv.promotion {
            self.squares[mv.destination.0] = Some((colour, promotion));
        } else {
            self.squares[mv.destination.0] = Some((colour, piece));
        }

        // Bitboards
        if let Some(promotion) = mv.promotion {
            self.bitboards[colour][piece] ^= 1 << mv.source.0;
            self.bitboards[colour][promotion] ^= 1 << mv.destination.0;
        } else {
            self.bitboards[colour][piece] ^= 1 << mv.destination.0 | 1 << mv.source.0;
        }

        if let Some(piece) = captured_piece {
            self.bitboards[piece.0][piece.1] ^= 1 << mv.destination.0;
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
        board.apply_move(Move::coordinate("e2e4"));

        assert_eq!(board.squares[E2], None);
        assert_eq!(board.squares[E4], Some((WHITE, PAWN)));
        assert_eq!(board.bitboards[WHITE][PAWN], 0x1000ef00);
    }

    #[test]
    fn capture() {
        let mut board =
            Board::from_fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2");
        board.apply_move(Move::coordinate("e4d5"));

        assert_eq!(board.squares[E4], None);
        assert_eq!(board.squares[D5], Some((WHITE, PAWN)));

        assert_eq!(board.bitboards[WHITE][PAWN], 0x80000ef00);
        assert_eq!(board.bitboards[BLACK][PAWN], 0xf7000000000000);
    }

    #[test]
    fn promote() {
        let mut board = Board::from_fen("8/3P4/8/8/8/8/8/8 w - - 0 1");
        board.apply_move(Move::coordinate("d7d8q"));

        assert_eq!(board.squares[D7], None);
        assert_eq!(board.squares[D8], Some((WHITE, QUEEN)));

        assert_eq!(board.bitboards[WHITE][PAWN], 0x0);
        assert_eq!(board.bitboards[WHITE][QUEEN], 0x800000000000000);

        let mut board = Board::from_fen("3q4/4P3/8/8/8/8/8/8 w - - 0 1");
        board.apply_move(Move::coordinate("e7d8n"));

        assert_eq!(board.squares[E7], None);
        assert_eq!(board.squares[D8], Some((WHITE, KNIGHT)));

        assert_eq!(board.bitboards[WHITE][PAWN], 0x0);
        assert_eq!(board.bitboards[WHITE][KNIGHT], 0x800000000000000);
        assert_eq!(board.bitboards[BLACK][QUEEN], 0x0);
    }

    #[test]
    fn castle() {}

    #[test]
    fn en_passant() {}

    #[test]
    fn legal_knight_moves() {
        let board = Board::from_fen("8/8/8/8/8/8/8/N7 w - - 0 1");
        let moves = board.get_legal_moves(Square(A1));
        assert!(moves.contains(&Move::new(A1, B3)));
        assert!(moves.contains(&Move::new(A1, C2)));
        assert_eq!(moves.len(), 2);

        let board = Board::default();
        let moves = board.get_legal_moves(Square(B1));
        assert!(moves.contains(&Move::new(B1, A3)));
        assert!(moves.contains(&Move::new(B1, C3)));
        assert_eq!(moves.len(), 2);

        let board = Board::from_fen("8/8/2K1K3/1K3K2/3N4/1K3K2/2K1K3/8 w - - 0 1");
        let moves = board.get_legal_moves(Square(D4));
        assert!(moves.is_empty());
    }

    #[test]
    fn legal_king_moves() {
        let board = Board::from_fen("8/8/8/8/8/8/8/K7 w - - 0 1");
        let moves = board.get_legal_moves(Square(A1));
        assert!(moves.contains(&Move::new(A1, A2)));
        assert!(moves.contains(&Move::new(A1, B1)));
        assert!(moves.contains(&Move::new(A1, B2)));
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
        assert!(moves.contains(&Move::new(E2, E3)));
        assert!(moves.contains(&Move::new(E2, E4)));
        assert_eq!(moves.len(), 2);

        // Normal move
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/3P4/PPP1PPPP/RNBQKBNR b KQkq - 0 1");
        let moves = board.get_legal_moves(Square(D3));
        assert!(moves.contains(&Move::new(D3, D4)));
        assert_eq!(moves.len(), 1);

        // Attack
        let board = Board::from_fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2");
        let moves = board.get_legal_moves(Square(E4));
        assert!(moves.contains(&Move::new(E4, D5)));
        assert!(moves.contains(&Move::new(E4, E5)));
        assert_eq!(moves.len(), 2);

        // Can't attack forward
        let board = Board::from_fen("rnbqkbnr/ppp1pppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2");
        let moves = board.get_legal_moves(Square(E4));
        assert!(moves.is_empty());

        // Normal promotion
        let board = Board::from_fen("8/3P4/8/8/8/8/8/8 w - - 0 1");
        let moves = board.get_legal_moves(Square(D7));
        assert!(moves.contains(&Move::promotion(D7, D8, BISHOP)));
        assert!(moves.contains(&Move::promotion(D7, D8, KNIGHT)));
        assert!(moves.contains(&Move::promotion(D7, D8, QUEEN)));
        assert!(moves.contains(&Move::promotion(D7, D8, ROOK)));
        assert_eq!(moves.len(), 4);

        // Attacking promotion
        let board = Board::from_fen("4q3/3P4/8/8/8/8/8/8 w - - 0 1");
        let moves = board.get_legal_moves(Square(D7));
        assert!(moves.contains(&Move::promotion(D7, D8, BISHOP)));
        assert!(moves.contains(&Move::promotion(D7, D8, KNIGHT)));
        assert!(moves.contains(&Move::promotion(D7, D8, QUEEN)));
        assert!(moves.contains(&Move::promotion(D7, D8, ROOK)));
        assert!(moves.contains(&Move::promotion(D7, E8, BISHOP)));
        assert!(moves.contains(&Move::promotion(D7, E8, KNIGHT)));
        assert!(moves.contains(&Move::promotion(D7, E8, QUEEN)));
        assert!(moves.contains(&Move::promotion(D7, E8, ROOK)));
        assert_eq!(moves.len(), 8);

        // TODO: En passant
    }

    #[test]
    fn legal_black_pawn_moves() {
        // Starting square
        let board = Board::default();
        let moves = board.get_legal_moves(Square(E7));
        assert!(moves.contains(&Move::new(E7, E5)));
        assert!(moves.contains(&Move::new(E7, E6)));
        assert_eq!(moves.len(), 2);

        // Normal move
        let board = Board::from_fen("rnbqkbnr/pppp1ppp/4p3/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2");
        let moves = board.get_legal_moves(Square(E6));
        assert!(moves.contains(&Move::new(E6, E5)));
        assert_eq!(moves.len(), 1);

        // Attack
        let board = Board::from_fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2");
        let moves = board.get_legal_moves(Square(D5));
        assert!(moves.contains(&Move::new(D5, D4)));
        assert!(moves.contains(&Move::new(D5, E4)));
        assert_eq!(moves.len(), 2);

        // Normal promotion
        let board = Board::from_fen("8/8/8/8/8/8/3p4/8 w - - 0 1");
        let moves = board.get_legal_moves(Square(D2));
        assert!(moves.contains(&Move::promotion(D2, D1, BISHOP)));
        assert!(moves.contains(&Move::promotion(D2, D1, KNIGHT)));
        assert!(moves.contains(&Move::promotion(D2, D1, QUEEN)));
        assert!(moves.contains(&Move::promotion(D2, D1, ROOK)));
        assert_eq!(moves.len(), 4);

        // Attacking promotion
        let board = Board::from_fen("8/8/8/8/8/8/3p4/4Q3 w - - 0 1");
        let moves = board.get_legal_moves(Square(D2));
        assert!(moves.contains(&Move::promotion(D2, D1, BISHOP)));
        assert!(moves.contains(&Move::promotion(D2, D1, KNIGHT)));
        assert!(moves.contains(&Move::promotion(D2, D1, QUEEN)));
        assert!(moves.contains(&Move::promotion(D2, D1, ROOK)));
        assert!(moves.contains(&Move::promotion(D2, E1, BISHOP)));
        assert!(moves.contains(&Move::promotion(D2, E1, KNIGHT)));
        assert!(moves.contains(&Move::promotion(D2, E1, QUEEN)));
        assert!(moves.contains(&Move::promotion(D2, E1, ROOK)));
        assert_eq!(moves.len(), 8);

        // TODO: Promotion
    }

    #[test]
    fn legal_rook_moves() {
        let board = Board::default();
        let moves = board.get_legal_moves(Square(A1));
        assert!(moves.is_empty());

        let board = Board::from_fen("8/3r4/8/8/3R3q/8/3B4/8 w - - 0 1");
        let moves = board.get_legal_moves(Square(D4));
        assert!(moves.contains(&Move::new(D4, A4)));
        assert!(moves.contains(&Move::new(D4, D3)));
        assert!(moves.contains(&Move::new(D4, D7)));
        assert!(moves.contains(&Move::new(D4, H4)));
        assert_eq!(moves.len(), 11);
    }

    #[test]
    fn legal_bishop_moves() {
        let board = Board::default();
        let moves = board.get_legal_moves(Square(C1));
        assert!(moves.is_empty());

        let board = Board::from_fen("8/6n1/1Q6/8/3B4/8/1r6/8 w - - 0 1");
        let moves = board.get_legal_moves(Square(D4));
        assert!(moves.contains(&Move::new(D4, B2)));
        assert!(moves.contains(&Move::new(D4, C5)));
        assert!(moves.contains(&Move::new(D4, G1)));
        assert!(moves.contains(&Move::new(D4, G7)));
        assert_eq!(moves.len(), 9);
    }

    #[test]
    fn legal_queen_moves() {
        let board = Board::default();
        let moves = board.get_legal_moves(Square(D1));
        assert!(moves.is_empty());

        let board = Board::from_fen("8/6n1/1R6/3K4/3Q2p1/8/1r6/8 w - - 0 1");
        let moves = board.get_legal_moves(Square(D4));
        assert!(moves.contains(&Move::new(D4, A4)));
        assert!(moves.contains(&Move::new(D4, B2)));
        assert!(moves.contains(&Move::new(D4, C5)));
        assert!(moves.contains(&Move::new(D4, D1)));
        assert!(moves.contains(&Move::new(D4, G1)));
        assert!(moves.contains(&Move::new(D4, G4)));
        assert!(moves.contains(&Move::new(D4, G7)));
        assert_eq!(moves.len(), 18);
    }
}
