use crate::repr::board::Board;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Score {
    Exact(i32),
    Alpha(i32),
    Beta(i32),
}

#[derive(Clone, Copy, Debug)]
pub struct Node {
    hash: u64,
    depth: usize,
    score: Score,
}

// Statically define size of transposition table to 16 MB
const TABLE_SIZE: usize = 16 * 1024 * 1024;

#[derive(Debug)]
pub struct TranspositionTable(Box<[Option<Node>]>);

impl TranspositionTable {
    pub fn new() -> Self {
        TranspositionTable(vec![None; TABLE_SIZE].into_boxed_slice())
    }

    pub fn get(&self, board: &Board, depth: usize, alpha: i32, beta: i32) -> Option<i32> {
        let probe = &self.0[board.hash as usize % TABLE_SIZE];

        if let Some(node) = probe
            && node.hash == board.hash
            && node.depth >= depth
        {
            return match node.score {
                Score::Exact(score) => Some(score),
                Score::Alpha(score) if score <= alpha => Some(alpha),
                Score::Beta(score) if score >= beta => Some(beta),
                _ => None,
            };
        }
        None
    }

    pub fn insert(&mut self, board: &Board, depth: usize, score: Score) {
        // Using an always replace schema
        self.0[board.hash as usize % TABLE_SIZE] = Some(Node {
            hash: board.hash,
            depth,
            score,
        });
    }
}
