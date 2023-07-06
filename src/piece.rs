use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Colour {
    White,
    Black,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

pub const WHITE_ROOK: Option<Piece> = Some(Piece {
    colour: Colour::White,
    kind: PieceType::Rook,
});

pub const WHITE_KNIGHT: Option<Piece> = Some(Piece {
    colour: Colour::White,
    kind: PieceType::Knight,
});

pub const WHITE_BISHOP: Option<Piece> = Some(Piece {
    colour: Colour::White,
    kind: PieceType::Bishop,
});

pub const WHITE_QUEEN: Option<Piece> = Some(Piece {
    colour: Colour::White,
    kind: PieceType::Queen,
});

pub const WHITE_KING: Option<Piece> = Some(Piece {
    colour: Colour::White,
    kind: PieceType::King,
});

pub const WHITE_PAWN: Option<Piece> = Some(Piece {
    colour: Colour::White,
    kind: PieceType::Pawn,
});

pub const BLACK_ROOK: Option<Piece> = Some(Piece {
    colour: Colour::Black,
    kind: PieceType::Rook,
});

pub const BLACK_KNIGHT: Option<Piece> = Some(Piece {
    colour: Colour::Black,
    kind: PieceType::Knight,
});

pub const BLACK_BISHOP: Option<Piece> = Some(Piece {
    colour: Colour::Black,
    kind: PieceType::Bishop,
});

pub const BLACK_QUEEN: Option<Piece> = Some(Piece {
    colour: Colour::Black,
    kind: PieceType::Queen,
});

pub const BLACK_KING: Option<Piece> = Some(Piece {
    colour: Colour::Black,
    kind: PieceType::King,
});

pub const BLACK_PAWN: Option<Piece> = Some(Piece {
    colour: Colour::Black,
    kind: PieceType::Pawn,
});
