use std::collections::VecDeque;

use crate::engine::Engine;

mod depth;
mod perft;

pub fn invoke(engine: &mut Engine, mut tokens: VecDeque<String>) {
    if let Some(tk) = tokens.pop_front() {
        match tk.as_str() {
            "depth" => depth::invoke(engine, tokens),
            "wtime" => {
                engine.iterative_deepening(3);
            }
            "evaluate" => {
                println!("{}", engine.evaluate())
            }
            "perft" => perft::invoke(engine, tokens),
            _ => {}
        }
    }
}
