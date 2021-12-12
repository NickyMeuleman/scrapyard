use criterion::{
    black_box, criterion_group, criterion_main, Criterion,
};
// How do I inport Data from main.rs?
// use day_11::Data;
// use crate::day_11::Data;
// use super::Data;
// use lib::Data;
// use self::Data;

// trying with a copy in lib.rs
// this statement spits out something when I cargo bench
// still has red squigglies though
// unresolved import `day_11`
// use of undeclared crate or module `day_11`rustc(E0432)
// my_benchmark.rs(13, 5): use of undeclared crate or module `day_11`
// use day_11::Data;
use day_11::Data;

const INPUT: &'static str = include_str!("../input.txt");

fn criterion_benchmark(c: &mut Criterion) {
    let data: Data = black_box(INPUT.parse().unwrap());
    c.bench_function("part1", |b| {
        b.iter(|| data.clone().part_one())
    });

    c.bench_function("part2", |b| {
        b.iter(|| data.clone().part_two())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);