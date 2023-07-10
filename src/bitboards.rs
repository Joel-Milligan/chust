/// Utility functions for working with bitboards

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

/// Counts the number of set bits
pub fn count_bits(mut bitboard: u64) -> u64 {
    let mut count = 0;
    while bitboard != 0 {
        count += 1;
        bitboard &= bitboard - 1;
    }
    count
}

/// Returns the index of the least significant bit. Panics if given an empty bitboard
pub fn lsb_index(bitboard: usize) -> usize {
    const INDEX_64: [usize; 64] = [
        0, 47, 1, 56, 48, 27, 2, 60, 57, 49, 41, 37, 28, 16, 3, 61, 54, 58, 35, 52, 50, 42, 21, 44,
        38, 32, 29, 23, 17, 11, 4, 62, 46, 55, 26, 59, 40, 36, 15, 53, 34, 51, 20, 43, 31, 22, 10,
        45, 25, 39, 14, 33, 19, 30, 9, 24, 13, 18, 8, 12, 7, 6, 5, 63,
    ];

    const DEBRUIJN: usize = 0x03f79d71b4cb0a89;

    assert_ne!(bitboard, 0);

    INDEX_64[((bitboard ^ (bitboard - 1)) * DEBRUIJN) >> 58]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count() {
        assert_eq!(count_bits(0b00110000), 2);
        assert_eq!(count_bits(0b10001101), 4);
        assert_eq!(count_bits(0b00000000), 0);
        assert_eq!(count_bits(0b11111111), 8);
    }

    #[test]
    fn lsb() {
        assert_eq!(lsb_index(0b0010000), 4);
        assert_eq!(lsb_index(0b1100000), 5);
    }
}
