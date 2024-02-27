use std::sync::LazyLock;

use crate::bitboards::filter;
use crate::constants::*;

pub fn generate_pawn_moves(square: usize, blockers: u64, colour: usize) -> u64 {
    let mut moves = PAWN_MOVES[colour][square];
    let bitboard = 1 << square;

    let rank = square / 8;

    if colour == WHITE && rank == 1 && blockers & bitboard << 8 != 0 {
        moves ^= bitboard << 16;
    } else if colour == BLACK && rank == 8 && blockers & bitboard >> 8 != 0 {
        moves ^= bitboard >> 16;
    }

    moves
}

static PAWN_MOVES: LazyLock<[[u64; 64]; 2]> = LazyLock::new(|| {
    let mut pawn_moves = [[0; 64]; 2];

    for side in WHITE..=BLACK {
        for square in A2..=H7 {
            let bitboard = 1 << square;
            let mut moves = 0;

            if side == WHITE {
                moves |= bitboard << 8;
                if bitboard & SECOND_RANK != 0 {
                    moves |= bitboard << 16;
                }
            } else {
                moves |= bitboard >> 8;
                if bitboard & SEVENTH_RANK != 0 {
                    moves |= bitboard >> 16;
                }
            }

            pawn_moves[side][square] = moves;
        }
    }

    pawn_moves
});

pub static PAWN_ATTACKS: LazyLock<[[u64; 64]; 2]> = LazyLock::new(|| {
    let mut pawn_attacks = [[0; 64]; 2];

    for side in WHITE..=BLACK {
        for square in A2..=H7 {
            let bitboard = 1 << square;
            let mut attacks = 0;

            if side == WHITE {
                attacks |= filter(bitboard, vec![H_FILE]) << 9;
                attacks |= filter(bitboard, vec![A_FILE]) << 7;
            } else {
                attacks |= filter(bitboard, vec![H_FILE]) >> 7;
                attacks |= filter(bitboard, vec![A_FILE]) >> 9;
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
    fn starting_moves() {
        assert_eq!(PAWN_MOVES[WHITE][E2], 0x10100000);
        assert_eq!(PAWN_MOVES[BLACK][D7], 0x80800000000);
    }

    #[test]
    fn normal_moves() {
        assert_eq!(PAWN_MOVES[WHITE][E3], 0x10000000);
        assert_eq!(PAWN_MOVES[BLACK][D6], 0x800000000);
    }

    #[test]
    fn full_attack_set() {
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
