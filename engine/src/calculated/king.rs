use std::sync::LazyLock;

use crate::repr::bitboards::*;
use crate::repr::constants::*;

pub fn generate_king_moves(
    square: u8,
    blockers: u64,
    colour: u8,
    attacked_squares: u64,
    castling: u8,
) -> u64 {
    let mut moves = KING_MOVES[square as usize];

    // Castling
    if colour == WHITE {
        let blocking_king_side = blockers & (1 << F1 | 1 << G1);
        let attacking_king_side = attacked_squares & (1 << E1 | 1 << F1 | 1 << G1);
        if blocking_king_side | attacking_king_side == 0 && castling & WHITE_KING_SIDE != 0 {
            moves |= 1 << G1;
        }

        let blocking_queen_side = blockers & (1 << B1 | 1 << C1 | 1 << D1);
        let attacking_queen_side = attacked_squares & (1 << C1 | 1 << D1 | 1 << E1);
        if blocking_queen_side | attacking_queen_side == 0 && castling & WHITE_QUEEN_SIDE != 0 {
            moves |= 1 << C1;
        }
    } else {
        let blocking_king_side = blockers & (1 << F8 | 1 << G8);
        let attacking_king_side = attacked_squares & (1 << E8 | 1 << F8 | 1 << G8);
        if blocking_king_side | attacking_king_side == 0 && castling & BLACK_KING_SIDE != 0 {
            moves |= 1 << G8;
        }

        let blocking_queen_side = blockers & (1 << B8 | 1 << C8 | 1 << D8);
        let attacking_queen_side = attacked_squares & (1 << C8 | 1 << D8 | 1 << E8);
        if blocking_queen_side | attacking_queen_side == 0 && castling & BLACK_QUEEN_SIDE != 0 {
            moves |= 1 << C8;
        }
    }

    moves
}

pub static KING_MOVES: LazyLock<[u64; 64]> = LazyLock::new(|| {
    let mut king_moves = [0; 64];

    for (square, mv) in king_moves.iter_mut().enumerate().take(H8 as usize + 1) {
        let bitboard = 1 << square;
        let mut moves = 0;

        // Start from NNW and go clockwise
        moves |= filter(bitboard, vec![A_FILE, EIGHTH_RANK]) << 7;
        moves |= filter(bitboard, vec![EIGHTH_RANK]) << 8;
        moves |= filter(bitboard, vec![H_FILE, EIGHTH_RANK]) << 9;
        moves |= filter(bitboard, vec![H_FILE]) << 1;
        moves |= filter(bitboard, vec![H_FILE, FIRST_RANK]) >> 7;
        moves |= filter(bitboard, vec![FIRST_RANK]) >> 8;
        moves |= filter(bitboard, vec![A_FILE, FIRST_RANK]) >> 9;
        moves |= filter(bitboard, vec![A_FILE]) >> 1;

        *mv = moves;
    }

    king_moves
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_move_set() {
        assert_eq!(KING_MOVES[E4 as usize], 0x0038_2838_0000);
        assert_eq!(KING_MOVES[B7 as usize], 0x0705_0700_0000_0000);
    }

    #[test]
    fn edges() {
        assert_eq!(KING_MOVES[H5 as usize], 0xc040_c000_0000);
        assert_eq!(KING_MOVES[A4 as usize], 0x0003_0203_0000);
        assert_eq!(KING_MOVES[E1 as usize], 0x3828);
        assert_eq!(KING_MOVES[E8 as usize], 0x2838_0000_0000_0000);
    }

    #[test]
    fn corners() {
        assert_eq!(KING_MOVES[A1 as usize], 0x302);
        assert_eq!(KING_MOVES[H1 as usize], 0xc040);
        assert_eq!(KING_MOVES[A8 as usize], 0x0203_0000_0000_0000);
        assert_eq!(KING_MOVES[H8 as usize], 0x40c0_0000_0000_0000);
    }
}
