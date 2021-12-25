use criterion::{
    black_box, criterion_group, criterion_main, Criterion,
};

use day_25::Data;

const INPUT: &'static str = include_str!("../input.txt");

fn criterion_benchmark(c: &mut Criterion) {
    let data: Data = black_box(INPUT.parse().unwrap());

    c.bench_function("part1", |b| {
        b.iter(|| data.clone().part_one())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);