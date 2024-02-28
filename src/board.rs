use bitvec::prelude::*;
use thiserror::Error;

use crate::calculated::bishop::generate_bishop_moves;
use crate::calculated::king::{generate_king_moves, KING_MOVES};
use crate::calculated::knight::generate_knight_moves;
use crate::calculated::pawn::{generate_pawn_moves, PAWN_ATTACKS};
use crate::calculated::rook::generate_rook_moves;
use crate::constants::*;
use crate::piece_move::Move;
use crate::square::Square;

use std::fmt::Display;
use std::str::FromStr;

#[derive(Error, Debug)]
#[error("poorly formatted fen string")]
pub struct FenError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Board {
    bitboards: [[u64; 6]; 2],
    /// Each square contains Option<(COLOUR, PIECE)>
    squares: [Option<(usize, usize)>; 64],
    active_colour: usize,
    castling: u8,
    en_passant: Option<usize>,
    /// Number of half moves since last capture or pawn push, used for the fifty-move rule
    half_moves: usize,
    full_moves: usize,
    previous_position: Option<Box<Board>>
}

impl Default for Board {
    fn default() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
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
    pub fn from_fen(fen: &str) -> Result<Self, FenError> {
        let mut squares = [None; 64];
        let mut bitboards = [[0; 6]; 2];

        let fields: Vec<&str> = fen.split_whitespace().collect();

        if fields.len() != 6 {
            return Err(FenError);
        }

        let mut idx = 0;
        for ch in fields[0].split('/').rev().flat_map(|rank| rank.chars()) {
            if ch.is_numeric() {
                let empty_squares = ch.to_digit(10).ok_or(FenError)?;
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
                _ => return Err(FenError),
            };

            bitboards[colour][piece] |= 1 << idx;
            squares[idx as usize] = Some((colour, piece));

            idx += 1;
        }

        let active_colour = match fields[1] {
            "w" => WHITE,
            "b" => BLACK,
            _ => return Err(FenError),
        };

        let castling = fields[2]
            .chars()
            .filter(|c| c != &'-')
            .fold(0u8, |acc, c| match c {
                'K' => acc | WHITE_KING_SIDE,
                'Q' => acc | WHITE_QUEEN_SIDE,
                'k' => acc | BLACK_KING_SIDE,
                'q' => acc | BLACK_QUEEN_SIDE,
                c => panic!("unknown castling character: {c}")
            });

        let en_passant = match fields[3] {
            "-" => None,
            square => Some(Square::from_str(square).map_err(|_| FenError)?.0),
        };

        let half_moves = fields[4].parse().map_err(|_| FenError)?;
        let full_moves = fields[5].parse().map_err(|_| FenError)?;

        Ok(Board {
            bitboards,
            squares,
            active_colour,
            castling,
            en_passant,
            half_moves,
            full_moves,
            previous_position: None,
        })
    }

    pub fn moves(&self) -> Vec<Move> {
        self.pseudo_legal_moves()
            .into_iter()
            .filter(|candidate| {
                let mut board = self.clone();
                board.make_move(candidate.clone());
                !board.pseudo_legal_moves()
                    .into_iter()
                    .any(|mv| board.squares[mv.destination.0] == Some((self.active_colour, KING)))
            })
            .collect()
    }

    fn pseudo_legal_moves(&self) -> Vec<Move> {
        self.squares
            .into_iter()
            .enumerate()
            .filter(|(_, s)| s.is_some_and(|(c, _)| c == self.active_colour))
            .flat_map(|(i, _)| self.pseudo_legal_moves_square(Square(i)))
            .collect()
    }

    fn pseudo_legal_moves_square(&self, square: Square) -> Vec<Move> {
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

        let pawn_attacks = PAWN_ATTACKS[colour][source] & (opponent_pieces | self.en_passant.map_or(0, |s| 1 << s));

        let moves = match piece {
            BISHOP => generate_bishop_moves(source, blockers),
            KING => generate_king_moves(source, blockers, colour, self.attacked_squares(opponent_colour), self.castling),
            KNIGHT => generate_knight_moves(source),
            PAWN => pawn_attacks | (generate_pawn_moves(source, blockers, colour) & !opponent_pieces),
            QUEEN => generate_bishop_moves(source, blockers) | generate_rook_moves(source, blockers),
            ROOK => generate_rook_moves(source, blockers),
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

    pub fn make_move(&mut self, mv: Move) {
        // Save current position for unmaking moves
        self.previous_position = Some(Box::new(self.clone()));

        // Pieces
        let (colour, piece) = self.squares[mv.source.0].unwrap();
        let captured_piece = self.squares[mv.destination.0];

        let opponent_colour = if colour == WHITE {
            BLACK
        } else {
            WHITE
        };

        // Squares
        self.squares[mv.source.0] = None;

        if let Some(promotion) = mv.promotion {
            self.squares[mv.destination.0] = Some((colour, promotion));
        } else {
            self.squares[mv.destination.0] = Some((colour, piece));
        }

        // Castling
        if piece == KING && mv.source.0.abs_diff(mv.destination.0) == 2 {
            match mv.destination.0 {
                G1 => {
                    self.squares[H1] = None;
                    self.squares[F1] = Some((WHITE, ROOK));
                    self.bitboards[colour][ROOK] ^= 1 << F1 | 1 << H1;
                },
                C1 => {
                    self.squares[A1] = None;
                    self.squares[D1] = Some((WHITE, ROOK));
                    self.bitboards[colour][ROOK] ^= 1 << D1 | 1 << A1;
                },
                G8 => {
                    self.squares[H8] = None;
                    self.squares[F8] = Some((BLACK, ROOK));
                    self.bitboards[colour][ROOK] ^= 1 << F8 | 1 << H8;
                },
                C8 => {
                    self.squares[A8] = None;
                    self.squares[D8] = Some((BLACK, ROOK));
                    self.bitboards[colour][ROOK] ^= 1 << D8 | 1 << A8;
                },
                s => panic!("can't castle to square: {}", Square(s))
            }
        }

        // Moving king prevents castling
        if piece == KING {
            if colour == WHITE {
                self.castling &= !(WHITE_KING_SIDE | WHITE_QUEEN_SIDE);
            } else {
                self.castling &= !(BLACK_KING_SIDE | BLACK_QUEEN_SIDE);
            }
        }
        
        // Moving rook prevents castling
        if piece == ROOK {
            if self.castling & WHITE_KING_SIDE != 0 && mv.source.0 == H1 {
                self.castling ^= WHITE_KING_SIDE;
            } else if self.castling & WHITE_QUEEN_SIDE != 0 && mv.source.0 == A1 {
                self.castling ^= WHITE_QUEEN_SIDE;
            } else if self.castling & BLACK_KING_SIDE != 0 && mv.source.0 == H8 {
                self.castling ^= BLACK_KING_SIDE;
            } else if self.castling & BLACK_QUEEN_SIDE != 0 && mv.source.0 == A8 {
                self.castling ^= BLACK_QUEEN_SIDE;
            }
        }

        // Captured rook prevents castling
        if self.castling & WHITE_KING_SIDE != 0 && mv.destination.0 == H1 {
            self.castling ^= WHITE_KING_SIDE;
        } else if self.castling & WHITE_QUEEN_SIDE != 0 && mv.destination.0 == A1 {
            self.castling ^= WHITE_QUEEN_SIDE;
        } else if self.castling & BLACK_KING_SIDE != 0 && mv.destination.0 == H8 {
            self.castling ^= BLACK_KING_SIDE;
        } else if self.castling & BLACK_QUEEN_SIDE != 0 && mv.destination.0 == A8 {
            self.castling ^= BLACK_QUEEN_SIDE;
        }

        // En passant
        if piece == PAWN && self.en_passant.is_some_and(|ep| mv.destination.0 == ep) {
            if colour == WHITE {
                self.squares[mv.destination.0 - 8] = None;
                self.bitboards[BLACK][PAWN] ^= 1 << mv.destination.0 - 8;
            } else {
                self.squares[mv.destination.0 + 8] = None;
                self.bitboards[WHITE][PAWN] ^= 1 << mv.destination.0 + 8;
            }
        }

        if piece == PAWN && mv.source.0.abs_diff(mv.destination.0) == 16 {
            if colour == WHITE {
                self.en_passant = Some(mv.source.0 + 8);
            } else {
                self.en_passant = Some(mv.source.0 - 8);
            }
        } else {
            self.en_passant = None;
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

        if self.active_colour == BLACK {
            self.full_moves += 1;
        }
        self.active_colour = opponent_colour;

        if captured_piece.is_some() || piece == PAWN {
            self.half_moves = 0;
        } else {
            self.half_moves += 1;
        }
    }

    pub fn unmake_move(&mut self) {
        if let Some(previous) = &self.previous_position {
            *self = *previous.clone();
        }
    }

    pub fn attacked_squares(&self, colour: usize) -> u64 {
        self.squares
            .into_iter()
            .enumerate()
            .filter(|(_, s)| s.is_some_and(|(c, _)| c == colour))
            .map(|(square, s)| {
                match s.unwrap().1 {
                BISHOP => generate_bishop_moves(square, self.blockers()),
                KING => KING_MOVES[square],
                KNIGHT => generate_knight_moves(square),
                PAWN => PAWN_ATTACKS[colour][square],
                QUEEN => generate_bishop_moves(square, self.blockers()) | generate_rook_moves(square, self.blockers()),
                ROOK => generate_rook_moves(square, self.blockers()),
                _ => panic!("unknown piece"),
            }})
            .reduce(|acc, e| acc | e)
            .unwrap_or(0)
    }

    fn blockers(&self) -> u64 {
        self.bitboards
            .into_iter()
            .flatten()
            .reduce(|acc, e| acc | e)
            .unwrap_or(0)
    }

    pub fn perft(&mut self, depth: usize) -> usize {
        let moves = self.moves();

        if depth == 0 {
            1
        } else if depth == 1 {
            moves.len()
        } else {
            let mut nodes = 0;

            for mv in moves {
                self.make_move(mv);
                nodes += self.perft(depth - 1);
                self.unmake_move();
            }

            nodes
        }
    }

    pub fn divide(&mut self, depth: usize) {
        let moves = self.moves();
        let mut nodes = 0;

        for mv in &moves {
            self.make_move(mv.clone());

            let perft = self.perft(depth - 1);
            nodes += perft;
            println!("{} {}", mv, perft);

            self.unmake_move();
        }

        println!("\n{}", nodes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_default() {
        let board = Board::default();

        // Squares
        assert_eq!(board.squares[E2], Some((WHITE, PAWN)));
        assert_eq!(board.squares[A8], Some((BLACK, ROOK)));
        assert_eq!(board.squares[F6], None);

        // Bitboards
        assert_eq!(board.bitboards[WHITE][PAWN], 0xff00);
        assert_eq!(board.bitboards[BLACK][KING], 0x1000000000000000);

        // Active Colour
        assert_eq!(board.active_colour, WHITE);

        // En passant square
        assert_eq!(board.en_passant, None);
        
        // Half moves
        assert_eq!(board.half_moves, 0);

        // Full moves
        assert_eq!(board.full_moves, 1);
    }

    #[test]
    fn board_from_fen() {
        let board = Board::from_fen("rn3rk1/p3qpp1/1p2b2p/2pp4/3P4/3BPN2/PP3PPP/R2Q1RK1 w - c6 0 13").unwrap();
        assert_eq!(board.active_colour, WHITE);
        assert_eq!(board.en_passant, Some(C6));
        assert_eq!(board.half_moves, 0);
        assert_eq!(board.full_moves, 13);

        let board = Board::from_fen("rnb2rk1/p1p1qpp1/1p5p/3p4/3P4/3BPN2/PP3PPP/R2QK2R b KQ - 1 11").unwrap();
        assert_eq!(board.active_colour, BLACK);
        assert_eq!(board.en_passant, None);
        assert_eq!(board.half_moves, 1);
        assert_eq!(board.full_moves, 11);
    }

    #[test]
    fn simple_move() {
        let mut board = Board::default();
        board.make_move(Move::coordinate("e2e4"));

        assert_eq!(board.squares[E2], None);
        assert_eq!(board.squares[E4], Some((WHITE, PAWN)));
        assert_eq!(board.bitboards[WHITE][PAWN], 0x1000ef00);
    }

    #[test]
    fn capture() {
        let mut board =
            Board::from_fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2").unwrap();
        board.make_move(Move::coordinate("e4d5"));

        assert_eq!(board.squares[E4], None);
        assert_eq!(board.squares[D5], Some((WHITE, PAWN)));

        assert_eq!(board.bitboards[WHITE][PAWN], 0x80000ef00);
        assert_eq!(board.bitboards[BLACK][PAWN], 0xf7000000000000);
    }

    #[test]
    fn promote() {
        let mut board = Board::from_fen("8/3P4/8/8/8/8/8/8 w - - 0 1").unwrap();
        board.make_move(Move::coordinate("d7d8q"));

        assert_eq!(board.squares[D7], None);
        assert_eq!(board.squares[D8], Some((WHITE, QUEEN)));

        assert_eq!(board.bitboards[WHITE][PAWN], 0x0);
        assert_eq!(board.bitboards[WHITE][QUEEN], 0x800000000000000);

        let mut board = Board::from_fen("3q4/4P3/8/8/8/8/8/8 w - - 0 1").unwrap();
        board.make_move(Move::coordinate("e7d8n"));

        assert_eq!(board.squares[E7], None);
        assert_eq!(board.squares[D8], Some((WHITE, KNIGHT)));

        assert_eq!(board.bitboards[WHITE][PAWN], 0x0);
        assert_eq!(board.bitboards[WHITE][KNIGHT], 0x800000000000000);
        assert_eq!(board.bitboards[BLACK][QUEEN], 0x0);
    }

    #[test]
    fn castle() {
        let mut board = Board::from_fen("8/8/8/8/8/8/8/4K2R w - - 0 1").unwrap();
        board.make_move(Move::coordinate("e1g1"));
        assert_eq!(board.squares[E1], None);
        assert_eq!(board.squares[F1], Some((WHITE, ROOK)));
        assert_eq!(board.squares[G1], Some((WHITE, KING)));
        assert_eq!(board.squares[H1], None);
        assert_eq!(board.bitboards[WHITE][KING], 0x40);
        assert_eq!(board.bitboards[WHITE][ROOK], 0x20);

        let mut board = Board::from_fen("8/8/8/8/8/8/8/R3K3 w - - 0 1").unwrap();
        board.make_move(Move::coordinate("e1c1"));
        assert_eq!(board.squares[E1], None);
        assert_eq!(board.squares[D1], Some((WHITE, ROOK)));
        assert_eq!(board.squares[C1], Some((WHITE, KING)));
        assert_eq!(board.squares[A1], None);
        assert_eq!(board.bitboards[WHITE][KING], 0x4);
        assert_eq!(board.bitboards[WHITE][ROOK], 0x8);

        let mut board = Board::from_fen("4k2r/8/8/8/8/8/8/8 b - - 0 1").unwrap();
        board.make_move(Move::coordinate("e8g8"));
        assert_eq!(board.squares[E8], None);
        assert_eq!(board.squares[F8], Some((BLACK, ROOK)));
        assert_eq!(board.squares[G8], Some((BLACK, KING)));
        assert_eq!(board.squares[H8], None);
        assert_eq!(board.bitboards[BLACK][KING], 0x4000000000000000);
        assert_eq!(board.bitboards[BLACK][ROOK], 0x2000000000000000);

        let mut board = Board::from_fen("r3k3/8/8/8/8/8/8/8 b - - 0 1").unwrap();
        board.make_move(Move::coordinate("e8c8"));
        assert_eq!(board.squares[E8], None);
        assert_eq!(board.squares[D8], Some((BLACK, ROOK)));
        assert_eq!(board.squares[C8], Some((BLACK, KING)));
        assert_eq!(board.squares[A8], None);
        assert_eq!(board.bitboards[BLACK][KING], 0x400000000000000);
        assert_eq!(board.bitboards[BLACK][ROOK], 0x800000000000000);
    }

    #[test]
    fn en_passant() {
        let mut board = Board::from_fen("8/3p4/8/4P3/8/8/8/8 b - - 0 1").unwrap();
        board.make_move(Move::coordinate("d7d5"));
        board.make_move(Move::coordinate("e5d6"));
        assert_eq!(board.squares[D5], None);
        assert_eq!(board.squares[D6], Some((WHITE, PAWN)));
        assert_eq!(board.bitboards[WHITE][PAWN], 0x80000000000);
        assert_eq!(board.bitboards[BLACK][PAWN], 0x0);

        let mut board = Board::from_fen("8/8/8/8/3p4/8/4P3/8 w - - 0 1").unwrap();
        board.make_move(Move::coordinate("e2e4"));
        board.make_move(Move::coordinate("d4e3"));
        assert_eq!(board.squares[E4], None);
        assert_eq!(board.squares[E3], Some((BLACK, PAWN)));
        assert_eq!(board.bitboards[BLACK][PAWN], 0x100000);
        assert_eq!(board.bitboards[WHITE][PAWN], 0x0);
    }

    #[test]
    fn legal_knight_moves() {
        let board = Board::from_fen("8/8/8/8/8/8/8/N7 w - - 0 1").unwrap();
        let moves = board.pseudo_legal_moves_square(Square(A1));
        assert!(moves.contains(&Move::new(A1, B3)));
        assert!(moves.contains(&Move::new(A1, C2)));
        assert_eq!(moves.len(), 2);

        let board = Board::default();
        let moves = board.pseudo_legal_moves_square(Square(B1));
        assert!(moves.contains(&Move::new(B1, A3)));
        assert!(moves.contains(&Move::new(B1, C3)));
        assert_eq!(moves.len(), 2);

        let board = Board::from_fen("8/8/2K1K3/1K3K2/3N4/1K3K2/2K1K3/8 w - - 0 1").unwrap();
        let moves = board.pseudo_legal_moves_square(Square(D4));
        assert!(moves.is_empty());
    }

    #[test]
    fn legal_king_moves() {
        let board = Board::from_fen("8/8/8/8/8/8/8/K7 w - - 0 1").unwrap();
        let moves = board.pseudo_legal_moves_square(Square(A1));
        assert!(moves.contains(&Move::new(A1, A2)));
        assert!(moves.contains(&Move::new(A1, B1)));
        assert!(moves.contains(&Move::new(A1, B2)));
        assert_eq!(moves.len(), 3);

        let board = Board::default();
        let moves = board.pseudo_legal_moves_square(Square(E1));
        assert!(moves.is_empty());

        let board = Board::from_fen("8/8/8/8/8/8/3PPP2/R3K2R w KQ - 0 1").unwrap();
        let moves = board.pseudo_legal_moves_square(Square(E1));
        assert!(moves.contains(&Move::new(E1, C1)));
        assert!(moves.contains(&Move::new(E1, G1)));
        assert_eq!(moves.len(), 4);

        let board = Board::from_fen("2r5/8/8/8/8/8/3PPP2/R3K2R w KQ - 0 1").unwrap();
        let moves = board.pseudo_legal_moves_square(Square(E1));
        assert!(moves.contains(&Move::new(E1, G1)));
        assert_eq!(moves.len(), 3);
    }

    #[test]
    fn legal_white_pawn_moves() {
        // Starting square
        let board = Board::default();
        let moves = board.pseudo_legal_moves_square(Square(E2));
        assert!(moves.contains(&Move::new(E2, E3)));
        assert!(moves.contains(&Move::new(E2, E4)));
        assert_eq!(moves.len(), 2);

        // Normal move
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/3P4/PPP1PPPP/RNBQKBNR b KQkq - 0 1").unwrap();
        let moves = board.pseudo_legal_moves_square(Square(D3));
        assert!(moves.contains(&Move::new(D3, D4)));
        assert_eq!(moves.len(), 1);

        // Attack
        let board = Board::from_fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2").unwrap();
        let moves = board.pseudo_legal_moves_square(Square(E4));
        assert!(moves.contains(&Move::new(E4, D5)));
        assert!(moves.contains(&Move::new(E4, E5)));
        assert_eq!(moves.len(), 2);

        // Can't attack forward
        let board = Board::from_fen("rnbqkbnr/ppp1pppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2").unwrap();
        let moves = board.pseudo_legal_moves_square(Square(E4));
        assert!(moves.is_empty());

        // Normal promotion
        let board = Board::from_fen("8/3P4/8/8/8/8/8/8 w - - 0 1").unwrap();
        let moves = board.pseudo_legal_moves_square(Square(D7));
        assert!(moves.contains(&Move::promotion(D7, D8, BISHOP)));
        assert!(moves.contains(&Move::promotion(D7, D8, KNIGHT)));
        assert!(moves.contains(&Move::promotion(D7, D8, QUEEN)));
        assert!(moves.contains(&Move::promotion(D7, D8, ROOK)));
        assert_eq!(moves.len(), 4);

        // Attacking promotion
        let board = Board::from_fen("4q3/3P4/8/8/8/8/8/8 w - - 0 1").unwrap();
        let moves = board.pseudo_legal_moves_square(Square(D7));
        assert!(moves.contains(&Move::promotion(D7, D8, BISHOP)));
        assert!(moves.contains(&Move::promotion(D7, D8, KNIGHT)));
        assert!(moves.contains(&Move::promotion(D7, D8, QUEEN)));
        assert!(moves.contains(&Move::promotion(D7, D8, ROOK)));
        assert!(moves.contains(&Move::promotion(D7, E8, BISHOP)));
        assert!(moves.contains(&Move::promotion(D7, E8, KNIGHT)));
        assert!(moves.contains(&Move::promotion(D7, E8, QUEEN)));
        assert!(moves.contains(&Move::promotion(D7, E8, ROOK)));
        assert_eq!(moves.len(), 8);

        // En passant
        let board = Board::from_fen("rnbqkb1r/ppp1pppp/5n2/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 3").unwrap();
        let moves = board.pseudo_legal_moves_square(Square(E5));
        assert!(moves.contains(&Move::new(E5, D6)));
        assert!(moves.contains(&Move::new(E5, E6)));
        assert!(moves.contains(&Move::new(E5, F6)));
        assert_eq!(moves.len(), 3);
    }

    #[test]
    fn legal_black_pawn_moves() {
        // Starting square
        let board = Board::default();
        let moves = board.pseudo_legal_moves_square(Square(E7));
        assert!(moves.contains(&Move::new(E7, E5)));
        assert!(moves.contains(&Move::new(E7, E6)));
        assert_eq!(moves.len(), 2);

        // Normal move
        let board = Board::from_fen("rnbqkbnr/pppp1ppp/4p3/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2").unwrap();
        let moves = board.pseudo_legal_moves_square(Square(E6));
        assert!(moves.contains(&Move::new(E6, E5)));
        assert_eq!(moves.len(), 1);

        // Attack
        let board = Board::from_fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2").unwrap();
        let moves = board.pseudo_legal_moves_square(Square(D5));
        assert!(moves.contains(&Move::new(D5, D4)));
        assert!(moves.contains(&Move::new(D5, E4)));
        assert_eq!(moves.len(), 2);

        // Normal promotion
        let board = Board::from_fen("8/8/8/8/8/8/3p4/8 w - - 0 1").unwrap();
        let moves = board.pseudo_legal_moves_square(Square(D2));
        assert!(moves.contains(&Move::promotion(D2, D1, BISHOP)));
        assert!(moves.contains(&Move::promotion(D2, D1, KNIGHT)));
        assert!(moves.contains(&Move::promotion(D2, D1, QUEEN)));
        assert!(moves.contains(&Move::promotion(D2, D1, ROOK)));
        assert_eq!(moves.len(), 4);

        // Attacking promotion
        let board = Board::from_fen("8/8/8/8/8/8/3p4/4Q3 w - - 0 1").unwrap();
        let moves = board.pseudo_legal_moves_square(Square(D2));
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
        let moves = board.pseudo_legal_moves_square(Square(A1));
        assert!(moves.is_empty());

        let board = Board::from_fen("8/3r4/8/8/3R3q/8/3B4/8 w - - 0 1").unwrap();
        let moves = board.pseudo_legal_moves_square(Square(D4));
        assert!(moves.contains(&Move::new(D4, A4)));
        assert!(moves.contains(&Move::new(D4, D3)));
        assert!(moves.contains(&Move::new(D4, D7)));
        assert!(moves.contains(&Move::new(D4, H4)));
        assert_eq!(moves.len(), 11);
    }

    #[test]
    fn legal_bishop_moves() {
        let board = Board::default();
        let moves = board.pseudo_legal_moves_square(Square(C1));
        assert!(moves.is_empty());

        let board = Board::from_fen("8/6n1/1Q6/8/3B4/8/1r6/8 w - - 0 1").unwrap();
        let moves = board.pseudo_legal_moves_square(Square(D4));
        assert!(moves.contains(&Move::new(D4, B2)));
        assert!(moves.contains(&Move::new(D4, C5)));
        assert!(moves.contains(&Move::new(D4, G1)));
        assert!(moves.contains(&Move::new(D4, G7)));
        assert_eq!(moves.len(), 9);
    }

    #[test]
    fn legal_queen_moves() {
        let board = Board::default();
        let moves = board.pseudo_legal_moves_square(Square(D1));
        assert!(moves.is_empty());

        let board = Board::from_fen("8/6n1/1R6/3K4/3Q2p1/8/1r6/8 w - - 0 1").unwrap();
        let moves = board.pseudo_legal_moves_square(Square(D4));
        assert!(moves.contains(&Move::new(D4, A4)));
        assert!(moves.contains(&Move::new(D4, B2)));
        assert!(moves.contains(&Move::new(D4, C5)));
        assert!(moves.contains(&Move::new(D4, D1)));
        assert!(moves.contains(&Move::new(D4, G1)));
        assert!(moves.contains(&Move::new(D4, G4)));
        assert!(moves.contains(&Move::new(D4, G7)));
        assert_eq!(moves.len(), 18);
    }

    #[test]
    fn unmake_move() {
        let mut board = Board::default();
        board.make_move(Move::coordinate("e2e4"));
        board.unmake_move();
        assert_eq!(board, Board::default());

        let mut board = Board::default();
        board.make_move(Move::coordinate("e2e4"));
        board.make_move(Move::coordinate("e7e4"));
        board.unmake_move();
        board.unmake_move();
        assert_eq!(board, Board::default());
    }

    #[test]
    fn perft() {
        let mut starting_position = Board::default();
        assert_eq!(starting_position.perft(1), 20);
        assert_eq!(starting_position.perft(2), 400);
        assert_eq!(starting_position.perft(3), 8_902);

        let mut opening = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
        assert_eq!(opening.perft(1), 48);
        assert_eq!(opening.perft(2), 2_039);

        let mut endgame = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
        assert_eq!(endgame.perft(1), 14);
        assert_eq!(endgame.perft(2), 191);
        assert_eq!(endgame.perft(3), 2_812);

        let mut middlegame = Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1").unwrap();
        assert_eq!(middlegame.perft(1), 6);
        assert_eq!(middlegame.perft(2), 264);
        assert_eq!(middlegame.perft(3), 9_467);

        let mut bug_finder = Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();
        assert_eq!(bug_finder.perft(1), 44);
        assert_eq!(bug_finder.perft(2), 1486);
    }
}
