use std::collections::VecDeque;

use crate::engine::Engine;

pub fn invoke(engine: &mut Engine, mut tokens: VecDeque<String>) {
    if let Some(depth) = tokens.pop_front() {
        if let Ok(depth) = depth.parse::<usize>() {
            engine.iterative_deepening(depth);
        }
    }
}
