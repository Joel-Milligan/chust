use chust_engine::{Board, Engine, Move};
use clap::{Args, Subcommand};

#[derive(Args, Debug)]
#[command(args_conflicts_with_subcommands = true)]
pub struct GoArgs {
    #[command(subcommand)]
    command: GoCommands,
}

#[derive(Debug, Subcommand)]
enum GoCommands {
    Depth {
        depth: usize,
    },
    Evaluate,
    Perft {
        depth: usize,
        #[command(subcommand)]
        fen_moves: Option<FenMovesCommand>,
    },
}

#[derive(Debug, Subcommand)]
enum FenMovesCommand {
    Fen {
        #[arg(num_args = 6, required = true)]
        fen: Vec<String>,
    },
    Moves {
        #[arg(required = true)]
        moves: Vec<String>,
    },
}

pub fn invoke_go(engine: &mut Engine, command: GoArgs) {
    match command.command {
        GoCommands::Depth { depth } => engine.search_depth(depth),
        GoCommands::Evaluate => println!("{}", engine.evaluate()),
        GoCommands::Perft { depth, fen_moves } => {
            if let Some(cmd) = fen_moves {
                match cmd {
                    FenMovesCommand::Fen { fen } => {
                        let fen = fen.join(" ");
                        engine.board = Board::from_fen(&fen).unwrap();
                    }
                    FenMovesCommand::Moves { moves } => {
                        for mv in &moves {
                            engine.board.make_move(&Move::coordinate(mv));
                        }
                    }
                }
            }

            engine.board.divide(depth);
        }
    }
}
