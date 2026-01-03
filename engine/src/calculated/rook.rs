#![allow(unused)]
use std::sync::LazyLock;

use crate::repr::constants::*;

pub fn generate_rook_moves(square: u8, blockers: u64) -> u64 {
    let mut moves = 0;

    let rank = square / 8;
    let file = square % 8;

    // N
    if rank < 7 {
        for r in (rank + 1)..=7 {
            moves |= 1 << (r * 8 + file);
            if 1 << (r * 8 + file) & blockers != 0 {
                break;
            }
        }
    }

    // E
    if file < 7 {
        for f in (file + 1)..=7 {
            moves |= 1 << (rank * 8 + f);
            if 1 << (rank * 8 + f) & blockers != 0 {
                break;
            }
        }
    }

    // S
    if rank > 0 {
        for r in (0..=(rank - 1)).rev() {
            moves |= 1 << (r * 8 + file);
            if 1 << (r * 8 + file) & blockers != 0 {
                break;
            }
        }
    }

    // W
    if file > 0 {
        for f in (0..=(file - 1)).rev() {
            moves |= 1 << (rank * 8 + f);
            if 1 << (rank * 8 + f) & blockers != 0 {
                break;
            }
        }
    }

    moves
}

pub static ROOK_MOVES: LazyLock<[u64; 64]> = LazyLock::new(|| {
    let mut rook_moves = [0; 64];

    for square in A1..=H8 {
        let mut moves = 0;

        let rank = square / 8;
        let file = square % 8;

        // N
        if rank < 7 {
            for r in (rank + 1)..=6 {
                moves |= 1 << (r * 8 + file)
            }
        }

        // E
        if file < 7 {
            for f in (file + 1)..=6 {
                moves |= 1 << (rank * 8 + f)
            }
        }

        // S
        if rank > 0 {
            for r in (1..=(rank - 1)).rev() {
                moves |= 1 << (r * 8 + file)
            }
        }

        // W
        if file > 0 {
            for f in (1..=(file - 1)).rev() {
                moves |= 1 << (rank * 8 + f)
            }
        }

        rook_moves[square as usize] = moves;
    }

    rook_moves
});

#[rustfmt::skip]
pub static ROOK_COUNT: [u64; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    12, 11, 11, 11, 11, 11, 11, 12,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn occupancy() {
        assert_eq!(ROOK_MOVES[E4 as usize], 0x1010106e101000);
        assert_eq!(ROOK_MOVES[E1 as usize], 0x1010101010106e);
        assert_eq!(ROOK_MOVES[D8 as usize], 0x7608080808080800);
        assert_eq!(ROOK_MOVES[A4 as usize], 0x101017e010100);
        assert_eq!(ROOK_MOVES[H5 as usize], 0x80807e80808000);
        assert_eq!(ROOK_MOVES[A1 as usize], 0x101010101017e);
        assert_eq!(ROOK_MOVES[H1 as usize], 0x8080808080807e);
        assert_eq!(ROOK_MOVES[A8 as usize], 0x7e01010101010100);
        assert_eq!(ROOK_MOVES[H8 as usize], 0x7e80808080808000);
    }

    #[test]
    fn blockers() {
        assert_eq!(generate_rook_moves(E4, 0x1082001000), 0x10ee101000);
        assert_eq!(generate_rook_moves(A1, 0x102), 0x102)
    }
}
