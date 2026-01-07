use chust_engine::Engine;
use chust_uci::respond;

fn main() -> Result<(), String> {
    let mut engine = Engine::new();

    loop {
        let mut buffer = String::new();
        std::io::stdin()
            .read_line(&mut buffer)
            .map_err(|e| e.to_string())?;
        let line = buffer.trim();
        if line.is_empty() {
            continue;
        }

        match respond(line, &mut engine) {
            Ok(quit) => {
                if quit {
                    break;
                }
            }
            Err(e) => eprintln!("{e}"),
        }
    }

    Ok(())
}
