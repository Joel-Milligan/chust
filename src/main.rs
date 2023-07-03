mod board;
mod piece;

use crate::board::Board;

fn main() {
    let mut board = Board::default();
    board.move_coordinate("e2e4");
    println!("{board}");
}
