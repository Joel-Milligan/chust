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

        let queens =
            friend[QUEEN as usize].count_ones() as i8 - enemy[QUEEN as usize].count_ones() as i8;
        let rooks =
            friend[ROOK as usize].count_ones() as i8 - enemy[ROOK as usize].count_ones() as i8;
        let bishops =
            friend[BISHOP as usize].count_ones() as i8 - enemy[BISHOP as usize].count_ones() as i8;
        let knights =
            friend[KNIGHT as usize].count_ones() as i8 - enemy[KNIGHT as usize].count_ones() as i8;
        let pawns =
            friend[PAWN as usize].count_ones() as i8 - enemy[PAWN as usize].count_ones() as i8;

        QUEEN_VALUE * queens as i32
            + ROOK_VALUE * rooks as i32
            + BISHOP_VALUE * bishops as i32
            + KNIGHT_VALUE * knights as i32
            + PAWN_VALUE * pawns as i32
    }

    pub fn start_search(&mut self, initial_depth: usize) -> (Vec<Move>, i32) {
        let mut max_eval = MATED_VALUE;
        let mut pv = vec![];
        let mut line = vec![];
        for mv in self.board.moves() {
            self.board.make_move(&mv);
            let neg_eval = self.alpha_beta(MATED_VALUE, i32::MAX, initial_depth, &mut line);
            let eval = -neg_eval;
            self.board.unmake_move();

            if eval > max_eval {
                max_eval = eval;
                update_line(&mut pv, mv, &line);
            }
        }

        (pv, max_eval)
    }

    fn alpha_beta(
        &mut self,
        alpha: i32,
        beta: i32,
        depth: usize,
        parent_line: &mut Vec<Move>,
    ) -> i32 {
        // Check transposition table for existing entry
        let hash = self.board.zobrist();
        let node = self.transposition_table.get(&hash);

        if let Some(node) = node
            && node.depth >= depth
        {
            match node.kind {
                NodeKind::Exact => {
                    return node.score;
                }
                NodeKind::Alpha => {
                    if node.score <= alpha {
                        return alpha;
                    }
                }
                NodeKind::Beta => {
                    if node.score >= beta {
                        return beta;
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
            return score;
        }

        let mut alpha = alpha;

        let moves = self.board.moves();
        if moves.is_empty() {
            if self.board.in_check() {
                return MATED_VALUE + depth as i32;
            }
            return 0;
        }

        let mut line = vec![];

        for mv in moves {
            self.board.make_move(&mv);
            let neg_score = self.alpha_beta(-beta, -alpha, depth - 1, &mut line);
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
                return beta;
            }

            if score > alpha {
                node_kind = NodeKind::Exact;
                alpha = score;
                update_line(parent_line, mv, &line);
            }
        }

        let node = Node {
            depth,
            score: alpha,
            kind: node_kind,
        };
        let hash = self.board.zobrist();
        self.transposition_table.insert(hash, node);

        alpha
    }
}

fn update_line(line: &mut Vec<Move>, mv: Move, child_line: &Vec<Move>) {
    let mut new_line = Vec::with_capacity(child_line.len() + 1);
    new_line.push(mv);
    new_line.extend(child_line.clone());
    *line = new_line;
}
