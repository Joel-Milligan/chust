use crate::board::Board;
use crate::constants::*;
use crate::piece_move::Move;

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

    pub fn evaluate(&self) -> i64 {
        let friend = self.board.pieces[self.board.active_colour];
        let enemy = self.board.pieces[!self.board.active_colour & 1];

        let queens = friend[QUEEN].count_ones() as i64 - enemy[QUEEN].count_ones() as i64;
        let rooks = friend[ROOK].count_ones() as i64 - enemy[ROOK].count_ones() as i64;
        let bishops = friend[BISHOP].count_ones() as i64 - enemy[BISHOP].count_ones() as i64;
        let knights = friend[KNIGHT].count_ones() as i64 - enemy[KNIGHT].count_ones() as i64;
        let pawns = friend[PAWN].count_ones() as i64 - enemy[PAWN].count_ones() as i64;

        QUEEN_VALUE * queens
            + ROOK_VALUE * rooks
            + BISHOP_VALUE * bishops
            + KNIGHT_VALUE * knights
            + PAWN_VALUE * pawns
    }

    pub fn start_search(&mut self, initial_depth: usize) -> (Move, i64) {
        let mut max_eval = MATED_VALUE;
        let mut best_move = Move::new(0, 0);

        let moves = self.board.moves();
        for mv in moves {
            self.board.make_move(&mv);
            let eval = -self.alpha_beta(MATED_VALUE, i64::MAX, initial_depth);
            self.board.unmake_move();

            if eval > max_eval {
                max_eval = eval;
                best_move = mv;
            }
        }

        (best_move, max_eval)
    }

    fn alpha_beta(&mut self, alpha: i64, beta: i64, depth: usize) -> i64 {
        if depth == 0 {
            return self.evaluate();
        }

        let mut alpha = alpha;

        let moves = self.board.moves();

        if moves.is_empty() {
            if self.board.in_check() {
                return MATED_VALUE + depth as i64;
            }
            return 0;
        }

        for mv in moves {
            self.board.make_move(&mv);
            let score = -self.alpha_beta(-beta, -alpha, depth - 1);
            self.board.unmake_move();

            if score >= beta {
                return beta;
            }

            if score > alpha {
                alpha = score;
            }
        }

        alpha
    }

    fn quiescense(&mut self, alpha: i64, beta: i64) -> i64 {
        let eval = self.evaluate();

        if eval >= beta {
            return eval;
        }

        let mut alpha = alpha;

        if alpha < eval {
            alpha = eval;
        }

        let captures = self
            .board
            .moves()
            .into_iter()
            .filter(|mv| self.board.get_piece_at_square(mv.destination.0).is_some())
            .collect::<Vec<_>>();

        for capture in captures {
            self.board.make_move(&capture);
            let eval = -self.quiescense(-beta, -alpha);
            self.board.unmake_move();

            if eval > alpha {
                if eval >= beta {
                    return beta;
                }

                alpha = eval;
            }
        }

        alpha
    }
}
