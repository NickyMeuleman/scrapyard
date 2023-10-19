use aoc2023::{day_01, day_02, day_03, day_04, day_05, day_06, get_input, AoCData, DAYS};
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
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
    let input = get_input(day).expect("Getting input failed");
    match day {
        1 => day_helper::<day_01::Data>(c, day, &input),
        2 => day_helper::<day_02::Data>(c, day, &input),
        3 => day_helper::<day_03::Data>(c, day, &input),
        4 => day_helper::<day_04::Data>(c, day, &input),
        5 => day_helper::<day_05::Data>(c, day, &input),
        6 => day_helper::<day_06::Data>(c, day, &input),
        n => panic!("Trying to bench an invalid day, found day {n}"),
    }
}

fn day_helper<'a, T: AoCData<'a> + Clone>(c: &mut Criterion, day: u8, input: &'a str) {
    let mut group = c.benchmark_group(format!("Day {:02}", day));
    group.bench_function("Parsing", |b| b.iter(|| black_box(T::try_new(&input))));
    let data = T::try_new(&input).unwrap();
    group.bench_function("Part 1", |b| b.iter(|| black_box(data.part_1())));
    group.bench_function("Part 2", |b| b.iter(|| black_box(data.part_2())));
    group.bench_function("Both parts", |b| {
        b.iter_batched(
            || data.clone(),
            |data| black_box(data.solve()),
            BatchSize::SmallInput,
        )
    });
    group.finish();
}

criterion_group!(benches, bench_main);
criterion_main!(benches);
