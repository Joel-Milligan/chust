use std::env;

use chust::board::Board;
use chust::piece_move::Move;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        let mut board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
        board.make_move(Move::coordinate("a5a4"));
        board.make_move(Move::coordinate("c7c5"));
        let moves = board.moves();
        for mv in moves {
            println!("{}", mv);
        }
    } else {
        let depth = &args[1];
        let fen = &args[2];
        let moves = args[3].split_whitespace();

        let mut board = Board::from_fen(fen).unwrap();

        for mv in moves {
            board.make_move(Move::coordinate(mv));
        }

        board.divide(depth.parse().unwrap());
    }
}
