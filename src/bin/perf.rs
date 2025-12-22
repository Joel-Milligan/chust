#![feature(int_roundings)]
use chust::board::Board;
use chust::engine::Engine;

#[allow(dead_code)]
const MATE_IN_THREE: &'static str = "rn3r1k/p3qp2/bp2p2p/3pP3/P2NRQ2/1Pb2NPP/5PB1/2R3K1 w - - 1 22";
#[allow(dead_code)]
const TRICKY: &'static str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
#[allow(dead_code)]
const BROKEN_MATE: &'static str =
    "r1bqk2r/2ppb1p1/n3P2p/8/2B1nP2/4P3/1PPP3P/RNBQK1NR w KQkq - 0 10";

fn main() {
    let mut engine = Engine::new();
    engine.board = Board::from_fen(MATE_IN_THREE).unwrap();
    engine.search_depth(20);
}
