use std::sync::LazyLock;

use crate::repr::bitboards::*;
use crate::repr::constants::*;

pub fn generate_knight_moves(square: u8) -> u64 {
    KNIGHT_MOVES[square as usize]
}

static KNIGHT_MOVES: LazyLock<[u64; 64]> = LazyLock::new(|| {
    let mut knight_moves = [0; 64];

    for (square, mv) in knight_moves.iter_mut().enumerate().take(H8 as usize + 1) {
        let bitboard = 1 << square;
        let mut attacks = 0;

        attacks |= filter(bitboard, vec![A_FILE, SEVENTH_RANK, EIGHTH_RANK]) << 15;
        attacks |= filter(bitboard, vec![H_FILE, SEVENTH_RANK, EIGHTH_RANK]) << 17;
        attacks |= filter(bitboard, vec![G_FILE, H_FILE, EIGHTH_RANK]) << 10;
        attacks |= filter(bitboard, vec![G_FILE, H_FILE, FIRST_RANK]) >> 6;
        attacks |= filter(bitboard, vec![H_FILE, FIRST_RANK, SECOND_RANK]) >> 15;
        attacks |= filter(bitboard, vec![A_FILE, FIRST_RANK, SECOND_RANK]) >> 17;
        attacks |= filter(bitboard, vec![A_FILE, B_FILE, FIRST_RANK]) >> 10;
        attacks |= filter(bitboard, vec![A_FILE, B_FILE, EIGHTH_RANK]) << 6;

        *mv = attacks;
    }

    knight_moves
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_move_set() {
        assert_eq!(KNIGHT_MOVES[D3 as usize], 0x0014_2200_2214);
        assert_eq!(KNIGHT_MOVES[E4 as usize], 0x2844_0044_2800);
        assert_eq!(KNIGHT_MOVES[F5 as usize], 0x0050_8800_8850_0000);
        assert_eq!(KNIGHT_MOVES[C6 as usize], 0x0a11_0011_0a00_0000);
    }

    #[test]
    fn vertical_edges() {
        assert_eq!(KNIGHT_MOVES[A4 as usize], 0x0204_0004_0200);
        assert_eq!(KNIGHT_MOVES[B4 as usize], 0x0508_0008_0500);
        assert_eq!(KNIGHT_MOVES[G4 as usize], 0xa010_0010_a000);
        assert_eq!(KNIGHT_MOVES[H4 as usize], 0x4020_0020_4000);
    }

    #[test]
    fn horizontal_edges() {
        assert_eq!(KNIGHT_MOVES[E1 as usize], 0x0028_4400);
        assert_eq!(KNIGHT_MOVES[E2 as usize], 0x2844_0044);
        assert_eq!(KNIGHT_MOVES[E7 as usize], 0x4400_4428_0000_0000);
        assert_eq!(KNIGHT_MOVES[E8 as usize], 0x0044_2800_0000_0000);
    }

    #[test]
    fn corners() {
        assert_eq!(KNIGHT_MOVES[A1 as usize], 0x20400);
        assert_eq!(KNIGHT_MOVES[H1 as usize], 0x0040_2000);
        assert_eq!(KNIGHT_MOVES[A8 as usize], 0x0004_0200_0000_0000);
        assert_eq!(KNIGHT_MOVES[H8 as usize], 0x0020_4000_0000_0000);
    }
}
