use std::collections::HashMap;

use crate::board::Board;
use crate::constants::*;
use crate::piece_move::Move;
use crate::uci::Uci;

pub const MAX_PLY: usize = 64;

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
            transposition_table: HashMap::new(),
            nodes: 0,
            ply: 0,
            killer_moves: ([None; MAX_PLY], [None; MAX_PLY]),
            history_moves: [[0; 64]; 12],
            pv_length: [0; MAX_PLY],
            pv_table: [[None; MAX_PLY]; MAX_PLY],
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

        let mut score = QUEEN_VALUE * queens as i32
            + ROOK_VALUE * rooks as i32
            + BISHOP_VALUE * bishops as i32
            + KNIGHT_VALUE * knights as i32
            + PAWN_VALUE * pawns as i32;

        for (square, piece) in self.board.squares.iter().enumerate() {
            if let &Some((colour, piece)) = piece {
                if colour == WHITE {
                    match piece {
                        PAWN => score += PAWN_SCORE[square],
                        KNIGHT => score += KNIGHT_SCORE[square],
                        BISHOP => score += BISHOP_SCORE[square],
                        ROOK => score += ROOK_SCORE[square],
                        KING => score += KING_SCORE[square],
                        _ => {}
                    };
                } else {
                    match piece {
                        PAWN => score -= PAWN_SCORE[63 - square],
                        KNIGHT => score -= KNIGHT_SCORE[63 - square],
                        BISHOP => score -= BISHOP_SCORE[63 - square],
                        ROOK => score -= ROOK_SCORE[63 - square],
                        KING => score -= KING_SCORE[63 - square],
                        _ => {}
                    };
                }
            }
        }

        score
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

    pub fn search_depth(&mut self, depth: usize) {
        self.nodes = 0;
        self.killer_moves = ([None; MAX_PLY], [None; MAX_PLY]);
        self.history_moves = [[0; 64]; 12];
        self.pv_length = [0; MAX_PLY];
        self.pv_table = [[None; MAX_PLY]; MAX_PLY];

        for current_depth in 1..=depth {
            self.transposition_table.clear();

            let eval = self.alpha_beta(MATED_VALUE, i32::MAX, current_depth);
            Uci::write_info(
                current_depth,
                self.nodes,
                eval,
                self.pv_length[0],
                &self.pv_table[0],
            );

            // for row in self.pv_table {
            //     for col in row {
            //         if let Some(mv) = col {
            //             print!("{mv} ");
            //         } else {
            //             print!("---- ");
            //         }
            //     }
            //     println!();
            // }
            // println!();
        }

        if let Some(best_move) = self.pv_table[0][0] {
            println!("bestmove {}", best_move);
        }
    }

    fn alpha_beta(&mut self, mut alpha: i32, beta: i32, depth: usize) -> i32 {
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

        self.pv_length[self.ply] = self.ply;

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

        self.nodes += 1;

        let moves = self.board.moves();
        if moves.is_empty() {
            if self.board.in_check() {
                return MATED_VALUE + depth as i32;
            }
            return 0;
        }

        let mut sorted_moves = self.board.moves();
        sorted_moves.sort_by(|a, b| self.score_move(b).cmp(&self.score_move(a)));
        for mv in sorted_moves {
            self.board.make_move(&mv);
            self.ply += 1;
            let eval = -self.alpha_beta(-beta, -alpha, depth - 1);
            self.board.unmake_move();
            self.ply -= 1;

            if eval > alpha {
                if self.board.squares[mv.destination.0 as usize].is_none() {
                    let (_, piece) = self.board.squares[mv.source.0 as usize]
                        .expect("all valid moves have a piece at source");
                    self.history_moves[piece as usize][mv.destination.0 as usize] += depth as i32;
                }

                node_kind = NodeKind::Exact;
                alpha = eval;

                self.pv_table[self.ply][self.ply] = Some(mv);
                for next_ply in (self.ply + 1)..self.pv_length[self.ply + 1] {
                    self.pv_table[self.ply][next_ply] = self.pv_table[self.ply + 1][next_ply];
                }
                self.pv_length[self.ply] = self.pv_length[self.ply + 1];

                if eval >= beta {
                    let hash = self.board.zobrist();
                    let node = Node {
                        depth,
                        score: beta,
                        kind: NodeKind::Beta,
                    };
                    self.transposition_table.insert(hash, node);

                    if self.board.squares[mv.destination.0 as usize].is_none() {
                        self.killer_moves.1[self.ply] = self.killer_moves.0[self.ply];
                        self.killer_moves.0[self.ply] = Some(mv);
                    }

                    return beta;
                }
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

        let moves = self.board.moves();
        if moves.is_empty() {
            if self.board.in_check() {
                return MATED_VALUE as i32;
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
        let captures = moves
            .into_iter()
            .filter(|x| self.board.squares[x.destination.0 as usize].is_some())
            .collect::<Vec<_>>();

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
