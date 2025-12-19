use std::fmt::Display;
use std::str::FromStr;

use crate::constants::{BISHOP, KNIGHT, QUEEN, ROOK};
use crate::square::Square;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Move {
    pub source: Square,
    pub destination: Square,
    pub promotion: Option<u8>,
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(promotion) = self.promotion {
            let piece = match promotion {
                KNIGHT => 'n',
                BISHOP => 'b',
                ROOK => 'r',
                QUEEN => 'q',
                _ => panic!("unknown promotion piece"),
            };
            write!(f, "{}{}{}", self.source, self.destination, piece)
        } else {
            write!(f, "{}{}", self.source, self.destination)
        }
    }
}

impl Move {
    pub fn new(source: u8, destination: u8) -> Move {
        Move {
            source: Square(source),
            destination: Square(destination),
            promotion: None,
        }
    }

    pub fn promotion(source: u8, destination: u8, piece: u8) -> Move {
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
