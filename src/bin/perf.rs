#![feature(int_roundings)]
use chust::board::Board;
use chust::constants::MATED_VALUE;
use chust::engine::Engine;
use chust::piece_move::Move;

use std::fmt::Write;

fn main() {
    let board =
        Board::from_fen("rn3r1k/p3qp2/bp2p2p/3pP3/P2NRQ2/1Pb2NPP/5PB1/2R3K1 w - - 1 22").unwrap();
    let mut engine = Engine::new();
    engine.board = board;

    let mut best_move = None;
    for depth in 0..=5 {
        let (pv, eval) = engine.start_search(depth);
        best_move = Some(pv.first().unwrap().clone());
        write_info(depth, eval, &pv);
    }

    if let Some(best_move) = best_move {
        println!("bestmove {}", best_move);
    }
}

fn write_info(initial_depth: usize, max_eval: i32, pv: &Vec<Move>) {
    let mut buffer = String::new();
    write!(buffer, "info depth {initial_depth} score ").unwrap();

    let mate = MATED_VALUE.abs() - max_eval.abs();

    if mate <= 100 {
        let mate = (initial_depth as i32 - mate + 1).div_ceil(2);
        let mate = if max_eval > 0 { mate } else { -mate };
        write!(buffer, "mate {mate} pv ").unwrap();
    } else {
        write!(buffer, "cp {max_eval} pv ").unwrap();
    }

    for mv in pv {
        write!(buffer, "{mv} ").unwrap();
    }

    println!("{buffer}");
}
