use std::env;
use std::io;

use chust::board::Board;
use chust::engine::Engine;
use chust::piece_move::Move;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 4 {
        let depth = &args[1];
        let fen = &args[2];
        let moves = args[3].split_whitespace();

        let mut board = Board::from_fen(fen).unwrap();

        for mv in moves {
            board.make_move(&Move::coordinate(mv));
        }

        board.divide(depth.parse().unwrap());
        return;
    }

    let mut board = Board::default();
    println!("{board}\n");

    let mut input_buffer = String::new();
    loop {
        input_buffer.clear();
        let stdin = io::stdin();
        stdin.read_line(&mut input_buffer).unwrap();

        // White
        board.make_move(&Move::coordinate(input_buffer.trim()));
        println!("{board}\n");

        // Black
        let mut engine = Engine::new(&board);
        let engine_move = engine.generate_move();
        board.make_move(&engine_move);
        println!("{board}\n");
    }
}
