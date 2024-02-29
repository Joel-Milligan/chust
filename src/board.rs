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
    pieces: [[u64; 6]; 2],
    pub active_colour: usize,
    castling: u8,
    en_passant: Option<usize>,
    /// Number of half moves since last capture or pawn push, used for the fifty-move rule
    half_moves: usize,
    full_moves: usize,
    previous_position: Option<Box<Board>>,
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
                if let Some(piece) = self.get_piece_at_square(rank * 8 + file) {
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
                        write!(f, "{kind} ")?;
                    }
                } else {
                    write!(f, ". ")?;
                }
            }
            writeln!(f)?;
        }
        write!(f, "  a b c d e f g h")
    }
}

impl Board {
    pub fn from_fen(fen: &str) -> Result<Self, FenError> {
        let mut pieces = [[0; 6]; 2];

        let fields: Vec<&str> = fen.split_whitespace().collect();

        if fields.len() != 6 {
            return Err(FenError);
        }

        let mut idx = 0;
        for ch in fields[0].split('/').rev().flat_map(str::chars) {
            if ch.is_numeric() {
                let empty_squares = ch.to_digit(10).ok_or(FenError)?;
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

            pieces[colour][piece] |= 1 << idx;

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
                c => panic!("unknown castling character: {c}"),
            });

        let en_passant = match fields[3] {
            "-" => None,
            square => Some(Square::from_str(square).map_err(|_| FenError)?.0),
        };

        let half_moves = fields[4].parse().map_err(|_| FenError)?;
        let full_moves = fields[5].parse().map_err(|_| FenError)?;

        Ok(Board {
            pieces,
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
                board.make_move(candidate);
                !board.pseudo_legal_moves().into_iter().any(|mv| {
                    board.get_piece_at_square(mv.destination.0) == Some((self.active_colour, KING))
                })
            })
            .collect()
    }

    fn pseudo_legal_moves(&self) -> Vec<Move> {
        (A1..=H8)
            .into_iter()
            .map(|square| (square, self.get_piece_at_square(square)))
            .filter(|(_, piece)| piece.is_some_and(|(colour, _)| colour == self.active_colour))
            .flat_map(|(square, _)| self.pseudo_legal_moves_square(square))
            .collect()
    }

    fn pseudo_legal_moves_square(&self, square: usize) -> Vec<Move> {
        let (colour, piece) = match self.get_piece_at_square(square) {
            Some(p) => p,
            None => return vec![],
        };
        let opponent_colour = if colour == WHITE { BLACK } else { WHITE };

        let friendly_pieces = self.pieces[colour]
            .into_iter()
            .reduce(|acc, e| acc | e)
            .unwrap();

        let opponent_pieces = self.pieces[opponent_colour]
            .into_iter()
            .reduce(|acc, e| acc | e)
            .unwrap();

        let blockers = friendly_pieces | opponent_pieces;

        let pawn_attacks = PAWN_ATTACKS[colour][square]
            & (opponent_pieces | self.en_passant.map_or(0, |s| 1 << s));

        let moves = match piece {
            BISHOP => generate_bishop_moves(square, blockers),
            KING => generate_king_moves(
                square,
                blockers,
                colour,
                self.attacked_squares(opponent_colour),
                self.castling,
            ),
            KNIGHT => generate_knight_moves(square),
            PAWN => {
                pawn_attacks | (generate_pawn_moves(square, blockers, colour) & !opponent_pieces)
            }
            QUEEN => {
                generate_bishop_moves(square, blockers) | generate_rook_moves(square, blockers)
            }
            ROOK => generate_rook_moves(square, blockers),
            _ => panic!("Unknown piece"),
        };

        let moves = moves & !friendly_pieces;
        let moves = moves.view_bits::<Lsb0>();

        // Generate promotions
        if piece == PAWN
            && ((colour == WHITE && (A7..H7).contains(&square))
                || (colour == BLACK && (A2..H2).contains(&square)))
        {
            return moves
                .into_iter()
                .enumerate()
                .filter(|(_, m)| **m)
                .flat_map(|destination| {
                    vec![
                        Move::promotion(square, destination.0, BISHOP),
                        Move::promotion(square, destination.0, KNIGHT),
                        Move::promotion(square, destination.0, QUEEN),
                        Move::promotion(square, destination.0, ROOK),
                    ]
                })
                .collect();
        }

        moves
            .into_iter()
            .enumerate()
            .filter(|(_, m)| **m)
            .map(|destination| Move::new(square, destination.0))
            .collect()
    }

    pub fn make_move(&mut self, mv: &Move) {
        // Save current position for unmaking moves
        self.previous_position = Some(Box::new(self.clone()));

        // Pieces
        let (colour, piece) = self.get_piece_at_square(mv.source.0).unwrap();
        let captured_piece = self.get_piece_at_square(mv.destination.0);

        let opponent_colour = if colour == WHITE { BLACK } else { WHITE };

        // Castling
        if piece == KING && mv.source.0.abs_diff(mv.destination.0) == 2 {
            match mv.destination.0 {
                G1 => self.pieces[colour][ROOK] ^= 1 << F1 | 1 << H1,
                C1 => self.pieces[colour][ROOK] ^= 1 << D1 | 1 << A1,
                G8 => self.pieces[colour][ROOK] ^= 1 << F8 | 1 << H8,
                C8 => self.pieces[colour][ROOK] ^= 1 << D8 | 1 << A8,
                s => panic!("can't castle to square: {}", Square(s)),
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
                self.pieces[BLACK][PAWN] ^= 1 << (mv.destination.0 - 8);
            } else {
                self.pieces[WHITE][PAWN] ^= 1 << (mv.destination.0 + 8);
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
            self.pieces[colour][piece] ^= 1 << mv.source.0;
            self.pieces[colour][promotion] ^= 1 << mv.destination.0;
        } else {
            self.pieces[colour][piece] ^= 1 << mv.destination.0 | 1 << mv.source.0;
        }

        if let Some(piece) = captured_piece {
            self.pieces[piece.0][piece.1] ^= 1 << mv.destination.0;
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

    pub fn attacked_squares(&self, attacking_colour: usize) -> u64 {
        (A1..=H8)
            .into_iter()
            .map(|square| (square, self.get_piece_at_square(square)))
            .filter(|(_, piece)| piece.is_some_and(|(colour, _)| colour == attacking_colour))
            .map(|(square, piece)| match piece.unwrap().1 {
                BISHOP => generate_bishop_moves(square, self.blockers()),
                KING => KING_MOVES[square],
                KNIGHT => generate_knight_moves(square),
                PAWN => PAWN_ATTACKS[attacking_colour][square],
                QUEEN => {
                    generate_bishop_moves(square, self.blockers())
                        | generate_rook_moves(square, self.blockers())
                }
                ROOK => generate_rook_moves(square, self.blockers()),
                _ => panic!("unknown piece"),
            })
            .reduce(|acc, e| acc | e)
            .unwrap_or(0)
    }

    fn blockers(&self) -> u64 {
        self.pieces
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
                self.make_move(&mv);
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
            self.make_move(mv);

            let perft = self.perft(depth - 1);
            nodes += perft;
            println!("{mv} {perft}");

            self.unmake_move();
        }

        println!("\n{nodes}");
    }

    pub fn get_piece_at_square(&self, square: usize) -> Option<(usize, usize)> {
        for colour in WHITE..=BLACK {
            for piece in KING..=PAWN {
                if self.pieces[colour][piece] & 1 << square != 0 {
                    return Some((colour, piece));
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unmake_move() {
        let mut board = Board::default();
        board.make_move(&Move::coordinate("e2e4"));
        board.unmake_move();
        assert_eq!(board, Board::default());

        let mut board = Board::default();
        board.make_move(&Move::coordinate("e2e4"));
        board.make_move(&Move::coordinate("e7e4"));
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

        let mut opening =
            Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
                .unwrap();
        assert_eq!(opening.perft(1), 48);
        assert_eq!(opening.perft(2), 2_039);
        assert_eq!(opening.perft(3), 97_862);

        let mut endgame = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
        assert_eq!(endgame.perft(1), 14);
        assert_eq!(endgame.perft(2), 191);
        assert_eq!(endgame.perft(3), 2_812);
        assert_eq!(endgame.perft(4), 43_238);

        let mut middlegame =
            Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1")
                .unwrap();
        assert_eq!(middlegame.perft(1), 6);
        assert_eq!(middlegame.perft(2), 264);
        assert_eq!(middlegame.perft(3), 9_467);

        let mut bug_finder =
            Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();
        assert_eq!(bug_finder.perft(1), 44);
        assert_eq!(bug_finder.perft(2), 1486);
        assert_eq!(bug_finder.perft(3), 62_379);
    }
}
