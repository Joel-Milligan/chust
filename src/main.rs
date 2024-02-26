use chust::board::Board;

fn main() {
    let board = Board::from_fen("rnbqkbnr/lppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    match board {
        Err(..) => println!("Bad FEN provided, cannot create board."),
        Ok(_) => (),
    };
}
