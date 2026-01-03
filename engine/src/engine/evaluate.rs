use crate::repr::constants::*;
use crate::calculated::values::*;
use crate::engine::Engine;

impl Engine {
    pub fn evaluate(&self) -> i32 {
        let friend = self.board.pieces[self.board.active_colour as usize];
        let enemy = self.board.pieces[1 - self.board.active_colour as usize];

        let queens =
            friend[QUEEN as usize].count_ones() as i8 - enemy[QUEEN as usize].count_ones() as i8;
        let rooks =
            friend[ROOK as usize].count_ones() as i8 - enemy[ROOK as usize].count_ones() as i8;
        let bishops =
            friend[BISHOP as usize].count_ones() as i8 - enemy[BISHOP as usize].count_ones() as i8;
        let knights =
            friend[KNIGHT as usize].count_ones() as i8 - enemy[KNIGHT as usize].count_ones() as i8;
        let pawns =
            friend[PAWN as usize].count_ones() as i8 - enemy[PAWN as usize].count_ones() as i8;

        let mut score = QUEEN_VALUE * queens as i32
            + ROOK_VALUE * rooks as i32
            + BISHOP_VALUE * bishops as i32
            + KNIGHT_VALUE * knights as i32
            + PAWN_VALUE * pawns as i32;

        for (square, piece) in self.board.squares.iter().enumerate() {
            if let &Some((colour, piece)) = piece {
                if colour == WHITE {
                    match piece {
                        PAWN => score += PAWN_SCORE[square],
                        KNIGHT => score += KNIGHT_SCORE[square],
                        BISHOP => score += BISHOP_SCORE[square],
                        ROOK => score += ROOK_SCORE[square],
                        KING => score += KING_SCORE[square],
                        _ => {}
                    };
                } else {
                    match piece {
                        PAWN => score -= PAWN_SCORE[63 - square],
                        KNIGHT => score -= KNIGHT_SCORE[63 - square],
                        BISHOP => score -= BISHOP_SCORE[63 - square],
                        ROOK => score -= ROOK_SCORE[63 - square],
                        KING => score -= KING_SCORE[63 - square],
                        _ => {}
                    };
                }
            }
        }

        score
    }
}
