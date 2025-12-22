# Chust

Chess in rust

## Running

`cargo run`: Run in UCI mode

`cargo bench`: Benches using uci perft at depth 3

`cargo test`: Uses perft to confirm correctness

`cargo flamegraph --bin perf`: Produce flamegraph of perf binary for helping with optimisation

### En Crossiant

1. Build in release mode
2. Add new engine in En Crossiant - Local: `target/release/chust`
3. Configure engine to depth of 3-5 (anything more takes too long)

## Tasks

1. Move ordering
2. Quiensence search

### Tech Debt

- Improve seperation of concerns between UCI and engine
    - Some has been completed, but still too tied together
- Error handling and validation
- Clean up clones

### Performance

- Move ordering
    - Iterative search should end early if mate is found
- pv using transposition table
- Pre-compute lookup tables for moves

### Strength

- Fails to find mate in 3 for "r1bqk2r/2ppb1p1/n3P2p/8/2B1nP2/4P3/1PPP3P/RNBQK1NR w KQkq - 0 10"
- Quiescence
- Research opening books
- Research endgame tablebases

### UCI

- Enough UCI to play on Lichess

### Features

- Algebraic notation
- PGN
