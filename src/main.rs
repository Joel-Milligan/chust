use std::env;

use chust::perftree::perftree;
use chust::uci::Uci;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    // perftree
    if args.len() == 4 {
        let depth = args[1].parse().unwrap();
        let fen = &args[2];
        let moves = args[3].split_whitespace().collect();
        perftree(depth, fen, moves);
        return Ok(());
    }

    let mut uci = Uci::new();
    uci.start()?;
    Ok(())
}
