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
        (A1..=H8)
            .into_iter()
            .map(|square| {
                if let Some((colour, piece)) = position.get_piece_at_square(square) {
                    let value = match piece {
                        PAWN => 1,
                        KNIGHT | BISHOP => 3,
                        ROOK => 5,
                        QUEEN => 9,
                        _ => 0,
                    };

                    if colour == self.board.active_colour {
                        value
                    } else {
                        -value
                    }
                } else {
                    0
                }
            })
            .sum()
    }
}
