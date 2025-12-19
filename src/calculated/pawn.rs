use std::sync::LazyLock;

use crate::bitboards::filter;
use crate::constants::*;

pub fn generate_pawn_moves(square: u8, blockers: u64, colour: u8) -> u64 {
    let mut moves = PAWN_MOVES[colour as usize][square as usize];
    let bitboard = 1 << square;

    let rank = square / 8;

    if colour == WHITE && rank == 1 && blockers & bitboard << 8 != 0 {
        moves ^= bitboard << 16;
    } else if colour == BLACK && rank == 6 && blockers & bitboard >> 8 != 0 {
        moves ^= bitboard >> 16;
    }

    moves
}

static PAWN_MOVES: LazyLock<[[u64; 64]; 2]> = LazyLock::new(|| {
    let mut pawn_moves = [[0; 64]; 2];

    for (side, item) in pawn_moves.iter_mut().enumerate().take(BLACK as usize + 1) {
        for (square, mv) in item
            .iter_mut()
            .enumerate()
            .take(H7 as usize + 1)
            .skip(A2 as usize)
        {
            let bitboard = 1 << square;
            let mut moves = 0;

            if side == WHITE as usize {
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

            *mv = moves;
        }
    }

    pawn_moves
});

pub static PAWN_ATTACKS: LazyLock<[[u64; 64]; 2]> = LazyLock::new(|| {
    let mut pawn_attacks = [[0; 64]; 2];

    for (side, item) in pawn_attacks.iter_mut().enumerate().take(BLACK as usize + 1) {
        for (square, mv) in item
            .iter_mut()
            .enumerate()
            .take(H7 as usize + 1)
            .skip(A2 as usize)
        {
            let bitboard = 1 << square;
            let mut attacks = 0;

            if side == WHITE as usize {
                attacks |= filter(bitboard, vec![H_FILE]) << 9;
                attacks |= filter(bitboard, vec![A_FILE]) << 7;
            } else {
                attacks |= filter(bitboard, vec![H_FILE]) >> 7;
                attacks |= filter(bitboard, vec![A_FILE]) >> 9;
            }

            *mv = attacks;
        }
    }

    pawn_attacks
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starting_moves() {
        assert_eq!(PAWN_MOVES[WHITE as usize][E2 as usize], 0x1010_0000);
        assert_eq!(PAWN_MOVES[BLACK as usize][D7 as usize], 0x0808_0000_0000);
    }

    #[test]
    fn normal_moves() {
        assert_eq!(PAWN_MOVES[WHITE as usize][E3 as usize], 0x1000_0000);
        assert_eq!(PAWN_MOVES[BLACK as usize][D6 as usize], 0x0008_0000_0000);
    }

    #[test]
    fn full_attack_set() {
        assert_eq!(PAWN_ATTACKS[WHITE as usize][E4 as usize], 0x0028_0000_0000);
        assert_eq!(PAWN_ATTACKS[BLACK as usize][D5 as usize], 0x1400_0000);
    }

    #[test]
    fn white_edges() {
        assert_eq!(PAWN_ATTACKS[WHITE as usize][A2 as usize], 0x20000);
        assert_eq!(PAWN_ATTACKS[WHITE as usize][H2 as usize], 0x0040_0000);
        assert_eq!(
            PAWN_ATTACKS[WHITE as usize][A7 as usize],
            0x0200_0000_0000_0000
        );
        assert_eq!(
            PAWN_ATTACKS[WHITE as usize][H7 as usize],
            0x4000_0000_0000_0000
        );
    }

    #[test]
    fn black_edges() {
        assert_eq!(PAWN_ATTACKS[BLACK as usize][A2 as usize], 0x2);
        assert_eq!(PAWN_ATTACKS[BLACK as usize][H2 as usize], 0x40);
        assert_eq!(PAWN_ATTACKS[BLACK as usize][A7 as usize], 0x0200_0000_0000);
        assert_eq!(PAWN_ATTACKS[BLACK as usize][H7 as usize], 0x4000_0000_0000);
    }

    #[test]
    fn end_of_board() {
        assert_eq!(PAWN_ATTACKS[WHITE as usize][A1 as usize], 0);
        assert_eq!(PAWN_ATTACKS[WHITE as usize][H1 as usize], 0);
        assert_eq!(PAWN_ATTACKS[WHITE as usize][A8 as usize], 0);
        assert_eq!(PAWN_ATTACKS[WHITE as usize][H8 as usize], 0);

        assert_eq!(PAWN_ATTACKS[BLACK as usize][A1 as usize], 0);
        assert_eq!(PAWN_ATTACKS[BLACK as usize][H1 as usize], 0);
        assert_eq!(PAWN_ATTACKS[BLACK as usize][A8 as usize], 0);
        assert_eq!(PAWN_ATTACKS[BLACK as usize][H8 as usize], 0);
    }
}
