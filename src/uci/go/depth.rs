use std::collections::VecDeque;

use crate::constants::MATED_VALUE;
use crate::engine::Engine;

pub fn invoke(engine: &mut Engine, mut tokens: VecDeque<String>) {
    if let Some(depth) = tokens.pop_front() {
        if let Ok(depth) = depth.parse::<usize>() {
            let (mv, eval) = engine.start_search(depth);

            let mate = MATED_VALUE.abs() - eval.abs();
            if mate <= 100 {
                let mate = (depth as i64 - mate + 1).div_ceil(2);
                if eval > 0 {
                    println!("info depth {depth} score mate {mate} pv {mv}");
                } else {
                    println!("info depth {depth} score mate -{mate} pv {mv}");
                }
            } else {
                println!("info depth {depth} score cp {eval} pv {mv}");
            }

            println!("bestmove {mv}");
        }
    }
}
