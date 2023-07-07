use std::sync::LazyLock;

use crate::constants::*;

pub static ROOK_MOVES: LazyLock<[u64; 64]> = LazyLock::new(|| {
    let mut rook_moves = [0; 64];

    for square in 0..64 {
        let mut moves = 0;

        // NORTH
        if square <= H7 {
            let mut ray = square + 8;
            while ray <= H7 {
                moves |= 1 << ray;
                ray += 8;
            }
            moves |= 1 << ray;
        }

        // SOUTH
        if square >= A2 {
            let mut ray = square - 8;
            while ray >= A2 {
                moves |= 1 << ray;
                ray -= 8;
            }
            moves |= 1 << ray;
        }

        // EAST
        if square % 8 != 7 {
            let mut ray = square + 1;
            while ray % 8 != 7 {
                moves |= 1 << ray;
                ray += 1;
            }
            moves |= 1 << ray;
        }

        // WEST
        if square % 8 != 0 {
            let mut ray = square - 1;
            while ray % 8 != 0 {
                moves |= 1 << ray;
                ray -= 1;
            }
            moves |= 1 << ray;
        }

        rook_moves[square] = moves;
    }

    rook_moves
});

pub static PAWN_ATTACKS: LazyLock<[[u64; 64]; 2]> = LazyLock::new(|| {
    let mut attacks = [[0; 64]; 2];

    for side in 0..2 {
        for square in 8..56 {
            attacks[side][square] = mask_pawn_attacks(side, square);
        }
    }

    attacks
});

fn mask_pawn_attacks(side: usize, square: usize) -> u64 {
    let mut attacks = 0u64;
    let mut bitboard = 0u64;

    bitboard |= 1 << square;

    if side == WHITE {
        if square % 8 != 0 {
            attacks |= bitboard << 7;
        }
        if square % 8 != 7 {
            attacks |= bitboard << 9;
        }
    } else {
        if square % 8 != 0 {
            attacks |= bitboard >> 9;
        }
        if square % 8 != 7 {
            attacks |= bitboard >> 7;
        }
    }

    attacks
}