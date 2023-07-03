use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Colour {
    White,
    Black,
}

#[derive(Clone, Copy, Debug)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub colour: Colour,
    pub kind: PieceType,
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let piece = match self.kind {
            PieceType::King => "k",
            PieceType::Queen => "q",
            PieceType::Rook => "r",
            PieceType::Bishop => "b",
            PieceType::Knight => "n",
            PieceType::Pawn => "p",
        };

        if self.colour == Colour::White {
            write!(f, "{}", piece.to_uppercase())
        } else {
            write!(f, "{}", piece)
        }
    }
}
