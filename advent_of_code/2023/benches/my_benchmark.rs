use aoc2023::{
    day_01,
    utils::{self, AoCData},
};
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
            for num in 1..=utils::DAYS {
                bench_day(c, num);
                println!("\n");
            }
        }
    }
}

pub fn bench_day(c: &mut Criterion, day: u8) {
    let input = utils::get_input(day, false).expect("Getting input failed");
    let data = match day {
        1 => day_01::Data::try_new(&input),
        // 2 => run_day::<day_02::Data>(&input),
        // 3 => run_day::<day_03::Data>(&input),
        // 4 => run_day::<day_04::Data>(&input),
        // 5 => run_day::<day_05::Data>(&input),
        // 6 => run_day::<day_06::Data>(&input),
        // 7 => run_day::<day_07::Data>(&input),
        // 8 => run_day::<day_08::Data>(&input),
        // 9 => run_day::<day_09::Data>(&input),
        // 10 => run_day::<day_10::Data>(&input),
        // 11 => run_day::<day_11::Data>(&input),
        // 12 => run_day::<day_12::Data>(&input),
        // 13 => run_day::<day_13::Data>(&input),
        // 14 => run_day::<day_14::Data>(&input),
        // 15 => run_day::<day_15::Data>(&input),
        // 16 => run_day::<day_16::Data>(&input),
        // 17 => run_day::<day_17::Data>(&input),
        // 18 => run_day::<day_18::Data>(&input),
        // 19 => run_day::<day_19::Data>(&input),
        // 20 => run_day::<day_20::Data>(&input),
        // 21 => run_day::<day_21::Data>(&input),
        // 22 => run_day::<day_22::Data>(&input),
        // 23 => run_day::<day_23::Data>(&input),
        // 24 => run_day::<day_24::Data>(&input),
        // 25 => run_day::<day_25::Data>(&input),
        _ => panic!("trying to solve invalid day"),
    }
    .expect("parsing failed");

    let mut group = c.benchmark_group(format!("Day {:02}", day));
    group.bench_function("Part 1", |b| b.iter(|| black_box(data.part_1())));
    group.bench_function("Part 2", |b| b.iter(|| black_box(data.part_2())));
    group.finish();
}

criterion_group!(benches, bench_main);
criterion_main!(benches);
