use clap::{Args, Subcommand};

use chust_engine::{Engine, Move};

#[derive(Debug, Subcommand)]
#[command(rename_all = "lower")]
pub(crate) enum FenMovesCommand {
    Fen(FenArgs),
    StartPos(StartPosArgs),
    Moves {
        #[arg(required = true)]
        moves: Vec<String>,
    },
}

#[derive(Debug, Args)]
pub(crate) struct FenArgs {
    position: String,
    colour: String,
    castling: String,
    en_passant: String,
    half_clock: String,
    full_clock: String,
    #[command(subcommand)]
    moves: Option<MovesCommand>,
}

#[derive(Debug, Args)]
pub(crate) struct StartPosArgs {
    #[command(subcommand)]
    moves: Option<MovesCommand>,
}

#[derive(Debug, Subcommand)]
pub(crate) enum MovesCommand {
    Moves { moves: Vec<String> },
}

pub(crate) fn apply_fen_and_moves(cmd: FenMovesCommand, engine: &mut Engine) {
    match cmd {
        FenMovesCommand::Fen(cmd) => {
            let fen = format!(
                "{} {} {} {} {} {}",
                cmd.position,
                cmd.colour,
                cmd.castling,
                cmd.en_passant,
                cmd.half_clock,
                cmd.full_clock
            );
            engine.reset(Some(fen));

            if let Some(moves) = cmd.moves {
                let MovesCommand::Moves { moves } = moves;
                apply_moves(moves, engine);
            }
        }
        FenMovesCommand::StartPos(cmd) => {
            engine.reset(None);
            if let Some(moves) = cmd.moves {
                let MovesCommand::Moves { moves } = moves;
                apply_moves(moves, engine);
            }
        }
        FenMovesCommand::Moves { moves } => {
            apply_moves(moves, engine);
        }
    }
}

pub(crate) fn apply_moves(moves: Vec<String>, engine: &mut Engine) {
    for mv in &moves {
        engine.board.make_move(&Move::coordinate(mv));
    }
}
