use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

use crate::board::Board;

#[derive(Debug, PartialEq, Eq)]
pub enum Score {
    Exact(i32),
    Alpha(i32),
    Beta(i32),
}

#[derive(Debug)]
pub struct Node {
    depth: usize,
    score: Score,
}

#[derive(Debug)]
pub struct TranspositionTable(HashMap<u64, Node>);

impl TranspositionTable {
    pub fn new() -> Self {
        TranspositionTable(HashMap::new())
    }

    pub fn get(&self, board: &Board, depth: usize, alpha: i32, beta: i32) -> Option<i32> {
        let mut hasher = DefaultHasher::new();
        board.hash(&mut hasher);
        let hash = hasher.finish();
        if let Some(node) = self.0.get(&hash) {
            if node.depth >= depth {
                // println!("Hit");
                return match node.score {
                    Score::Exact(score) => Some(score),
                    Score::Alpha(score) if score <= alpha => Some(alpha),
                    Score::Beta(score) if score >= beta => Some(beta),
                    _ => None,
                };
            }
        }
        None
    }

    pub fn insert(&mut self, board: &Board, depth: usize, score: Score) {
        let mut hasher = DefaultHasher::new();
        board.hash(&mut hasher);
        let hash = hasher.finish();
        self.0.insert(hash, Node { depth, score });
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}
