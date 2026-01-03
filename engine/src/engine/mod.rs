use crate::engine::transposition_table::TranspositionTable;
use crate::repr::board::Board;
use crate::repr::piece_move::Move;

mod evaluate;
mod search;
mod transposition_table;

pub const MAX_PLY: usize = 64;

pub struct Engine {
    pub board: Board,
    pub tt: TranspositionTable,
    pub nodes: usize,
    pub ply: usize,
    pub killer_moves: ([Option<Move>; MAX_PLY], [Option<Move>; MAX_PLY]),
    pub history_moves: [[i32; 64]; 12],
    pub pv_length: [usize; MAX_PLY],
    pub pv_table: [[Option<Move>; MAX_PLY]; MAX_PLY],
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

impl Engine {
    pub fn new() -> Self {
        Self {
            board: Board::default(),
            tt: TranspositionTable::new(),
            nodes: 0,
            ply: 0,
            killer_moves: ([None; MAX_PLY], [None; MAX_PLY]),
            history_moves: [[0; 64]; 12],
            pv_length: [0; MAX_PLY],
            pv_table: [[None; MAX_PLY]; MAX_PLY],
        }
    }
}
