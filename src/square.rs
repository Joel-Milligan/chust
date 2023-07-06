use std::fmt::Display;

#[derive(Debug)]
pub struct Square(pub usize);

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file = match self.0 % 8 {
            0 => "a",
            1 => "b",
            2 => "c",
            3 => "d",
            4 => "e",
            5 => "f",
            6 => "g",
            7 => "h",
            _ => unreachable!(),
        };

        let rank = self.0 / 8 + 1;

        write!(f, "{file}{rank}")
    }
}
