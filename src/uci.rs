use std::io;

use crate::board::Board;

use crate::piece_move::Move;

const NAME: &str = "Chust";
const AUTHOR: &str = "Joel Milligan";

pub struct Uci {
    pub board: Board
}

impl Uci {
    pub fn new() -> Uci {
        Uci { board: Board::default() }
    }

    pub fn start(&mut self, args: Vec<String>) -> Result<(), std::io::Error> {
        if args.len() > 1 {
            return Ok(self.command(args)?)
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
        let mut tokens = tokens.into_iter().skip(1);
        if let Some(operator) = tokens.next() {
            match operator.as_str() {
                "uci" => println!("id name {NAME}\nid author {AUTHOR}\nuciok"),
                "isready" => println!("readyok"),
                "ucinewgame" => {
                    self.board = Board::default();
                    println!("readyok")
                },
                "position" => {
                    if let Some(pos) = tokens.next() {
                        if pos == "startpos" {
                            self.board = Board::default();
                        } else {
                            if let Some(game) = Board::from_fen(&pos).ok() {
                                self.board = game;
                            } else {
                                return Ok(());
                            }
                        }

                        // Rest of tokens are moves
                        for mv in tokens {
                            self.board.make_move(&Move::coordinate(&mv))
                        }
                    }
                },
                "go" => {
                    if let Some(tk) = tokens.next() {
                        match tk.as_str() {
                            "perft" => {
                                if let Some(depth) = tokens.next() {
                                    if let Some(fen) = tokens.next() {
                                        self.board = Board::from_fen(&fen).unwrap();
                                    }

                                    // Rest of tokens are moves
                                    if let Some(moves) = tokens.next() {
                                        for mv in moves.split_whitespace() {
                                            self.board.make_move(&Move::coordinate(&mv))
                                        }
                                    }

                                    if let Some(depth) = depth.parse::<usize>().ok() {
                                        self.board.divide(depth);
                                    }
                                }
                            },
                            _ => {}
                        }
                    }
                },
                "quit" => return Ok(()),
                _ => {}
            }
        }
        Ok(())
    }
}
