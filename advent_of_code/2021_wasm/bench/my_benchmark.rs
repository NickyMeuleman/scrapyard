use aoc2021::{
    day_01, day_02, day_03, day_04, day_05, day_06, day_07, day_08, day_09, day_10,
    utils::{self, AoCData},
};
use criterion::{criterion_group, criterion_main, Criterion};
use std::env;

// not sure how arguments work here, but passing one seems to work
// passing more errors, but we only need one, so :shrug:
fn criterion_benchmark(c: &mut Criterion) {
    let mut args = env::args();

    let day: Option<u8> = {
        // the first argument is the location the program is running, we don't need that
        args.next();
        // the second argument should be the day
        match args.next() {
            Some(day) => day.parse().ok(),
            None => panic!("No day specified"),
        }
    };

    match day {
        Some(num) => {
            // run single day
            run(c, num)
        }
        None => {
            // run all days
            for idx in 1..=utils::DAYS {
                run(c, idx)
            }
        }
    }
}

fn run(c: &mut Criterion, num: u8) {
    // run single day
    let input = utils::get_input(num);

    match num {
        1 => run_day::<day_01::Data>(c, num, input),
        2 => run_day::<day_02::Data>(c, num, input),
        3 => run_day::<day_03::Data>(c, num, input),
        4 => run_day::<day_04::Data>(c, num, input),
        5 => run_day::<day_05::Data>(c, num, input),
        6 => run_day::<day_06::Data>(c, num, input),
        7 => run_day::<day_07::Data>(c, num, input),
        8 => run_day::<day_08::Data>(c, num, input),
        9 => run_day::<day_09::Data>(c, num, input),
        10 => run_day::<day_10::Data>(c, num, input),
        _ => todo!("not implemented yet"),
    }

    println!("\n\n");
}

fn run_day<T: AoCData>(c: &mut Criterion, num: u8, input: String) {
    println!("Running benchmarks for day{:02}", num);

    let data = T::new(input);

    let mut group = c.benchmark_group(format!("Day {:02}", num));

    // TODO: benchmark for parsing
    // put todo because I couldn't figure out how to get the correct struct since data is an impl Trait and not a specific struct

    group.bench_function("Part 1", |b| b.iter(|| data.part_1()));

    group.bench_function("Part 2", |b| b.iter(|| data.part_2()));

    group.finish()
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
