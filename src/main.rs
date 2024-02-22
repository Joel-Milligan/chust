use chust::board::Board;

fn main() {
    let board = Board::default();
    println!("{board}");

    let moves = board.get_legal_moves("b1".to_string()).unwrap();

    println!("\nMOVES:");
    for mv in moves {
        println!("{}", mv.destination());
    }
}
