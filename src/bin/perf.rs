#![feature(int_roundings)]
use chust::board::Board;
use chust::engine::Engine;
use chust::uci::Uci;

fn main() {
    let board =
        Board::from_fen("rn3r1k/p3qp2/bp2p2p/3pP3/P2NRQ2/1Pb2NPP/5PB1/2R3K1 w - - 1 22").unwrap();
    let mut engine = Engine::new();
    engine.board = board;

    for depth in 0..=5 {
        let eval = engine.search_depth(depth);
        Uci::write_info(
            depth,
            engine.nodes,
            eval,
            engine.pv_length[0],
            &engine.pv_table[0],
        );
    }

    if let Some(best_move) = engine.pv_table[0][0] {
        println!("bestmove {}", best_move);
    }
}
