use std::fmt::Display;

use crate::square::Square;

#[derive(Debug)]
pub struct Move(pub Square, pub Square);

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

impl Move {
    pub fn coordinate(input: String) -> Move {
        // TODO: Validate
        let mut chars = input.chars();

        let source_file = match chars.next().unwrap() {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => panic!(),
        };

        let source_rank = match chars.next().unwrap() {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            _ => panic!(),
        };

        let destination_file = match chars.next().unwrap() {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => panic!(),
        };

        let destination_rank = match chars.next().unwrap() {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            _ => panic!(),
        };

        Move(
            Square(source_file + source_rank * 8),
            Square(destination_file + destination_rank * 8),
        )
    }

    pub fn source(&self) -> &Square {
        &self.0
    }

    pub fn destination(&self) -> &Square {
        &self.1
    }
}
