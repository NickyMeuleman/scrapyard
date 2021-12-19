use criterion::{
    black_box, criterion_group, criterion_main, Criterion,
};

use day_19::Data;

const INPUT: &'static str = include_str!("../input.txt");

fn criterion_benchmark(c: &mut Criterion) {
    let data: Data = black_box(INPUT.parse().unwrap());
    let mut group = c.benchmark_group("reduces-sample-size");
    group.sample_size(10);

    group.bench_function("part1", |b| {
        b.iter(|| data.clone().part_one())
    });

    group.bench_function("part2", |b| {
        b.iter(|| data.part_two())
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);