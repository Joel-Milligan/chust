use std::sync::LazyLock;

use crate::bitboards::filter;
use crate::constants::*;

pub static KNIGHT_MOVES: LazyLock<[u64; 64]> = LazyLock::new(|| {
    // TODO: Filter out moves onto friendly pieces
    let mut knight_moves = [0; 64];

    for square in A1..=H8 {
        let bitboard = 1 << square;
        let mut attacks = 0;

        // All knight moves starting from NNW going clockwise
        attacks |= filter(bitboard, vec![A_FILE, SEVENTH_RANK, EIGHTH_RANK]) << 15;
        attacks |= filter(bitboard, vec![H_FILE, SEVENTH_RANK, EIGHTH_RANK]) << 17;
        attacks |= filter(bitboard, vec![G_FILE, H_FILE, EIGHTH_RANK]) << 10;
        attacks |= filter(bitboard, vec![G_FILE, H_FILE, FIRST_RANK]) >> 6;
        attacks |= filter(bitboard, vec![H_FILE, FIRST_RANK, SECOND_RANK]) >> 15;
        attacks |= filter(bitboard, vec![A_FILE, FIRST_RANK, SECOND_RANK]) >> 17;
        attacks |= filter(bitboard, vec![A_FILE, B_FILE, FIRST_RANK]) >> 10;
        attacks |= filter(bitboard, vec![A_FILE, B_FILE, EIGHTH_RANK]) << 6;

        knight_moves[square] = attacks;
    }

    knight_moves
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_move_set() {
        assert_eq!(KNIGHT_MOVES[D3], 0x1422002214);
        assert_eq!(KNIGHT_MOVES[E4], 0x284400442800);
        assert_eq!(KNIGHT_MOVES[F5], 0x50880088500000);
        assert_eq!(KNIGHT_MOVES[C6], 0xa1100110a000000);
    }

    #[test]
    fn vertical_edges() {
        assert_eq!(KNIGHT_MOVES[A4], 0x20400040200);
        assert_eq!(KNIGHT_MOVES[B4], 0x50800080500);
        assert_eq!(KNIGHT_MOVES[G4], 0xa0100010a000);
        assert_eq!(KNIGHT_MOVES[H4], 0x402000204000);
    }

    #[test]
    fn horizontal_edges() {
        assert_eq!(KNIGHT_MOVES[E1], 0x284400);
        assert_eq!(KNIGHT_MOVES[E2], 0x28440044);
        assert_eq!(KNIGHT_MOVES[E7], 0x4400442800000000);
        assert_eq!(KNIGHT_MOVES[E8], 0x44280000000000);
    }

    #[test]
    fn corners() {
        assert_eq!(KNIGHT_MOVES[A1], 0x20400);
        assert_eq!(KNIGHT_MOVES[H1], 0x402000);
        assert_eq!(KNIGHT_MOVES[A8], 0x4020000000000);
        assert_eq!(KNIGHT_MOVES[H8], 0x20400000000000);
    }
}
