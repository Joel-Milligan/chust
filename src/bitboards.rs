/// Utility functions for working with bitboards

/// Returns a vector of all the indicies with 1 bit
pub fn indicies(bitboard: &u64) -> Vec<usize> {
    let mut indicies = vec![];
    let mut bb = bitboard.clone();

    for i in 0..64 {
        if bb & 1 == 1 {
            indicies.push(i);
        }
        bb >>= 1;
    }

    indicies
}

pub fn print_bitboard(bitboard: u64) {
    for rank in (0..8).rev() {
        print!("{} ", rank + 1);
        for file in 0..8 {
            let square = rank * 8 + file;
            let occupied = bitboard & (1u64 << square) != 0;
            print!(" {} ", if occupied { "o" } else { "." })
        }
        println!();
    }
    println!("   a  b  c  d  e  f  g  h");
}
