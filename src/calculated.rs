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
    let mut pawn_attacks = [[0; 64]; 2];

    for side in WHITE..=BLACK {
        for square in A2..=H7 {
            let bitboard = 1 << square;
            let mut attacks = 0;

            if side == WHITE {
                attacks |= (bitboard << 7) & !H_FILE;
                attacks |= (bitboard << 9) & !A_FILE;
            } else {
                attacks |= (bitboard >> 7) & !A_FILE;
                attacks |= (bitboard >> 9) & !H_FILE;
            }

            pawn_attacks[side][square] = attacks;
        }
    }

    pawn_attacks
});
