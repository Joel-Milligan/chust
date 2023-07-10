use std::sync::LazyLock;

use crate::constants::*;

pub static BISHOP_MOVES: LazyLock<[u64; 64]> = LazyLock::new(|| {
    let mut bishop_moves = [0; 64];

    for square in A1..=H8 {
        let mut moves = 0;

        let rank = square / 8;
        let file = square % 8;

        // NW
        if rank < 7 && file > 0 {
            for (r, f) in ((rank + 1)..=6).zip((1..=(file - 1)).rev()) {
                moves |= 1 << (r * 8 + f)
            }
        }

        // NE
        if rank < 7 && file < 7 {
            for (r, f) in ((rank + 1)..=6).zip((file + 1)..=6) {
                moves |= 1 << (r * 8 + f)
            }
        }

        // SE
        if rank > 0 && file < 7 {
            for (r, f) in (1..=(rank - 1)).rev().zip((file + 1)..=6) {
                moves |= 1 << (r * 8 + f)
            }
        }

        // SW
        if rank > 0 && file > 0 {
            for (r, f) in (1..=(rank - 1)).rev().zip((1..=(file - 1)).rev()) {
                moves |= 1 << (r * 8 + f)
            }
        }

        bishop_moves[square] = moves;
    }

    bishop_moves
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_move_set() {
        assert_eq!(BISHOP_MOVES[E4], 0x2442800284400);
    }

    #[test]
    fn edges() {
        assert_eq!(BISHOP_MOVES[E1], 0x2442800);
        assert_eq!(BISHOP_MOVES[D8], 0x14224000000000);
        assert_eq!(BISHOP_MOVES[A4], 0x8040200020400);
        assert_eq!(BISHOP_MOVES[H5], 0x20400040201000);
    }

    #[test]
    fn corners() {
        assert_eq!(BISHOP_MOVES[A1], 0x40201008040200);
        assert_eq!(BISHOP_MOVES[H1], 0x2040810204000);
        assert_eq!(BISHOP_MOVES[A8], 0x2040810204000);
        assert_eq!(BISHOP_MOVES[H8], 0x40201008040200);
    }
}
