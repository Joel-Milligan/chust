use std::io;

pub struct Uci;

impl Uci {
    pub fn new() -> Uci {
        Uci
    }

    pub fn start(&mut self) -> Result<(), std::io::Error> {
        let mut command = String::new();

        loop {
            command.clear();
            io::stdin().read_line(&mut command)?;

            match command.split_whitespace().next() {
                Some("quit") => std::process::exit(0),
                _ => {}
            }
        }
    }
}
