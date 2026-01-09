use clap::{Parser, Subcommand};

use chust_engine::Engine;

use crate::fen_move::{FenMovesCommand, apply_fen_and_moves};
use crate::go::{GoCommands, invoke_go};

mod fen_move;
mod go;

const NAME: &str = "Chust";
const AUTHOR: &str = "Joel Milligan";

#[derive(Debug, Parser)]
#[command(multicall = true)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Debug, Subcommand)]
#[command(rename_all = "lower")]
enum Commands {
    Uci,
    IsReady,
    UciNewGame,
    Position {
        #[command(subcommand)]
        cmd: FenMovesCommand,
    },
    Go {
        #[command(subcommand)]
        cmd: GoCommands,
    },
    Quit,
}

pub fn respond(line: &str, engine: &mut Engine) -> Result<bool, String> {
    let args = line.split_whitespace();
    let cli = Cli::try_parse_from(args).map_err(|e| e.to_string())?;
    match cli.cmd {
        Commands::Uci => println!("id name {NAME}\nid author {AUTHOR}\nuciok"),
        Commands::UciNewGame => {
            engine.reset(None);
            println!("readyok");
        }
        Commands::IsReady => println!("readyok"),
        Commands::Position { cmd } => apply_fen_and_moves(cmd, engine),
        Commands::Go { cmd } => invoke_go(engine, cmd),
        Commands::Quit => return Ok(true),
    }
    Ok(false)
}
