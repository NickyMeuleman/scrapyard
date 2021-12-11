use criterion::{
    black_box, criterion_group, criterion_main, Criterion,
};
// How do I inport Data from main.rs?
// use day_11::Data;
// use crate::day_11::Data;
// use super::Data;
// use lib::Data;
// use self::Data;

const INPUT: &'static str = include_str!("../input.txt");

fn criterion_benchmark(c: &mut Criterion) {
    let data: Data = INPUT.parse().unwrap();
    c.bench_function("part1", |b| {
        b.iter(|| data.part_one(black_box(INPUT)))
    });

    c.bench_function("part2", |b| {
        b.iter(|| data.part_two(black_box(INPUT)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);