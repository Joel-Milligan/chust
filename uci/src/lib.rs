use clap::{Parser, Subcommand};

use crate::go::GoArgs;
use crate::go::invoke_go;
use crate::position::PositionArgs;
use crate::position::invoke_position;
use chust_engine::Engine;

mod go;
mod position;

const NAME: &str = "Chust";
const AUTHOR: &str = "Joel Milligan";

#[derive(Debug, Parser)]
#[command(multicall = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
#[command(rename_all = "lower")]
enum Commands {
    Uci,
    IsReady,
    UciNewGame,
    Position(PositionArgs),
    Go(GoArgs),
    Quit,
}

pub fn respond(line: &str, engine: &mut Engine) -> Result<bool, String> {
    let args = line.split_whitespace();
    let cli = Cli::try_parse_from(args).map_err(|e| e.to_string())?;
    match cli.command {
        Commands::Uci => println!("id name {NAME}\nid author {AUTHOR}\nuciok"),
        Commands::UciNewGame => {
            engine.reset(None);
            println!("readyok");
        }
        Commands::IsReady => println!("readyok"),
        Commands::Position(command) => invoke_position(engine, command),
        Commands::Go(command) => invoke_go(engine, command),
        Commands::Quit => return Ok(true),
    }
    Ok(false)
}
