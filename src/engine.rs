use crate::board::Board;
use crate::constants::*;
use crate::piece_move::Move;

pub struct Engine<'a> {
    board: &'a Board
}

impl Engine<'_> {
    pub fn new(board: &Board) -> Engine {
        Engine { board }
    }

    pub fn generate_move(&mut self) -> Move {
        let mut test_board = self.board.clone();

        test_board
            .moves()
            .into_iter()
            .map(|mv| {
                test_board.make_move(&mv);
                let eval = self.evaluate(&test_board);
                test_board.unmake_move();
                (eval, mv)
            })
            .max_by(|x, y| x.0.cmp(&y.0))
            .unwrap()
            .1
    }

    fn evaluate(&self, position: &Board) -> i32 {
        position.squares.into_iter()
            .filter_map(|s| s)
            .map(|(colour, piece)| {
                let value = match piece {
                    PAWN => 1,
                    KNIGHT => 3,
                    BISHOP => 3,
                    ROOK => 5,
                    QUEEN => 9,
                    _ => 0,
                };

                if colour == self.board.active_colour {
                    value
                } else {
                    -value
                }
            })
            .fold(0, |evaluation, value| evaluation + value)
    }
}
