use std::collections::HashMap;

use crate::board::Board;
use crate::constants::*;
use crate::piece_move::Move;

#[derive(Debug, PartialEq, Eq)]
pub enum NodeKind {
    Exact,
    Alpha,
    Beta,
}

#[derive(Debug)]
pub struct Node {
    depth: usize,
    score: i64,
    kind: NodeKind,
}

pub struct Engine {
    pub board: Board,
    pub transposition_table: HashMap<u64, Node>,
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
            transposition_table: HashMap::new(),
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

    pub fn iterative_deepening(&mut self, deepest: usize) {
        for depth in 0..deepest {
            self.start_search(depth);
        }

        let (pv, _) = self.start_search(deepest);

        println!("bestmove {}", pv.first().unwrap());
    }

    fn start_search(&mut self, initial_depth: usize) -> (Vec<Move>, i64) {
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

        print!("info depth {initial_depth} score ");

        let mate = MATED_VALUE.abs() - max_eval.abs();

        if mate <= 100 {
            let mate = (initial_depth as i64 - mate + 1).div_ceil(2);
            let mate = if max_eval > 0 { mate } else { -mate };
            print!("mate {mate} pv ");
        } else {
            print!("cp {max_eval} pv ");
        }

        for mv in &pv {
            print!("{mv} ");
        }
        println!();

        (pv, max_eval)
    }

    fn alpha_beta(
        &mut self,
        alpha: i64,
        beta: i64,
        depth: usize,
        parent_line: Vec<Move>,
    ) -> (Vec<Move>, i64) {
        // Check transposition table for existing entry
        let hash = self.board.zobrist();
        let node = self.transposition_table.get(&hash);

        if let Some(node) = node {
            if node.depth >= depth {
                match node.kind {
                    NodeKind::Exact => {
                        return (parent_line, node.score);
                    }
                    NodeKind::Alpha => {
                        if node.score <= alpha {
                            return (parent_line, alpha);
                        }
                    }
                    NodeKind::Beta => {
                        if node.score >= beta {
                            return (parent_line, beta);
                        }
                    }
                }
            }
        }

        let mut node_kind = NodeKind::Alpha;

        if depth == 0 {
            let score = self.evaluate();
            let node = Node {
                depth,
                score,
                kind: NodeKind::Exact,
            };
            self.transposition_table.insert(hash, node);
            return (parent_line, score);
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
                let node = Node {
                    depth,
                    score: beta,
                    kind: NodeKind::Beta,
                };

                let hash = self.board.zobrist();
                self.transposition_table.insert(hash, node);
                return (line, beta);
            }

            if score > alpha {
                node_kind = NodeKind::Exact;
                alpha = score;
                best_line = line;
            }
        }

        let node = Node {
            depth,
            score: alpha,
            kind: node_kind,
        };
        let hash = self.board.zobrist();
        self.transposition_table.insert(hash, node);

        (best_line, alpha)
    }
}
