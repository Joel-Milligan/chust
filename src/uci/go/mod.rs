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
    for depth in 0..=deepest {
        let eval = engine.search_depth(depth);
        Uci::write_info(
            depth,
            engine.nodes,
            eval,
            engine.pv_length[0],
            &engine.pv_table[0],
        );
    }

    if let Some(best_move) = engine.pv_table[0][0] {
        println!("bestmove {}", best_move);
    }
}
