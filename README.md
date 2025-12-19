# Chust

Chess in rust

## Running

`cargo run`: Run in UCI mode

`cargo bench`: Benches using uci perft at depth 3

`cargo test`: Uses perft to confirm correctness

### En Crossiant
1. Build in release mode
2. Add new engine in En Crossiant - Local: `target/release/chust`
3. Configure engine to depth of 3-5 (anything more takes too long)

## Tasks

### Tech Debt

- Improve seperation of concerns between UCI and engine
- Error handling and validation
- Clean up warning suppression
- Clean up clones

### Performance

- pv using transposition table
- Move ordering
- Pre-compute lookup tables for moves

### Strength

- Quiescence
- Research opening books
- Research endgame tablebases

### Features

- Enough UCI to play on Lichess
- Algebraic notation
- PGN
