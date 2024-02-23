use std::str::FromStr;

use chust::board::Board;
use chust::square::Square;

fn main() {
    let board = Board::default();
    println!("{board}");

    let square = Square::from_str("b1").unwrap();
    let moves = board.get_legal_moves(square);

    println!("\nMOVES: b1");
    for mv in moves {
        println!("{}", mv.destination());
    }
}
