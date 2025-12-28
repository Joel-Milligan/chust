use std::collections::VecDeque;
use std::fmt::Write;
use std::io;

use crate::board::Board;
use crate::constants::MATE_VALUE;
use crate::engine::{Engine, MAX_PLY};
use crate::piece_move::Move;

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

    pub fn write_info(
        depth: usize,
        nodes: usize,
        score: i32,
        pv_length: usize,
        pv_table: &[Option<Move>; MAX_PLY],
    ) {
        let mut buffer = String::new();
        write!(buffer, "info depth {depth} nodes {nodes} score ").unwrap();

        let dist = MATE_VALUE - score.abs();

        if dist <= MAX_PLY as i32 {
            let dist = if score > 0 { dist } else { -dist };
            let mate = if dist > 0 { dist + 1 } else { dist } / 2;
            write!(buffer, "mate {mate} pv ").unwrap();
        } else {
            write!(buffer, "cp {score} pv ").unwrap();
        }

        for i in 0..pv_length {
            write!(buffer, "{} ", pv_table[i].unwrap()).unwrap();
        }

        println!("{buffer}");
    }
}
