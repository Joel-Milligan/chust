use std::fmt::Display;
use std::str::FromStr;

use bitvec::prelude::*;
use thiserror::Error;

use super::piece_move::{HistoryMove, Move};
use super::square::Square;
use crate::calculated::bishop::generate_bishop_moves;
use crate::repr::constants::*;
use crate::calculated::king::{KING_MOVES, generate_king_moves};
use crate::calculated::knight::generate_knight_moves;
use crate::calculated::pawn::{PAWN_ATTACKS, generate_pawn_moves};
use crate::calculated::rook::generate_rook_moves;
use crate::calculated::zobrist::*;

#[derive(Error, Debug)]
#[error("poorly formatted fen string")]
pub struct FenError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Board {
    pub pieces: [[u64; 6]; 2],
    pub squares: [Option<(u8, u8)>; 64],
    pub active_colour: u8,
    castling: u8,
    en_passant: Option<u8>,
    /// Number of half moves since last capture or pawn push, used for the fifty-move rule
    pub half_moves: u8,
    full_moves: u8,
    history: Vec<HistoryMove>,
    pub hash: u64,
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
        let mut squares = [None; 64];

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

            pieces[colour as usize][piece as usize] |= 1 << idx;
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
            squares,
            active_colour,
            castling,
            en_passant,
            half_moves,
            full_moves,
            history: Vec::new(),
            hash: instantiate_board_hash(active_colour, squares, castling, en_passant),
        })
    }

    pub fn moves(&mut self) -> Vec<Move> {
        let mut moves = vec![];
        for square in A1..=H8 {
            if let Some((colour, piece)) = self.squares[square as usize]
                && colour == self.active_colour
            {
                self.get_moves_for_square(square, colour, piece, &mut moves);
            }
        }
        moves
    }

    fn get_moves_for_square(&mut self, square: u8, colour: u8, piece: u8, moves: &mut Vec<Move>) {
        for candidate_move in self.generate_pseudo_moves(square, colour, piece) {
            self.make_move(&candidate_move);

            // Only add move if it doesn't result in our king being attacked
            if self.attacked(self.active_colour)
                & self.pieces[1 - self.active_colour as usize][KING as usize]
                == 0
            {
                moves.push(candidate_move);
            }

            self.unmake_move();
        }
    }

    fn generate_pseudo_moves(
        &mut self,
        square: u8,
        colour: u8,
        piece: u8,
    ) -> impl Iterator<Item = Move> + use<> {
        let friendly_pieces = self.pieces[colour as usize]
            .into_iter()
            .reduce(|acc, e| acc | e)
            .unwrap();

        let opponent_pieces = self.pieces[1 - colour as usize]
            .into_iter()
            .reduce(|acc, e| acc | e)
            .unwrap();

        let blockers = friendly_pieces | opponent_pieces;

        let pawn_attacks = PAWN_ATTACKS[colour as usize][square as usize]
            & (opponent_pieces | self.en_passant.map_or(0, |s| 1 << s));

        let pseudo_moves = match piece {
            BISHOP => generate_bishop_moves(square, blockers),
            KING => generate_king_moves(
                square,
                blockers,
                colour,
                self.attacked(1 - colour),
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
            _ => panic!("unknown piece"),
        } & !friendly_pieces;

        pseudo_moves
            .view_bits::<Lsb0>()
            .to_owned()
            .into_iter()
            .enumerate()
            .filter(|(_, m)| *m)
            .flat_map(move |(destination, _)| {
                if piece == PAWN
                    && ((colour == WHITE && (A7..H7).contains(&square))
                        || (colour == BLACK && (A2..H2).contains(&square)))
                {
                    vec![
                        Move::promotion(square, destination as u8, BISHOP),
                        Move::promotion(square, destination as u8, KNIGHT),
                        Move::promotion(square, destination as u8, QUEEN),
                        Move::promotion(square, destination as u8, ROOK),
                    ]
                } else {
                    vec![Move::new(square, destination as u8)]
                }
            })
    }

    pub fn make_move(&mut self, mv: &Move) {
        // Data needed for saving history move
        let prev_castling = self.castling;
        let prev_en_passant = self.en_passant;
        let mut captured_en_passant = false;

        // Pieces
        let (colour, piece) = self.squares[mv.source.0 as usize].unwrap();
        let captured_piece = self.squares[mv.destination.0 as usize];
        let mut en_passant = None;

        if piece == PAWN {
            // Remove opposing pawn via en passant
            if self.en_passant.is_some_and(|ep| mv.destination.0 == ep) {
                if colour == WHITE {
                    self.pieces[BLACK as usize][PAWN as usize] ^= 1 << (mv.destination.0 - 8);
                    self.squares[mv.destination.0 as usize - 8] = None;
                    self.hash ^= ZOBRIST_SQUARES[mv.destination.0 as usize - 8][BLACK as usize]
                        [PAWN as usize];
                } else {
                    self.pieces[WHITE as usize][PAWN as usize] ^= 1 << (mv.destination.0 + 8);
                    self.squares[mv.destination.0 as usize + 8] = None;
                    self.hash ^= ZOBRIST_SQUARES[mv.destination.0 as usize + 8][WHITE as usize]
                        [PAWN as usize];
                }
                captured_en_passant = true;
            }

            if mv.source.0.abs_diff(mv.destination.0) == 16 {
                if colour == WHITE {
                    en_passant = Some(mv.source.0 + 8);
                } else {
                    en_passant = Some(mv.source.0 - 8);
                }
            }
        } else if piece == ROOK {
            // Moving rook prevents castling
            if self.castling & WHITE_KING_SIDE != 0 && mv.source.0 == H1 {
                self.castling ^= WHITE_KING_SIDE;
                self.hash ^= *ZOBRIST_WHITE_KING_CASTLING;
            } else if self.castling & WHITE_QUEEN_SIDE != 0 && mv.source.0 == A1 {
                self.castling ^= WHITE_QUEEN_SIDE;
                self.hash ^= *ZOBRIST_WHITE_QUEEN_CASTLING;
            } else if self.castling & BLACK_KING_SIDE != 0 && mv.source.0 == H8 {
                self.castling ^= BLACK_KING_SIDE;
                self.hash ^= *ZOBRIST_BLACK_KING_CASTLING;
            } else if self.castling & BLACK_QUEEN_SIDE != 0 && mv.source.0 == A8 {
                self.castling ^= BLACK_QUEEN_SIDE;
                self.hash ^= *ZOBRIST_BLACK_QUEEN_CASTLING;
            }
        } else if piece == KING {
            // Castling
            if mv.source.0.abs_diff(mv.destination.0) == 2 {
                match mv.destination.0 {
                    G1 => {
                        self.pieces[colour as usize][ROOK as usize] ^= 1 << F1 | 1 << H1;
                        self.squares[H1 as usize] = None;
                        self.hash ^= ZOBRIST_SQUARES[H1 as usize][colour as usize][ROOK as usize];
                        self.squares[F1 as usize] = Some((colour, ROOK));
                        self.hash ^= ZOBRIST_SQUARES[F1 as usize][colour as usize][ROOK as usize];
                    }
                    C1 => {
                        self.pieces[colour as usize][ROOK as usize] ^= 1 << D1 | 1 << A1;
                        self.squares[A1 as usize] = None;
                        self.hash ^= ZOBRIST_SQUARES[A1 as usize][colour as usize][ROOK as usize];
                        self.squares[D1 as usize] = Some((colour, ROOK));
                        self.hash ^= ZOBRIST_SQUARES[D1 as usize][colour as usize][ROOK as usize];
                    }
                    G8 => {
                        self.pieces[colour as usize][ROOK as usize] ^= 1 << F8 | 1 << H8;
                        self.squares[H8 as usize] = None;
                        self.hash ^= ZOBRIST_SQUARES[H8 as usize][colour as usize][ROOK as usize];
                        self.squares[F8 as usize] = Some((colour, ROOK));
                        self.hash ^= ZOBRIST_SQUARES[F8 as usize][colour as usize][ROOK as usize];
                    }
                    C8 => {
                        self.pieces[colour as usize][ROOK as usize] ^= 1 << D8 | 1 << A8;
                        self.squares[A8 as usize] = None;
                        self.hash ^= ZOBRIST_SQUARES[A8 as usize][colour as usize][ROOK as usize];
                        self.squares[D8 as usize] = Some((colour, ROOK));
                        self.hash ^= ZOBRIST_SQUARES[D8 as usize][colour as usize][ROOK as usize];
                    }
                    s => panic!("can't castle to square: {}", Square(s)),
                }
            }

            // Moving king prevents castling
            if colour == WHITE {
                if self.castling & WHITE_KING_SIDE != 0 {
                    self.hash ^= *ZOBRIST_WHITE_KING_CASTLING;
                }
                if self.castling & WHITE_QUEEN_SIDE != 0 {
                    self.hash ^= *ZOBRIST_WHITE_QUEEN_CASTLING;
                }
                self.castling &= !(WHITE_KING_SIDE | WHITE_QUEEN_SIDE);
            } else {
                if self.castling & BLACK_KING_SIDE != 0 {
                    self.hash ^= *ZOBRIST_BLACK_KING_CASTLING;
                }
                if self.castling & BLACK_QUEEN_SIDE != 0 {
                    self.hash ^= *ZOBRIST_BLACK_QUEEN_CASTLING;
                }
                self.castling &= !(BLACK_KING_SIDE | BLACK_QUEEN_SIDE);
            }
        }

        // Captured rook prevents castling
        if self.castling & WHITE_KING_SIDE != 0 && mv.destination.0 == H1 {
            self.castling ^= WHITE_KING_SIDE;
            self.hash ^= *ZOBRIST_WHITE_KING_CASTLING;
        } else if self.castling & WHITE_QUEEN_SIDE != 0 && mv.destination.0 == A1 {
            self.castling ^= WHITE_QUEEN_SIDE;
            self.hash ^= *ZOBRIST_WHITE_QUEEN_CASTLING;
        } else if self.castling & BLACK_KING_SIDE != 0 && mv.destination.0 == H8 {
            self.castling ^= BLACK_KING_SIDE;
            self.hash ^= *ZOBRIST_BLACK_KING_CASTLING;
        } else if self.castling & BLACK_QUEEN_SIDE != 0 && mv.destination.0 == A8 {
            self.castling ^= BLACK_QUEEN_SIDE;
            self.hash ^= *ZOBRIST_BLACK_QUEEN_CASTLING;
        }

        if let Some(existing_en_passant) = self.en_passant {
            self.hash ^= ZOBRIST_EN_PASSANT[existing_en_passant as usize % 8];
        }
        self.en_passant = en_passant;
        if let Some(new_en_passant) = self.en_passant {
            self.hash ^= ZOBRIST_EN_PASSANT[new_en_passant as usize % 8];
        }

        if let Some(promotion) = mv.promotion {
            // Promote
            self.pieces[colour as usize][piece as usize] ^= 1 << mv.source.0;
            self.pieces[colour as usize][promotion as usize] ^= 1 << mv.destination.0;
            self.squares[mv.source.0 as usize] = None;
            self.hash ^= ZOBRIST_SQUARES[mv.source.0 as usize][colour as usize][piece as usize];
            self.squares[mv.destination.0 as usize] = Some((colour, promotion));
            self.hash ^=
                ZOBRIST_SQUARES[mv.destination.0 as usize][colour as usize][promotion as usize];
        } else {
            // Normal move
            self.pieces[colour as usize][piece as usize] ^=
                1 << mv.destination.0 | 1 << mv.source.0;
            self.squares[mv.source.0 as usize] = None;
            self.hash ^= ZOBRIST_SQUARES[mv.source.0 as usize][colour as usize][piece as usize];
            self.squares[mv.destination.0 as usize] = Some((colour, piece));
            self.hash ^=
                ZOBRIST_SQUARES[mv.destination.0 as usize][colour as usize][piece as usize];
        }

        // Capture
        if let Some(captured) = captured_piece {
            self.pieces[captured.0 as usize][captured.1 as usize] ^= 1 << mv.destination.0;
            self.hash ^= ZOBRIST_SQUARES[mv.destination.0 as usize][captured.0 as usize]
                [captured.1 as usize];
        };

        // Switch colour
        if self.active_colour == BLACK {
            self.full_moves += 1;
        }
        self.active_colour = 1 - colour;
        self.hash ^= *ZOBRIST_BLACK;

        // Increment counter for 50-move rule
        if captured_piece.is_some() || piece == PAWN {
            self.half_moves = 0;
        } else {
            self.half_moves += 1;
        }

        self.history.push(HistoryMove {
            moved: (colour, piece),
            source: mv.source,
            destination: mv.destination,
            captured: captured_piece,
            previous_en_passant_square: prev_en_passant,
            promotion: mv.promotion,
            en_passant_capture: captured_en_passant,
            removed_castling_rights: self.castling ^ prev_castling,
            previous_full_moves: self.full_moves,
            previous_half_moves: self.half_moves,
        });
    }

    pub fn unmake_move(&mut self) {
        let mv = self
            .history
            .pop()
            .expect("should never reverse a move when no moves in history");

        // Pieces
        let (colour, piece) = mv.moved;
        let captured_piece = mv.captured;
        let en_passant = mv.en_passant_capture;

        if piece == PAWN {
            // Add back in opposing pawn due to en passant
            if en_passant {
                if colour == WHITE {
                    self.pieces[BLACK as usize][PAWN as usize] ^= 1 << (mv.destination.0 - 8);
                    self.squares[mv.destination.0 as usize - 8] = Some((BLACK, PAWN));
                    self.hash ^= ZOBRIST_SQUARES[mv.destination.0 as usize - 8][BLACK as usize]
                        [PAWN as usize];
                } else {
                    self.pieces[WHITE as usize][PAWN as usize] ^= 1 << (mv.destination.0 + 8);
                    self.squares[mv.destination.0 as usize + 8] = Some((WHITE, PAWN));
                    self.hash ^= ZOBRIST_SQUARES[mv.destination.0 as usize + 8][WHITE as usize]
                        [PAWN as usize];
                }
            }
        } else if piece == KING {
            // Reverse castling for rook
            if mv.source.0.abs_diff(mv.destination.0) == 2 {
                match mv.destination.0 {
                    G1 => {
                        self.pieces[colour as usize][ROOK as usize] ^= 1 << F1 | 1 << H1;
                        self.squares[F1 as usize] = None;
                        self.hash ^= ZOBRIST_SQUARES[H1 as usize][colour as usize][ROOK as usize];
                        self.squares[H1 as usize] = Some((colour, ROOK));
                        self.hash ^= ZOBRIST_SQUARES[F1 as usize][colour as usize][ROOK as usize];
                    }
                    C1 => {
                        self.pieces[colour as usize][ROOK as usize] ^= 1 << D1 | 1 << A1;
                        self.squares[D1 as usize] = None;
                        self.hash ^= ZOBRIST_SQUARES[A1 as usize][colour as usize][ROOK as usize];
                        self.squares[A1 as usize] = Some((colour, ROOK));
                        self.hash ^= ZOBRIST_SQUARES[D1 as usize][colour as usize][ROOK as usize];
                    }
                    G8 => {
                        self.pieces[colour as usize][ROOK as usize] ^= 1 << F8 | 1 << H8;
                        self.squares[F8 as usize] = None;
                        self.hash ^= ZOBRIST_SQUARES[H8 as usize][colour as usize][ROOK as usize];
                        self.squares[H8 as usize] = Some((colour, ROOK));
                        self.hash ^= ZOBRIST_SQUARES[F8 as usize][colour as usize][ROOK as usize];
                    }
                    C8 => {
                        self.pieces[colour as usize][ROOK as usize] ^= 1 << D8 | 1 << A8;
                        self.squares[D8 as usize] = None;
                        self.hash ^= ZOBRIST_SQUARES[A8 as usize][colour as usize][ROOK as usize];
                        self.squares[A8 as usize] = Some((colour, ROOK));
                        self.hash ^= ZOBRIST_SQUARES[D8 as usize][colour as usize][ROOK as usize];
                    }
                    s => panic!("can't castle to square: {}", Square(s)),
                }
            }
        }

        // Re-institute any lost castling rights
        self.castling |= mv.removed_castling_rights;
        if mv.removed_castling_rights & WHITE_KING_SIDE != 0 {
            self.hash ^= *ZOBRIST_WHITE_KING_CASTLING;
        }
        if mv.removed_castling_rights & WHITE_QUEEN_SIDE != 0 {
            self.hash ^= *ZOBRIST_WHITE_QUEEN_CASTLING;
        }
        if mv.removed_castling_rights & BLACK_KING_SIDE != 0 {
            self.hash ^= *ZOBRIST_BLACK_KING_CASTLING;
        }
        if mv.removed_castling_rights & BLACK_QUEEN_SIDE != 0 {
            self.hash ^= *ZOBRIST_BLACK_QUEEN_CASTLING;
        }

        if let Some(promotion) = mv.promotion {
            // Promote
            self.pieces[colour as usize][piece as usize] ^= 1 << mv.source.0;
            self.pieces[colour as usize][promotion as usize] ^= 1 << mv.destination.0;
            self.squares[mv.destination.0 as usize] = None;
            self.hash ^= ZOBRIST_SQUARES[mv.source.0 as usize][colour as usize][piece as usize];
            self.squares[mv.source.0 as usize] = Some((colour, PAWN));
            self.hash ^=
                ZOBRIST_SQUARES[mv.destination.0 as usize][colour as usize][promotion as usize];
        } else {
            // Normal move
            self.pieces[colour as usize][piece as usize] ^=
                1 << mv.destination.0 | 1 << mv.source.0;
            self.squares[mv.destination.0 as usize] = None;
            self.hash ^=
                ZOBRIST_SQUARES[mv.destination.0 as usize][colour as usize][piece as usize];
            self.squares[mv.source.0 as usize] = Some((colour, piece));
            self.hash ^= ZOBRIST_SQUARES[mv.source.0 as usize][colour as usize][piece as usize];
        }

        // Capture
        if let Some(captured) = captured_piece {
            self.pieces[captured.0 as usize][captured.1 as usize] ^= 1 << mv.destination.0;
            self.squares[mv.destination.0 as usize] = Some((captured.0, captured.1));
            self.hash ^= ZOBRIST_SQUARES[mv.destination.0 as usize][captured.0 as usize]
                [captured.1 as usize];
        } else {
            self.squares[mv.destination.0 as usize] = None;
        }

        // Reset en passant square
        if let Some(existing_en_passant) = self.en_passant {
            self.hash ^= ZOBRIST_EN_PASSANT[existing_en_passant as usize % 8];
        }
        self.en_passant = mv.previous_en_passant_square;
        if let Some(new_en_passant) = self.en_passant {
            self.hash ^= ZOBRIST_EN_PASSANT[new_en_passant as usize % 8];
        }

        // Switch colour
        if self.active_colour == WHITE {
            self.full_moves -= 1;
        }
        self.hash ^= *ZOBRIST_BLACK;
        self.active_colour = colour;

        // Reset counter for 50-move rule
        self.half_moves = mv.previous_half_moves;
    }

    pub fn attacked(&self, attacking_colour: u8) -> u64 {
        (A1..=H8)
            .map(|square| (square, self.squares[square as usize]))
            .filter(|(_, piece)| piece.is_some_and(|(colour, _)| colour == attacking_colour))
            .map(|(square, piece)| match piece.unwrap().1 {
                PAWN => PAWN_ATTACKS[attacking_colour as usize][square as usize],
                KNIGHT => generate_knight_moves(square),
                BISHOP => generate_bishop_moves(square, self.blockers()),
                ROOK => generate_rook_moves(square, self.blockers()),
                QUEEN => {
                    generate_bishop_moves(square, self.blockers())
                        | generate_rook_moves(square, self.blockers())
                }
                KING => KING_MOVES[square as usize],
                _ => panic!("unknown piece"),
            })
            .reduce(|acc, e| acc | e)
            .unwrap_or(0)
    }

    pub fn in_check(&self) -> bool {
        let attacked = self.attacked(1 - self.active_colour);
        self.pieces[self.active_colour as usize][KING as usize] & attacked != 0
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

        if depth == 1 {
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
}

fn instantiate_board_hash(
    active_colour: u8,
    squares: [Option<(u8, u8)>; 64],
    castling: u8,
    en_passant: Option<u8>,
) -> u64 {
    let mut hash = 0u64;

    if active_colour == BLACK {
        hash ^= *ZOBRIST_BLACK;
    }

    for square in A1..=H8 {
        if let Some((colour, piece)) = squares[square as usize] {
            hash ^= ZOBRIST_SQUARES[square as usize][colour as usize][piece as usize];
        }
    }

    if castling & WHITE_KING_SIDE != 0 {
        hash ^= *ZOBRIST_WHITE_KING_CASTLING;
    }
    if castling & WHITE_QUEEN_SIDE != 0 {
        hash ^= *ZOBRIST_WHITE_QUEEN_CASTLING;
    }
    if castling & BLACK_KING_SIDE != 0 {
        hash ^= *ZOBRIST_BLACK_KING_CASTLING;
    }
    if castling & BLACK_QUEEN_SIDE != 0 {
        hash ^= *ZOBRIST_BLACK_QUEEN_CASTLING;
    }

    if let Some(en_passant) = en_passant {
        hash ^= ZOBRIST_EN_PASSANT[(en_passant % 8) as usize]
    }

    hash
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
