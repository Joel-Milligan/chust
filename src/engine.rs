use crate::board::Board;
use crate::constants::*;
use crate::piece_move::Move;

const KING_VALUE: f64 = 200.0;
const QUEEN_VALUE: f64 = 9.0;
const ROOK_VALUE: f64 = 5.0;
const BISHOP_VALUE: f64 = 3.0;
const KNIGHT_VALUE: f64 = 3.0;
const PAWN_VALUE: f64 = 1.0;

pub struct Engine {
    pub board: Board,
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            board: Board::default(),
        }
    }

    pub fn evaluate(&self, position: &Board) -> f64 {
        let friendly_pieces = position.pieces[position.active_colour];
        let opponent_pieces = position.pieces[!position.active_colour & 1];

        KING_VALUE
            * (friendly_pieces[KING].count_ones() as f64
                - opponent_pieces[KING].count_ones() as f64)
            + QUEEN_VALUE
                * (friendly_pieces[QUEEN].count_ones() as f64
                    - opponent_pieces[QUEEN].count_ones() as f64)
            + ROOK_VALUE
                * (friendly_pieces[ROOK].count_ones() as f64
                    - opponent_pieces[ROOK].count_ones() as f64)
            + BISHOP_VALUE
                * (friendly_pieces[BISHOP].count_ones() as f64
                    - opponent_pieces[BISHOP].count_ones() as f64)
            + KNIGHT_VALUE
                * (friendly_pieces[KNIGHT].count_ones() as f64
                    - opponent_pieces[KNIGHT].count_ones() as f64)
            + PAWN_VALUE
                * (friendly_pieces[PAWN].count_ones() as f64
                    - opponent_pieces[PAWN].count_ones() as f64)
    }

    pub fn start_search(&mut self, initial_depth: usize) -> Move {
        let mut max = f64::MIN;
        let mut best_move = Move::new(0, 0);

        let moves = self.board.moves();
        for mv in moves {
            self.board.make_move(&mv);
            let eval = -self.negamax(initial_depth);
            self.board.unmake_move();

            if eval > max {
                max = eval;
                best_move = mv;
            }
        }

        best_move
    }

    fn negamax(&mut self, depth: usize) -> f64 {
        if depth == 0 {
            return self.evaluate(&self.board);
        }

        let mut max: f64 = f64::MIN;

        let moves = self.board.moves();
        for mv in moves {
            self.board.make_move(&mv);
            let eval = -self.negamax(depth - 1);
            self.board.unmake_move();

            if eval > max {
                max = eval;
            }
        }

        max
    }
}
