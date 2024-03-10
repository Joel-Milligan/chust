use std::collections::VecDeque;
use std::io;

use crate::board::Board;
use crate::engine::Engine;

mod go;
mod position;

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
                "position" => position::invoke(&mut self.engine, tokens),
                "go" => go::invoke(&mut self.engine, tokens),
                "quit" => std::process::exit(0),
                _ => {}
            }
        }
        Ok(())
    }
}
