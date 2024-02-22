use std::fmt::Display;
use std::str::FromStr;

use crate::square::Square;

#[derive(Debug, PartialEq, Eq)]
pub struct Move(pub Square, pub Square);

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

impl Move {
    pub fn coordinate(input: String) -> Move {
        // TODO: Validate
        let source = Square::from_str(&input[..2]).unwrap();
        let destination = Square::from_str(&input[2..]).unwrap();
        Move(source, destination)
    }

    pub fn source(&self) -> &Square {
        &self.0
    }

    pub fn destination(&self) -> &Square {
        &self.1
    }
}
