#!/bin/bash
cargo run --release --bin uci -- go perft $1 "$2" "$3"
