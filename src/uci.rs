use std::io;

use crate::board::Board;

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

    pub fn start(&mut self, args: Vec<String>) -> Result<(), std::io::Error> {
        if args.len() > 1 {
            return self.command(args[1..].to_vec());
        }

        let mut command = String::new();

        loop {
            command.clear();
            io::stdin().read_line(&mut command)?;
            let tokens = command.split_whitespace().map(str::to_string).collect();
            self.command(tokens)?;
        }
    }

    fn command(&mut self, tokens: Vec<String>) -> Result<(), std::io::Error> {
        let mut tokens = tokens.into_iter();
        if let Some(operator) = tokens.next() {
            match operator.as_str() {
                "uci" => println!("id name {NAME}\nid author {AUTHOR}\nuciok"),
                "isready" => println!("readyok"),
                "ucinewgame" => {
                    self.engine.board = Board::default();
                    println!("readyok")
                }
                "position" => {
                    if let Some(pos) = tokens.next() {
                        if pos == "startpos" {
                            self.engine.board = Board::default();
                        } else if pos == "fen" {
                            let fen = tokens
                                .clone()
                                .take_while(|tk| tk != "moves")
                                .collect::<Vec<_>>()
                                .join(" ");
                            self.engine.board = Board::from_fen(&fen).unwrap();
                        } else {
                            return Ok(());
                        }

                        if tokens.next() == Some("moves".to_string()) {
                            for mv in tokens {
                                self.engine.board.make_move(&Move::coordinate(&mv));
                            }
                        }
                    }
                }
                "go" => {
                    if let Some(tk) = tokens.next() {
                        match tk.as_str() {
                            "depth" => {
                                if let Some(depth) = tokens.next() {
                                    if let Ok(depth) = depth.parse::<usize>() {
                                        let (mv, eval) = self.engine.start_search(depth);
                                        println!("info depth {depth} score cp {eval} pv {mv}");
                                        println!("bestmove {mv}");
                                    }
                                }
                            }
                            "evaluate" => {
                                println!("{}", self.engine.evaluate(&self.engine.board))
                            }
                            "perft" => {
                                if let Some(depth) = tokens.next() {
                                    if let Some(fen) = tokens.next() {
                                        self.engine.board = Board::from_fen(&fen).unwrap();
                                    }

                                    // Rest of tokens are moves
                                    if let Some(moves) = tokens.next() {
                                        for mv in moves.split_whitespace() {
                                            self.engine.board.make_move(&Move::coordinate(mv))
                                        }
                                    }

                                    if let Ok(depth) = depth.parse::<usize>() {
                                        self.engine.board.divide(depth);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                "quit" => std::process::exit(0),
                _ => {}
            }
        }
        Ok(())
    }
}
