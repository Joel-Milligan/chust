#![feature(lazy_cell)]

mod bitboards;
mod board;
mod calculated;
mod constants;
mod piece_move;
mod square;

use crate::bitboards::print_bitboard;
use crate::board::Board;
use crate::calculated::*;
use crate::constants::*;

fn main() {
    let board = Board::from_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2");
    println!("{board}");

    print_bitboard(PAWN_ATTACKS[WHITE][A2]);
    print_bitboard(PAWN_ATTACKS[WHITE][H2]);
    print_bitboard(PAWN_ATTACKS[BLACK][A7]);
    print_bitboard(PAWN_ATTACKS[BLACK][H7]);
}
