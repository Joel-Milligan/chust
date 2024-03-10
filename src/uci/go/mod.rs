use std::collections::VecDeque;

use crate::engine::Engine;

mod depth;
mod perft;

pub fn invoke(engine: &mut Engine, mut tokens: VecDeque<String>) {
    if let Some(tk) = tokens.pop_front() {
        match tk.as_str() {
            "depth" => depth::invoke(engine, tokens),
            "wtime" => {
                let (mv, _) = engine.start_search(3);
                println!("info depth 3 pv {mv}");
                println!("bestmove {mv}");
            }
            "evaluate" => {
                println!("{}", engine.evaluate())
            }
            "perft" => perft::invoke(engine, tokens),
            _ => {}
        }
    }
}
