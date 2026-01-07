use crate::repr::Board;
use crate::repr::Move;
use transposition_table::TranspositionTable;

mod evaluate;
mod print;
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

    pub fn reset(&mut self, fen: Option<String>) {
        self.board = Board::from_fen(
            &fen.unwrap_or("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()),
        )
        .unwrap();
        self.tt = TranspositionTable::new();
        self.nodes = 0;
        self.ply = 0;
        self.killer_moves = ([None; MAX_PLY], [None; MAX_PLY]);
        self.history_moves = [[0; 64]; 12];
        self.pv_length = [0; MAX_PLY];
        self.pv_table = [[None; MAX_PLY]; MAX_PLY];
    }
}
