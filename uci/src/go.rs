use clap::Subcommand;

use chust_engine::Engine;

use crate::fen_move::*;

#[derive(Debug, Subcommand)]
#[command(rename_all = "lower")]
pub enum GoCommands {
    Depth {
        depth: usize,
    },
    Evaluate,
    Perft {
        depth: usize,
        #[command(subcommand)]
        fen: Option<FenMovesCommand>,
    },
}

pub fn invoke_go(engine: &mut Engine, go_cmd: GoCommands) {
    match go_cmd {
        GoCommands::Depth { depth } => engine.search_depth(depth),
        GoCommands::Evaluate => println!("{}", engine.evaluate()),
        GoCommands::Perft { depth, fen } => {
            if let Some(fen) = fen {
                apply_fen_and_moves(fen, engine);
            }

            engine.board.divide(depth);
        }
    }
}
