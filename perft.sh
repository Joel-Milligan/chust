#!/bin/bash
cargo run --release -- go perft $1 "$2" "$3"
