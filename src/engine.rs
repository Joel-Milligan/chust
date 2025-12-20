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
    score: i32,
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

    pub fn evaluate(&self) -> i32 {
        let friend = self.board.pieces[self.board.active_colour as usize];
        let enemy = self.board.pieces[1 - self.board.active_colour as usize];

        let queens = friend[QUEEN as usize].count_ones() - enemy[QUEEN as usize].count_ones();
        let rooks = friend[ROOK as usize].count_ones() - enemy[ROOK as usize].count_ones();
        let bishops = friend[BISHOP as usize].count_ones() - enemy[BISHOP as usize].count_ones();
        let knights = friend[KNIGHT as usize].count_ones() - enemy[KNIGHT as usize].count_ones();
        let pawns = friend[PAWN as usize].count_ones() - enemy[PAWN as usize].count_ones();

        QUEEN_VALUE * queens as i32
            + ROOK_VALUE * rooks as i32
            + BISHOP_VALUE * bishops as i32
            + KNIGHT_VALUE * knights as i32
            + PAWN_VALUE * pawns as i32
    }

    pub fn start_search(&mut self, initial_depth: usize) -> (Vec<Move>, i32) {
        let mut max_eval = MATED_VALUE;
        let mut pv = vec![];

        let moves = self.board.moves();
        for mv in moves {
            self.board.make_move(&mv);
            let (line, neg_eval) = self.alpha_beta(MATED_VALUE, i32::MAX, initial_depth, vec![mv]);
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
        alpha: i32,
        beta: i32,
        depth: usize,
        parent_line: Vec<Move>,
    ) -> (Vec<Move>, i32) {
        // Check transposition table for existing entry
        let hash = self.board.zobrist();
        let node = self.transposition_table.get(&hash);

        if let Some(node) = node
            && node.depth >= depth
        {
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
                return (parent_line, MATED_VALUE + depth as i32);
            }
            return (parent_line, 0);
        }

        for mv in moves {
            let mut line = parent_line.clone();
            line.push(mv);
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
