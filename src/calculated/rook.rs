use std::sync::LazyLock;

use crate::constants::*;

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

        rook_moves[square] = moves;
    }

    rook_moves
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_move_set() {
        assert_eq!(ROOK_MOVES[E4], 0x1010106e101000);
    }

    #[test]
    fn edges() {
        assert_eq!(ROOK_MOVES[E1], 0x1010101010106e);
        assert_eq!(ROOK_MOVES[D8], 0x7608080808080800);
        assert_eq!(ROOK_MOVES[A4], 0x101017e010100);
        assert_eq!(ROOK_MOVES[H5], 0x80807e80808000);
    }

    #[test]
    fn corners() {
        assert_eq!(ROOK_MOVES[A1], 0x101010101017e);
        assert_eq!(ROOK_MOVES[H1], 0x8080808080807e);
        assert_eq!(ROOK_MOVES[A8], 0x7e01010101010100);
        assert_eq!(ROOK_MOVES[H8], 0x7e80808080808000);
    }
}
