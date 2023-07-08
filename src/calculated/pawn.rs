use std::sync::LazyLock;

use crate::bitboards::filter;
use crate::constants::*;

pub static PAWN_ATTACKS: LazyLock<[[u64; 64]; 2]> = LazyLock::new(|| {
    let mut pawn_attacks = [[0; 64]; 2];

    for side in WHITE..=BLACK {
        for square in A2..=H7 {
            let bitboard = 1 << square;
            let mut attacks = 0;

            if side == WHITE {
                attacks |= filter(&bitboard, vec![H_FILE]) << 9;
                attacks |= filter(&bitboard, vec![A_FILE]) << 7;
            } else {
                attacks |= filter(&bitboard, vec![H_FILE]) >> 7;
                attacks |= filter(&bitboard, vec![A_FILE]) >> 9;
            }

            pawn_attacks[side][square] = attacks;
        }
    }

    pawn_attacks
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_move_set() {
        assert_eq!(PAWN_ATTACKS[WHITE][E4], 0x2800000000);
        assert_eq!(PAWN_ATTACKS[BLACK][D5], 0x14000000);
    }

    #[test]
    fn white_edges() {
        assert_eq!(PAWN_ATTACKS[WHITE][A2], 0x20000);
        assert_eq!(PAWN_ATTACKS[WHITE][H2], 0x400000);
        assert_eq!(PAWN_ATTACKS[WHITE][A7], 0x200000000000000);
        assert_eq!(PAWN_ATTACKS[WHITE][H7], 0x4000000000000000);
    }

    #[test]
    fn black_edges() {
        assert_eq!(PAWN_ATTACKS[BLACK][A2], 0x2);
        assert_eq!(PAWN_ATTACKS[BLACK][H2], 0x40);
        assert_eq!(PAWN_ATTACKS[BLACK][A7], 0x20000000000);
        assert_eq!(PAWN_ATTACKS[BLACK][H7], 0x400000000000);
    }

    #[test]
    fn end_of_board() {
        assert_eq!(PAWN_ATTACKS[WHITE][A1], 0);
        assert_eq!(PAWN_ATTACKS[WHITE][H1], 0);
        assert_eq!(PAWN_ATTACKS[WHITE][A8], 0);
        assert_eq!(PAWN_ATTACKS[WHITE][H8], 0);

        assert_eq!(PAWN_ATTACKS[BLACK][A1], 0);
        assert_eq!(PAWN_ATTACKS[BLACK][H1], 0);
        assert_eq!(PAWN_ATTACKS[BLACK][A8], 0);
        assert_eq!(PAWN_ATTACKS[BLACK][H8], 0);
    }
}
