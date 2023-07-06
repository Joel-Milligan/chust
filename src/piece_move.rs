use std::fmt::Display;

#[derive(Debug)]
pub struct Move(pub usize, pub usize);

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let src_file = match self.0 % 8 {
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

        let src_rank = self.0 / 8 + 1;

        let dst_file = match self.1 % 8 {
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

        let dst_rank = self.1 / 8 + 1;

        write!(f, "{src_file}{src_rank}{dst_file}{dst_rank}")
    }
}
