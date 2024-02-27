use std::fmt::Display;
use std::str::FromStr;

use crate::constants::{BISHOP, KNIGHT, QUEEN, ROOK};
use crate::square::Square;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Move {
    pub source: Square,
    pub destination: Square,
    pub promotion: Option<usize>,
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.promotion.is_some() {
            let piece = match self.promotion.unwrap() {
                KNIGHT => 'n',
                BISHOP => 'b',
                ROOK => 'r',
                QUEEN => 'q',
                _ => panic!("unknown promotition piece"),
            };
            write!(f, "{}{}{}", self.source, self.destination, piece)
        } else {
            write!(f, "{}{}", self.source, self.destination)
        }
    }
}

impl Move {
    pub fn new(source: usize, destination: usize) -> Move {
        Move {
            source: Square(source),
            destination: Square(destination),
            promotion: None,
        }
    }

    pub fn promotion(source: usize, destination: usize, piece: usize) -> Move {
        let mut mv = Move::new(source, destination);
        mv.promotion = Some(piece);
        mv
    }

    pub fn coordinate(input: &str) -> Move {
        if input.len() == 4 {
            Move {
                source: Square::from_str(&input[..2]).unwrap(),
                destination: Square::from_str(&input[2..4]).unwrap(),
                promotion: None,
            }
        } else if input.len() == 5 {
            let piece = match input.chars().nth(4).unwrap() {
                'b' => BISHOP,
                'n' => KNIGHT,
                'q' => QUEEN,
                'r' => ROOK,
                _ => panic!(),
            };

            Move {
                source: Square::from_str(&input[..2]).unwrap(),
                destination: Square::from_str(&input[2..4]).unwrap(),
                promotion: Some(piece),
            }
        } else {
            todo!("validation")
        }
    }
}
