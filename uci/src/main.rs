use std::env;

use chust_uci::Uci;

fn main() -> Result<(), std::io::Error> {
    let args = env::args().collect();
    let mut uci = Uci::new();
    uci.start(args)
}
