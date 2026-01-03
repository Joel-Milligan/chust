use crate::Engine;

mod depth;
mod perft;

pub fn invoke(engine: &mut Engine, tokens: &[&str]) {
    if let Some((&token, rest)) = tokens.split_first() {
        match token {
            "depth" => depth::invoke(engine, rest),
            // "wtime" => engine.search_depth(3),
            "evaluate" => {
                println!("{}", engine.evaluate())
            }
            "perft" => perft::invoke(engine, rest),
            _ => {}
        }
    }
}
