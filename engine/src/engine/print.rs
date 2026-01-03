use std::fmt::Write;

use crate::Move;
use crate::calculated::values::MATE_VALUE;
use crate::engine::MAX_PLY;

pub fn print_info(
    depth: usize,
    nodes: usize,
    score: i32,
    pv_length: usize,
    pv_table: &[Option<Move>; MAX_PLY],
) {
    let mut buffer = String::new();
    write!(buffer, "info depth {depth} nodes {nodes} score ").unwrap();

    let dist = MATE_VALUE - score.abs();

    if dist <= 64_i32 {
        let dist = if score > 0 { dist } else { -dist };
        let mate = if dist > 0 { dist + 1 } else { dist } / 2;
        write!(buffer, "mate {mate} pv ").unwrap();
    } else {
        write!(buffer, "cp {score} pv ").unwrap();
    }

    for pv_move in pv_table.iter().take(pv_length).flatten() {
        write!(buffer, "{} ", pv_move).unwrap();
    }

    println!("{buffer}");
}
