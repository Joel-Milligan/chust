use std::collections::VecDeque;

use crate::engine::Engine;
use crate::uci::go::iterative_deepening;

pub fn invoke(engine: &mut Engine, mut tokens: VecDeque<String>) {
    if let Some(depth) = tokens.pop_front()
        && let Ok(depth) = depth.parse::<usize>()
    {
        iterative_deepening(engine, depth);
    }
}
