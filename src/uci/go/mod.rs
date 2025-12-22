use std::collections::VecDeque;

use crate::engine::Engine;
use crate::uci::Uci;

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
        let (pv, eval) = engine.search_depth(depth);
        best_move = Some(pv.first().unwrap().clone());
        Uci::write_info(depth, engine.nodes, eval, &pv);
    }

    if let Some(best_move) = best_move {
        println!("bestmove {}", best_move);
    }
}
