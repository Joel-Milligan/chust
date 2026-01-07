use clap::{Args, Subcommand};

use chust_engine::{Engine, Move};

#[derive(Args, Debug)]
#[command(args_conflicts_with_subcommands = true)]
pub struct PositionArgs {
    #[command(subcommand)]
    command: PositionCommands,
}

#[derive(Debug, Subcommand)]
#[command(rename_all = "lower")]
enum PositionCommands {
    StartPos {
        #[command(subcommand)]
        moves: Option<MovesCommand>,
    },
    Fen {
        fen: Vec<String>,
    },
}

#[derive(Debug, Subcommand)]
enum MovesCommand {
    Moves {
        #[arg(required = true)]
        moves: Vec<String>,
    },
}

pub fn invoke_position(engine: &mut Engine, command: PositionArgs) {
    match command.command {
        PositionCommands::StartPos { moves } => {
            engine.reset(None);
            if let Some(moves) = moves {
                let MovesCommand::Moves { moves } = moves;
                for mv in &moves {
                    engine.board.make_move(&Move::coordinate(mv));
                }
            }
        }
        PositionCommands::Fen { fen } => {
            let fen = fen.join(" ");
            engine.reset(Some(fen));
        }
    };
}
