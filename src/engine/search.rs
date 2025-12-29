use crate::constants::*;
use crate::engine::transposition_table::Score;
use crate::engine::{Engine, MAX_PLY};
use crate::piece_move::Move;
use crate::uci::Uci;

impl Engine {
    pub fn search_depth(&mut self, depth: usize) {
        self.nodes = 0;
        self.killer_moves = ([None; MAX_PLY], [None; MAX_PLY]);
        self.history_moves = [[0; 64]; 12];
        self.pv_length = [0; MAX_PLY];
        self.pv_table = [[None; MAX_PLY]; MAX_PLY];

        for current_depth in 1..=depth {
            let eval = self.alpha_beta(current_depth, -20_000, 20_000);
            Uci::print_info(
                current_depth,
                self.nodes,
                eval,
                self.pv_length[0],
                &self.pv_table[0],
            );
        }

        if let Some(best_move) = self.pv_table[0][0] {
            println!("bestmove {}", best_move);
        }
    }

    fn score_move(&mut self, mv: &Move) -> i32 {
        if self.pv_table[0][self.ply] == Some(*mv) {
            return 20_000;
        }

        let (_, piece) = self.board.squares[mv.source.0 as usize]
            .expect("valid moves always have a piece at source");

        // FIXME: Does not identify en passant captures
        if let Some((_, victim)) = self.board.squares[mv.destination.0 as usize] {
            // Captures
            MVV_LVA[piece as usize][victim as usize] + 10_000
        } else {
            // Quiet
            if self.killer_moves.0[self.board.half_moves as usize] == Some(*mv) {
                9000
            } else if self.killer_moves.1[self.board.half_moves as usize] == Some(*mv) {
                8000
            } else {
                self.history_moves[piece as usize][mv.destination.0 as usize]
            }
        }
    }

    fn alpha_beta(&mut self, depth: usize, mut alpha: i32, beta: i32) -> i32 {
        if let Some(score) = self.tt.get(&self.board, depth, alpha, beta) {
            return score;
        }

        self.pv_length[self.ply] = self.ply;

        if depth == 0 {
            let eval = self.quiescence(alpha, beta);
            self.tt.insert(&self.board, depth, Score::Exact(eval));
            return eval;
        }

        let mut score = Score::Alpha(alpha);
        let mut found_pv = false;

        self.nodes += 1;

        let moves = self.board.moves();
        if moves.is_empty() {
            if self.board.in_check() {
                return -MATE_VALUE + self.ply as i32;
            }
            return 0;
        }

        let mut sorted_moves = self.board.moves();
        sorted_moves.sort_by_key(|mv| std::cmp::Reverse(self.score_move(mv)));
        for mv in sorted_moves {
            self.board.make_move(&mv);
            self.ply += 1;

            let eval = if found_pv {
                let eval = -self.alpha_beta(depth - 1, -alpha - 1, -alpha);
                if (eval > alpha) && (eval < beta) {
                    // Failed to prove move is worse than current, re-search normally
                    -self.alpha_beta(depth - 1, -beta, -alpha)
                } else {
                    eval
                }
            } else {
                -self.alpha_beta(depth - 1, -beta, -alpha)
            };

            self.board.unmake_move();
            self.ply -= 1;

            if eval >= beta {
                self.tt.insert(&self.board, depth, Score::Beta(beta));

                if self.board.squares[mv.destination.0 as usize].is_none() {
                    self.killer_moves.1[self.ply] = self.killer_moves.0[self.ply];
                    self.killer_moves.0[self.ply] = Some(mv);
                }

                return beta;
            }

            if eval > alpha {
                if self.board.squares[mv.destination.0 as usize].is_none() {
                    let (_, piece) = self.board.squares[mv.source.0 as usize]
                        .expect("all valid moves have a piece at source");
                    self.history_moves[piece as usize][mv.destination.0 as usize] += depth as i32;
                }

                alpha = eval;
                score = Score::Exact(alpha);
                found_pv = true;

                self.pv_table[self.ply][self.ply] = Some(mv);
                for next_ply in (self.ply + 1)..self.pv_length[self.ply + 1] {
                    self.pv_table[self.ply][next_ply] = self.pv_table[self.ply + 1][next_ply];
                }
                self.pv_length[self.ply] = self.pv_length[self.ply + 1];
            }
        }

        self.tt.insert(&self.board, depth, score);

        alpha
    }

    fn quiescence(&mut self, mut alpha: i32, beta: i32) -> i32 {
        self.nodes += 1;

        let moves = self.board.moves();
        if moves.is_empty() {
            if self.board.in_check() {
                return -MATE_VALUE + self.ply as i32;
            }
            return 0;
        }

        let eval = self.evaluate();
        if eval >= beta {
            return beta;
        }
        if eval > alpha {
            alpha = eval;
        }

        // FIXME: Does not identify en passant captures
        let mut captures = moves
            .into_iter()
            .filter(|x| self.board.squares[x.destination.0 as usize].is_some())
            .collect::<Vec<_>>();
        captures.sort_by_key(|mv| std::cmp::Reverse(self.score_move(mv)));
        for mv in captures {
            self.board.make_move(&mv);
            self.ply += 1;
            let eval = -self.quiescence(-beta, -alpha);
            self.board.unmake_move();
            self.ply -= 1;

            if eval >= beta {
                return beta;
            }
            if eval > alpha {
                alpha = eval;
            }
        }
        alpha
    }
}
