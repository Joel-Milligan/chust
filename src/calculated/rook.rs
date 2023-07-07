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
