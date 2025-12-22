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
    pub nodes: usize,
    pub killer_moves: ([Option<Move>; 246], [Option<Move>; 246]),
    pub history_moves: [[usize; 64]; 12],
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
            transposition_table: HashMap::new(),
            nodes: 0,
            killer_moves: ([None; 246], [None; 246]),
            history_moves: [[0; 64]; 12],
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

    fn score_move(&self, mv: &Move) -> usize {
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

    pub fn search_depth(&mut self, depth: usize) -> (Vec<Move>, i32) {
        let mut max_eval = MATED_VALUE;
        let mut pv = vec![];
        let mut line = vec![];

        let mut sorted_moves = self.board.moves();
        sorted_moves.sort_by(|a, b| self.score_move(a).cmp(&self.score_move(b)));
        for mv in sorted_moves {
            self.board.make_move(&mv);
            let eval = -self.alpha_beta(MATED_VALUE, i32::MAX, depth, &mut line);
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
        let table_node = self.transposition_table.get(&hash);

        if let Some(node) = table_node
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

        self.nodes += 1;

        let mut node_kind = NodeKind::Alpha;

        if depth == 0 {
            let score = self.quiescence(alpha, beta);
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

        let mut sorted_moves = self.board.moves();
        sorted_moves.sort_by(|a, b| self.score_move(b).cmp(&self.score_move(a)));
        for mv in sorted_moves {
            self.board.make_move(&mv);
            let eval = -self.alpha_beta(-beta, -alpha, depth - 1, &mut line);

            if eval >= beta {
                let hash = self.board.zobrist();
                self.board.unmake_move();

                // Only update on quiet moves
                if self.board.squares[mv.destination.0 as usize].is_none() {
                    let ply = self.board.half_moves as usize;
                    self.killer_moves.1[ply] = self.killer_moves.0[ply];
                    self.killer_moves.0[ply] = Some(mv);
                }

                let node = Node {
                    depth,
                    score: beta,
                    kind: NodeKind::Beta,
                };
                self.transposition_table.insert(hash, node);
                return beta;
            }

            self.board.unmake_move();

            if eval > alpha {
                if self.board.squares[mv.destination.0 as usize].is_none() {
                    let (_, piece) = self.board.squares[mv.source.0 as usize]
                        .expect("all valid moves have a piece at source");
                    self.history_moves[piece as usize][mv.destination.0 as usize] += depth;
                }

                node_kind = NodeKind::Exact;
                alpha = eval;
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

    fn quiescence(&mut self, mut alpha: i32, beta: i32) -> i32 {
        self.nodes += 1;

        let eval = self.evaluate();

        if eval >= beta {
            return beta;
        }

        if eval > alpha {
            alpha = eval;
        }

        let moves = self.board.moves();
        // FIXME: Does not identify en passant captures
        let captures = moves
            .into_iter()
            .filter(|x| self.board.squares[x.destination.0 as usize].is_some())
            .collect::<Vec<_>>();

        for mv in captures {
            self.board.make_move(&mv);
            let eval = -self.quiescence(-beta, -alpha);
            self.board.unmake_move();

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

fn update_line(line: &mut Vec<Move>, mv: Move, child_line: &Vec<Move>) {
    let mut new_line = Vec::with_capacity(child_line.len() + 1);
    new_line.push(mv);
    new_line.extend(child_line.clone());
    *line = new_line;
}
