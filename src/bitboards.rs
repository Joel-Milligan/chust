/// Utility functions for working with bitboards

/// Returns a vector of all the indicies with 1 bits
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
