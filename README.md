# Chust

Chess in rust

## Running

`cargo run -r`: Searches a specific position to a specific depth, defined in the `perf` binary

`cargo run -r --bin uci`: Run in UCI mode

`cargo bench`: Benches using uci perft at depth 3

`cargo test`: Uses perft to confirm correctness

`cargo flamegraph`: Produce flamegraph of perf binary for helping with optimisation

### En Crossiant

1. Build in release mode
2. Add new engine in En Crossiant - Local: `target/release/chust`
3. Configure engine to depth of 3-5 (anything more takes too long)

## Tasks

### Tech Debt

- Improve seperation of concerns between UCI and engine
    - Some has been completed, but still too tied together
- Error handling and validation

### Performance

- Pre-compute lookup tables for moves

### Strength

- Research opening books
- Research endgame tablebases

### UCI

- Enough UCI to play on Lichess

### Features

- Algebraic notation
- PGN
- TUI