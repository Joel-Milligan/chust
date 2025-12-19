#![allow(dead_code)]
use std::sync::LazyLock;
// Sides
pub const WHITE: u8 = 0;
pub const BLACK: u8 = 1;

// Piece Types
pub const KING: u8 = 0;
pub const QUEEN: u8 = 1;
pub const ROOK: u8 = 2;
pub const BISHOP: u8 = 3;
pub const KNIGHT: u8 = 4;
pub const PAWN: u8 = 5;

// Castling
pub const WHITE_KING_SIDE: u8 = 1;
pub const WHITE_QUEEN_SIDE: u8 = 2;
pub const BLACK_KING_SIDE: u8 = 4;
pub const BLACK_QUEEN_SIDE: u8 = 8;

// Engine Values
pub const QUEEN_VALUE: i64 = 900;
pub const ROOK_VALUE: i64 = 500;
pub const BISHOP_VALUE: i64 = 300;
pub const KNIGHT_VALUE: i64 = 300;
pub const PAWN_VALUE: i64 = 100;
pub const MATED_VALUE: i64 = i64::MIN / 2;

// Files
pub const A_FILE: u64 = 0x0101010101010101;
pub const B_FILE: u64 = 0x0202020202020202;
pub const C_FILE: u64 = 0x0404040404040404;
pub const F_FILE: u64 = 0x2020202020202020;
pub const G_FILE: u64 = 0x4040404040404040;
pub const H_FILE: u64 = 0x8080808080808080;

// Ranks
pub const FIRST_RANK: u64 = 0xff;
pub const SECOND_RANK: u64 = 0xff00;
pub const SEVENTH_RANK: u64 = 0xff000000000000;
pub const EIGHTH_RANK: u64 = 0xff00000000000000;

// Square Indexes
pub const A1: u8 = 0;
pub const B1: u8 = 1;
pub const C1: u8 = 2;
pub const D1: u8 = 3;
pub const E1: u8 = 4;
pub const F1: u8 = 5;
pub const G1: u8 = 6;
pub const H1: u8 = 7;
pub const A2: u8 = 8;
pub const B2: u8 = 9;
pub const C2: u8 = 10;
pub const D2: u8 = 11;
pub const E2: u8 = 12;
pub const F2: u8 = 13;
pub const G2: u8 = 14;
pub const H2: u8 = 15;
pub const A3: u8 = 16;
pub const B3: u8 = 17;
pub const C3: u8 = 18;
pub const D3: u8 = 19;
pub const E3: u8 = 20;
pub const F3: u8 = 21;
pub const G3: u8 = 22;
pub const H3: u8 = 23;
pub const A4: u8 = 24;
pub const B4: u8 = 25;
pub const C4: u8 = 26;
pub const D4: u8 = 27;
pub const E4: u8 = 28;
pub const F4: u8 = 29;
pub const G4: u8 = 30;
pub const H4: u8 = 31;
pub const A5: u8 = 32;
pub const B5: u8 = 33;
pub const C5: u8 = 34;
pub const D5: u8 = 35;
pub const E5: u8 = 36;
pub const F5: u8 = 37;
pub const G5: u8 = 38;
pub const H5: u8 = 39;
pub const A6: u8 = 40;
pub const B6: u8 = 41;
pub const C6: u8 = 42;
pub const D6: u8 = 43;
pub const E6: u8 = 44;
pub const F6: u8 = 45;
pub const G6: u8 = 46;
pub const H6: u8 = 47;
pub const A7: u8 = 48;
pub const B7: u8 = 49;
pub const C7: u8 = 50;
pub const D7: u8 = 51;
pub const E7: u8 = 52;
pub const F7: u8 = 53;
pub const G7: u8 = 54;
pub const H7: u8 = 55;
pub const A8: u8 = 56;
pub const B8: u8 = 57;
pub const C8: u8 = 58;
pub const D8: u8 = 59;
pub const E8: u8 = 60;
pub const F8: u8 = 61;
pub const G8: u8 = 62;
pub const H8: u8 = 63;

pub static ZOBRIST_BLACK: LazyLock<u64> = LazyLock::new(|| rand::random());

pub static ZOBRIST: LazyLock<[[[u64; 6]; 2]; 64]> = LazyLock::new(|| {
    let mut table = [[[0; 6]; 2]; 64];

    for square in A1..=H8 {
        for colour in WHITE..=BLACK {
            for piece in KING..=PAWN {
                table[square as usize][colour as usize][piece as usize] = rand::random();
            }
        }
    }

    table
});
