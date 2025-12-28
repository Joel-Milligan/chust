//! Utility functions for working with bitboards

/// Returns a vector of all the indicies with an ON bit
pub fn bit_indicies(mut bitboard: u64) -> Vec<usize> {
    let mut indicies = vec![];

    for i in 0..64 {
        if bitboard & 1 == 1 {
            indicies.push(i);
        }
        bitboard >>= 1;
    }

    indicies
}

/// Filter out certain squares from a bitboard
pub fn filter(bitboard: u64, filters: Vec<u64>) -> u64 {
    bitboard & !(filters.into_iter().reduce(|acc, e| acc | e).unwrap())
}

/// Nicely prints out a given 64 bit number as a chess bitboard
pub fn print_bitboard(bitboard: u64) {
    for rank in (0..8).rev() {
        print!("{} ", rank + 1);
        for file in 0..8 {
            let square = rank * 8 + file;
            let occupied = bitboard & (1u64 << square) != 0;
            print!("{} ", if occupied { "o" } else { "." })
        }
        println!();
    }
    println!("  a b c d e f g h");
}

/// Indexes into all possible occupancy configurations for a given mask
pub fn get_occupancies(index: usize, mut mask: u64) -> u64 {
    let mut occupancy = 0;
    let bits = mask.count_ones();

    for count in 0..bits {
        println!("Mask: {mask}");
        let square = mask.trailing_zeros();

        mask &= !(1 << square);

        if (index & (1 << count)) != 0 {
            occupancy |= 1 << square;
        }
    }

    occupancy
}

#[cfg(test)]
mod tests {
    use crate::calculated::rook::*;
    use crate::constants::*;

    use super::*;

    #[test]
    fn occupancy() {
        assert_eq!(get_occupancies(0, ROOK_MOVES[A1 as usize]), 0);
        assert_eq!(
            get_occupancies(4095, ROOK_MOVES[A1 as usize]),
            0x101010101017e
        );
    }
}
