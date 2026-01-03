use std::io;

use chust_engine::{Board, Engine, Move};

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

    pub fn start(&mut self, args: Vec<String>) -> Result<(), std::io::Error> {
        if args.len() > 1 {
            let args = args[1..].iter().map(AsRef::as_ref).collect::<Vec<_>>();
            return self.command(args.as_slice());
        }

        let mut command = String::new();

        loop {
            command.clear();
            io::stdin().read_line(&mut command)?;
            let tokens = command
                .split_whitespace()
                .map(AsRef::as_ref)
                .collect::<Vec<_>>();
            self.command(tokens.as_slice())?;
        }
    }

    fn command(&mut self, tokens: &[&str]) -> Result<(), std::io::Error> {
        if let Some((&operator, rest)) = tokens.split_first() {
            match operator {
                "uci" => println!("id name {NAME}\nid author {AUTHOR}\nuciok"),
                "isready" => println!("readyok"),
                "ucinewgame" => {
                    self.engine.board = Board::default();
                    println!("readyok")
                }
                "position" => position::invoke(&mut self.engine, rest),
                "go" => go::invoke(&mut self.engine, rest),
                "quit" => std::process::exit(0),
                _ => {}
            }
        }
        Ok(())
    }
}
