use std::collections::VecDeque;
use std::fmt::Write;

use crate::constants::MATED_VALUE;
use crate::engine::Engine;
use crate::piece_move::Move;

mod depth;
mod perft;

pub fn invoke(engine: &mut Engine, mut tokens: VecDeque<String>) {
    if let Some(tk) = tokens.pop_front() {
        match tk.as_str() {
            "depth" => depth::invoke(engine, tokens),
            "wtime" => iterative_deepening(engine, 3),
            "evaluate" => {
                println!("{}", engine.evaluate())
            }
            "perft" => perft::invoke(engine, tokens),
            _ => {}
        }
    }
}

pub fn iterative_deepening(engine: &mut Engine, deepest: usize) {
    let mut best_move = None;
    for depth in 0..=deepest {
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
