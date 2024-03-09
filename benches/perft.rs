use chust::board::Board;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn perft_bench(c: &mut Criterion) {
    let mut board = Board::default();
    c.bench_function("perft 3", |b| b.iter(|| board.perft(black_box(3))));
}

criterion_group!(benches, perft_bench);
criterion_main!(benches);
