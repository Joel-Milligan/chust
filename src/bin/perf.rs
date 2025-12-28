#![feature(int_roundings)]
use chust::board::Board;
use chust::engine::Engine;

#[allow(dead_code)]
const MATE_IN_2: &'static str = "4r3/1pp2rbk/6pn/4n3/P3BN1q/1PB2bPP/8/2Q1RRK1 b - - 0 31";
#[allow(dead_code)]
const MATE_IN_3: &'static str = "rn3r1k/p3qp2/bp2p2p/3pP3/P2NRQ2/1Pb2NPP/5PB1/2R3K1 w - - 1 22";
#[allow(dead_code)]
const MATED_IN_2: &'static str = "rn3r1k/p3qp2/bp2p2Q/3pP3/P2NR3/1Pb2NPP/5PB1/2R3K1 b - - 0 22";
#[allow(dead_code)]
const TRICKY: &'static str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
#[allow(dead_code)]
const BROKEN_MATE: &'static str =
    "r1bqk2r/2ppb1p1/n3P2p/8/2B1nP2/4P3/1PPP3P/RNBQK1NR w KQkq - 0 10";
#[allow(dead_code)]
const MATE_IN_4: &'static str =
    "r1bqk1nr/pp1p2bp/4n3/2p1Npp1/5P2/2N1P1PP/PPP5/1RBQKB1R w Kkq - 0 10";
#[allow(dead_code)]
const MATE_IN_5: &'static str = "4rb1k/2pqn2p/6pn/ppp3N1/P1QP2b1/1P2p3/2B3PP/B3RRK1 w - - 0 24";

fn main() {
    let mut engine = Engine::new();
    engine.board = Board::from_fen(MATE_IN_3).unwrap();
    engine.search_depth(5);
}
