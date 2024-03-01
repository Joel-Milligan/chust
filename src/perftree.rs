use crate::board::Board;
use crate::piece_move::Move;

pub fn perftree(depth: usize, fen: &str, moves: Vec<&str>) {
    let mut board = Board::from_fen(fen).unwrap();

    for mv in moves {
        board.make_move(&Move::coordinate(mv));
    }

    board.divide(depth);
}
