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
                    let tokens: Vec<String> = tokens.collect();
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
                "go" => {
                    if let Some(tk) = tokens.next() {
                        match tk.as_str() {
                            "depth" => {
                                if let Some(depth) = tokens.next() {
                                    if let Ok(depth) = depth.parse::<usize>() {
                                        let (mv, eval) = self.engine.start_search(depth);

                                        if eval > 200.0 {
                                            println!("info depth {depth} score mate 1 pv {mv}");
                                        } else {
                                            println!("info depth {depth} score cp {eval} pv {mv}");
                                        }

                                        println!("bestmove {mv}");
                                    }
                                }
                            }
                            "wtime" => {
                                let (mv, eval) = self.engine.start_search(3);
                                if eval > 200.0 {
                                    println!("info depth 3 score mate 1 pv {mv}");
                                } else {
                                    println!("info depth 3 score cp {eval} pv {mv}");
                                }
                                println!("bestmove {mv}");
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
