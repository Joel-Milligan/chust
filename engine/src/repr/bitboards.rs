//! Utility functions for working with bitboards

// Files
pub const A_FILE: u64 = 0x0101010101010101;
pub const B_FILE: u64 = 0x0202020202020202;
pub const G_FILE: u64 = 0x4040404040404040;
pub const H_FILE: u64 = 0x8080808080808080;

// Ranks
pub const FIRST_RANK: u64 = 0xff;
pub const SECOND_RANK: u64 = 0xff00;
pub const SEVENTH_RANK: u64 = 0xff000000000000;
pub const EIGHTH_RANK: u64 = 0xff00000000000000;

/// Filter out certain squares from a bitboard
pub fn filter(bitboard: u64, filters: Vec<u64>) -> u64 {
    bitboard & !(filters.into_iter().reduce(|acc, e| acc | e).unwrap())
}

/// Nicely prints out a given 64 bit number as a chess bitboard
#[allow(dead_code)]
pub fn print_bitboard(bitboard: u64) {
    for rank in (0..8).rev() {
        print!("{} ", rank + 1);
        for file in 0..8 {
            let square = rank * 8 + file;
            let occupied = bitboard & (1u64 << square) != 0;
            print!("{} ", if occupied { "o" } else { "." })
        }
        println!();
    }
    println!("  a b c d e f g h");
}
