use std::fmt::Display;

use crate::square::Square;

#[derive(Debug)]
pub struct Move(pub Square, pub Square);

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}
