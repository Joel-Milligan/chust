use std::sync::LazyLock;

use crate::constants::*;

pub fn generate_bishop_moves(square: usize, blockers: u64) -> u64 {
    let mut moves = 0;

    let rank = square / 8;
    let file = square % 8;

    // NW
    if rank < 7 && file > 0 {
        for (r, f) in ((rank + 1)..=7).zip((0..=(file - 1)).rev()) {
            moves |= 1 << (r * 8 + f);
            if 1 << (r * 8 + f) & blockers != 0 {
                break;
            }
        }
    }

    // NE
    if rank < 7 && file < 7 {
        for (r, f) in ((rank + 1)..=7).zip((file + 1)..=7) {
            moves |= 1 << (r * 8 + f);
            if 1 << (r * 8 + f) & blockers != 0 {
                break;
            }
        }
    }

    // SE
    if rank > 0 && file < 7 {
        for (r, f) in (0..=(rank - 1)).rev().zip((file + 1)..=7) {
            moves |= 1 << (r * 8 + f);
            if 1 << (r * 8 + f) & blockers != 0 {
                break;
            }
        }
    }

    // SW
    if rank > 0 && file > 0 {
        for (r, f) in (0..=(rank - 1)).rev().zip((0..=(file - 1)).rev()) {
            moves |= 1 << (r * 8 + f);
            if 1 << (r * 8 + f) & blockers != 0 {
                break;
            }
        }
    }

    moves
}

pub static BISHOP_MOVES: LazyLock<[u64; 64]> = LazyLock::new(|| {
    let mut moves = [0; 64];

    for square in A1..=H8 {
        let mut squares = 0;

        let rank = square / 8;
        let file = square % 8;

        // NW
        if rank < 7 && file > 0 {
            for (r, f) in ((rank + 1)..=7).zip((0..=(file - 1)).rev()) {
                squares |= 1 << (r * 8 + f)
            }
        }

        // NE
        if rank < 7 && file < 7 {
            for (r, f) in ((rank + 1)..=7).zip((file + 1)..=7) {
                squares |= 1 << (r * 8 + f)
            }
        }

        // SE
        if rank > 0 && file < 7 {
            for (r, f) in (0..=(rank - 1)).rev().zip((file + 1)..=7) {
                squares |= 1 << (r * 8 + f)
            }
        }

        // SW
        if rank > 0 && file > 0 {
            for (r, f) in (0..=(rank - 1)).rev().zip((0..=(file - 1)).rev()) {
                squares |= 1 << (r * 8 + f)
            }
        }

        moves[square] = squares;
    }

    moves
});

pub static BISHOP_OCCUPANCY: LazyLock<[u64; 64]> = LazyLock::new(|| {
    let mut occupancy = [0; 64];

    for square in A1..=H8 {
        let mut squares = 0;

        let rank = square / 8;
        let file = square % 8;

        // NW
        if rank < 7 && file > 0 {
            for (r, f) in ((rank + 1)..=6).zip((1..=(file - 1)).rev()) {
                squares |= 1 << (r * 8 + f)
            }
        }

        // NE
        if rank < 7 && file < 7 {
            for (r, f) in ((rank + 1)..=6).zip((file + 1)..=6) {
                squares |= 1 << (r * 8 + f)
            }
        }

        // SE
        if rank > 0 && file < 7 {
            for (r, f) in (1..=(rank - 1)).rev().zip((file + 1)..=6) {
                squares |= 1 << (r * 8 + f)
            }
        }

        // SW
        if rank > 0 && file > 0 {
            for (r, f) in (1..=(rank - 1)).rev().zip((1..=(file - 1)).rev()) {
                squares |= 1 << (r * 8 + f)
            }
        }

        occupancy[square] = squares;
    }

    occupancy
});

pub static BISHOP_COUNT: [u64; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 7, 7, 7, 7, 5, 5, 5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5, 5, 5, 7, 7, 7, 7, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 5, 5, 5, 5, 5, 5, 6,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn occupancy() {
        assert_eq!(BISHOP_OCCUPANCY[E4], 0x2442800284400);
        assert_eq!(BISHOP_OCCUPANCY[E1], 0x2442800);
        assert_eq!(BISHOP_OCCUPANCY[D8], 0x14224000000000);
        assert_eq!(BISHOP_OCCUPANCY[A4], 0x8040200020400);
        assert_eq!(BISHOP_OCCUPANCY[H5], 0x20400040201000);
        assert_eq!(BISHOP_OCCUPANCY[A1], 0x40201008040200);
        assert_eq!(BISHOP_OCCUPANCY[H1], 0x2040810204000);
        assert_eq!(BISHOP_OCCUPANCY[A8], 0x2040810204000);
        assert_eq!(BISHOP_OCCUPANCY[H8], 0x40201008040200);
    }

    #[test]
    fn moves() {
        assert_eq!(BISHOP_MOVES[E4], 0x182442800284482);
        assert_eq!(BISHOP_MOVES[E1], 0x182442800);
        // assert_eq!(BISHOP_MOVES[D8], 0x14224000000000);
        // assert_eq!(BISHOP_MOVES[A4], 0x8040200020400);
        // assert_eq!(BISHOP_MOVES[H5], 0x20400040201000);
        assert_eq!(BISHOP_MOVES[A1], 0x8040201008040200);
        // assert_eq!(BISHOP_MOVES[H1], 0x2040810204000);
        // assert_eq!(BISHOP_MOVES[A8], 0x2040810204000);
        // assert_eq!(BISHOP_MOVES[H8], 0x40201008040200);
    }

    #[test]
    fn blockers() {
        assert_eq!(
            generate_bishop_moves(E4, 0x100400000084000),
            0x102442800284000
        )
    }
}
