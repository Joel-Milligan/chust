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

    pub fn start_search(&mut self, initial_depth: usize) -> (Vec<Move>, i64) {
        let mut max_eval = MATED_VALUE;
        let mut pv = vec![];

        let moves = self.board.moves();
        for mv in moves {
            self.board.make_move(&mv);
            let (line, neg_eval) =
                self.alpha_beta(MATED_VALUE, i64::MAX, initial_depth, vec![mv.clone()]);
            let eval = -neg_eval;
            self.board.unmake_move();

            if eval > max_eval {
                max_eval = eval;
                pv = line;
            }
        }

        (pv, max_eval)
    }

    fn alpha_beta(
        &mut self,
        alpha: i64,
        beta: i64,
        depth: usize,
        parent_line: Vec<Move>,
    ) -> (Vec<Move>, i64) {
        if depth == 0 {
            return (parent_line, self.evaluate());
        }

        let mut alpha = alpha;
        let mut best_line = parent_line.clone();

        let moves = self.board.moves();

        if moves.is_empty() {
            if self.board.in_check() {
                return (parent_line, MATED_VALUE + depth as i64);
            }
            return (parent_line, 0);
        }

        for mv in moves {
            let mut line = parent_line.clone();
            line.push(mv.clone());
            self.board.make_move(&mv);
            let (line, neg_score) = self.alpha_beta(-beta, -alpha, depth - 1, line);
            let score = -neg_score;
            self.board.unmake_move();

            if score >= beta {
                return (line, beta);
            }

            if score > alpha {
                alpha = score;
                best_line = line;
            }
        }

        (best_line, alpha)
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
