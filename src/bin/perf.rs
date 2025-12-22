#![feature(int_roundings)]
use chust::board::Board;
use chust::engine::Engine;
use chust::uci::Uci;

fn main() {
    let board =
        Board::from_fen("rn3r1k/p3qp2/bp2p2p/3pP3/P2NRQ2/1Pb2NPP/5PB1/2R3K1 w - - 1 22").unwrap();
    let mut engine = Engine::new();
    engine.board = board;

    let mut best_move = None;
    for depth in 0..=5 {
        let (pv, eval) = engine.search_depth(depth);
        best_move = Some(pv.first().unwrap().clone());
        Uci::write_info(depth, engine.nodes, eval, &pv);
    }

    if let Some(best_move) = best_move {
        println!("bestmove {}", best_move);
    }
}
