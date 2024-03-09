use std::collections::VecDeque;
use std::env;

use chust::uci::Uci;

fn main() -> Result<(), std::io::Error> {
    let args: VecDeque<String> = env::args().collect();
    let mut uci = Uci::new();
    uci.start(args)
}
