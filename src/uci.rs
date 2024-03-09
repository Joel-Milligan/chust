use std::collections::VecDeque;
use std::io;

use crate::board::Board;
use crate::constants::MATED_VALUE;
use crate::engine::Engine;
use crate::piece_move::Move;

const NAME: &str = "Chust";
const AUTHOR: &str = "Joel Milligan";

pub struct Uci {
    pub engine: Engine,
}

impl Default for Uci {
    fn default() -> Self {
        Self::new()
    }
}

impl Uci {
    pub fn new() -> Uci {
        Uci {
            engine: Engine::new(),
        }
    }

    pub fn start(&mut self, mut args: VecDeque<String>) -> Result<(), std::io::Error> {
        if args.len() > 1 {
            args.pop_front();
            return self.command(args);
        }

        let mut command = String::new();

        loop {
            command.clear();
            io::stdin().read_line(&mut command)?;
            let tokens = command.split_whitespace().map(str::to_string).collect();
            self.command(tokens)?;
        }
    }

    fn command(&mut self, mut tokens: VecDeque<String>) -> Result<(), std::io::Error> {
        if let Some(operator) = tokens.pop_front() {
            match operator.as_str() {
                "uci" => println!("id name {NAME}\nid author {AUTHOR}\nuciok"),
                "isready" => println!("readyok"),
                "ucinewgame" => {
                    self.engine.board = Board::default();
                    println!("readyok")
                }
                "position" => self.position(tokens),
                "go" => self.go(tokens),
                "quit" => std::process::exit(0),
                _ => {}
            }
        }
        Ok(())
    }

    fn go(&mut self, mut tokens: VecDeque<String>) {
        if let Some(tk) = tokens.pop_front() {
            match tk.as_str() {
                "depth" => self.depth(tokens),
                "wtime" => {
                    let (mv, _) = self.engine.start_search(3);
                    println!("info depth 3 pv {mv}");
                    println!("bestmove {mv}");
                }
                "evaluate" => {
                    println!("{}", self.engine.evaluate(&self.engine.board))
                }
                "perft" => self.perft(tokens),
                _ => {}
            }
        }
    }

    fn depth(&mut self, mut tokens: VecDeque<String>) {
        if let Some(depth) = tokens.pop_front() {
            if let Ok(depth) = depth.parse::<usize>() {
                let (mv, eval) = self.engine.start_search(depth);

                let mate = MATED_VALUE.abs() - eval.abs();
                if mate <= 100 {
                    let mate = (depth as i64 - mate + 1).div_ceil(2);
                    if eval > 0 {
                        println!("info depth {depth} score mate {mate} pv {mv}");
                    } else {
                        println!("info depth {depth} score mate -{mate} pv {mv}");
                    }
                } else {
                    println!("info depth {depth} score cp {eval} pv {mv}");
                }

                println!("bestmove {mv}");
            }
        }
    }

    fn perft(&mut self, mut tokens: VecDeque<String>) {
        if let Some(depth) = tokens.pop_front() {
            if let Some(fen) = tokens.pop_front() {
                self.engine.board = Board::from_fen(&fen).unwrap();
            }

            if let Some(moves) = tokens.pop_front() {
                for mv in moves.split_whitespace() {
                    self.engine.board.make_move(&Move::coordinate(mv))
                }
            }

            if let Ok(depth) = depth.parse::<usize>() {
                self.engine.board.divide(depth);
            }
        }
    }

    fn position(&mut self, tokens: VecDeque<String>) {
        let tokens: Vec<String> = tokens.into();
        if tokens.len() > 0 && tokens[0] == "startpos" {
            self.engine.board = Board::default();

            if tokens.get(1) == Some(&"moves".to_string()) {
                let moves = tokens[2..].into_iter();
                for mv in moves {
                    self.engine.board.make_move(&Move::coordinate(&mv));
                }
            }
        } else if tokens.len() >= 7 && tokens[0] == "fen" {
            let fen = tokens[1..7].join(" ");
            self.engine.board = Board::from_fen(&fen).unwrap();

            if tokens.get(7) == Some(&"moves".to_string()) {
                let moves = tokens[8..].into_iter();
                for mv in moves {
                    self.engine.board.make_move(&Move::coordinate(&mv));
                }
            }
        }
    }
}
