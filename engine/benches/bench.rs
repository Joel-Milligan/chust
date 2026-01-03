use chust_engine::Board;
use chust_engine::Engine;
use criterion::{Criterion, criterion_group, criterion_main};

pub fn perft_bench(c: &mut Criterion) {
    let mut board = Board::default();
    c.bench_function("perft", |b| b.iter(|| board.perft(std::hint::black_box(5))));
}

pub fn search_bench(c: &mut Criterion) {
    let board = Board::from_fen("4r3/1pp2rbk/6pn/4n3/P3BN1q/1PB2bPP/8/2Q1RRK1 b - - 0 31").unwrap();

    c.bench_function("search: mate in 2", |b| {
        b.iter(|| {
            let mut engine = Engine::new();
            engine.board = board.clone();
            engine.search_depth(std::hint::black_box(3));
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = perft_bench, search_bench
}

criterion_main!(benches);
