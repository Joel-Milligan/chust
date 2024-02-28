use std::sync::LazyLock;

use crate::bitboards::filter;
use crate::constants::*;

pub fn generate_king_moves(square: usize, blockers: u64, colour: usize, attacked_squares: u64, castling: u8) -> u64 {
    let mut moves = KING_MOVES[square];

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

    for square in A1..=H8 {
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

        king_moves[square] = moves;
    }

    king_moves
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_move_set() {
        assert_eq!(KING_MOVES[E4], 0x3828380000);
        assert_eq!(KING_MOVES[B7], 0x705070000000000);
    }

    #[test]
    fn edges() {
        assert_eq!(KING_MOVES[H5], 0xc040c0000000);
        assert_eq!(KING_MOVES[A4], 0x302030000);
        assert_eq!(KING_MOVES[E1], 0x3828);
        assert_eq!(KING_MOVES[E8], 0x2838000000000000);
    }

    #[test]
    fn corners() {
        assert_eq!(KING_MOVES[A1], 0x302);
        assert_eq!(KING_MOVES[H1], 0xc040);
        assert_eq!(KING_MOVES[A8], 0x203000000000000);
        assert_eq!(KING_MOVES[H8], 0x40c0000000000000);
    }
}
