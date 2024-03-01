use std::env;

use chust::uci::Uci;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let mut uci = Uci::new();
    uci.start(args)
}
