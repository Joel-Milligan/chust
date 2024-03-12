use std::collections::VecDeque;

use crate::engine::Engine;

mod depth;
mod perft;

pub fn invoke(engine: &mut Engine, mut tokens: VecDeque<String>) {
    if let Some(tk) = tokens.pop_front() {
        match tk.as_str() {
            "depth" => depth::invoke(engine, tokens),
            "wtime" => {
                let (pv, _) = engine.start_search(3);
                print!("info depth 3 pv ");
                for mv in &pv {
                    print!("{mv} ");
                }
                println!();
                println!("bestmove {}", pv.first().unwrap());
            }
            "evaluate" => {
                println!("{}", engine.evaluate())
            }
            "perft" => perft::invoke(engine, tokens),
            _ => {}
        }
    }
}
