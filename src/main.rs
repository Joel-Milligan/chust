use chust::board::Board;
use chust::piece_move::Move;

fn main() {
    let mut board = Board::default();
    println!("{board}");

    println!("\n--------1--------");
    board.apply_move(Move::coordinate("e2e4".to_string()));
    println!("{board}");

    println!("\n--------2--------");
    board.apply_move(Move::coordinate("e7e5".to_string()));
    println!("{board}");

    println!("\n--------3--------");
    board.apply_move(Move::coordinate("g1f3".to_string()));
    println!("{board}");
}
