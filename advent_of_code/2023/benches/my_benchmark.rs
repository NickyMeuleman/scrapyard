use aoc2023::{day_01, day_02, get_input, AoCData, DAYS};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::env;

fn bench_main(c: &mut Criterion) {
    let mut args = env::args();

    let day: Option<u8> = {
        // the first argument is the location the program is running, we don't need that
        args.next();
        // the second argument should be the day
        match args.next() {
            Some(day) => day.parse().ok(),
            None => None,
        }
    };

    match day {
        Some(num) => {
            // bench single day
            bench_day(c, num);
        }
        None => {
            // bench all days
            for num in 1..=DAYS {
                bench_day(c, num);
                println!("\n");
            }
        }
    }
}

pub fn bench_day(c: &mut Criterion, day: u8) {
    let input = get_input(day, false).expect("Getting input failed");
    match day {
        1 => day_helper::<day_01::Data>(c, day, &input),
        2 => day_helper::<day_02::Data>(c, day, &input),
        n => panic!("Trying to bench an invalid day, found day {n}"),
    }
}

fn day_helper<'a, T: AoCData<'a>>(c: &mut Criterion, day: u8, input: &'a str) {
    let data = T::try_new(&input).unwrap();
    let mut group = c.benchmark_group(format!("Day {:02}", day));
    group.bench_function("Part 1", |b| b.iter(|| black_box(data.part_1())));
    group.bench_function("Part 2", |b| b.iter(|| black_box(data.part_2())));
    group.finish();
}

criterion_group!(benches, bench_main);
criterion_main!(benches);
