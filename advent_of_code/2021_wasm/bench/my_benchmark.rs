use aoc2021::{
    day_01, day_02, day_03, day_04, day_05, day_06, day_07, day_08, day_09, day_10, day_11, day_12,
    day_13, day_14, day_15, day_16, day_17, day_18, day_19, day_20, day_21,
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
        11 => run_day::<day_11::Data>(c, num, input),
        12 => run_day::<day_12::Data>(c, num, input),
        13 => run_day::<day_13::Data>(c, num, input),
        14 => run_day::<day_14::Data>(c, num, input),
        15 => run_day::<day_15::Data>(c, num, input),
        16 => run_day::<day_16::Data>(c, num, input),
        17 => run_day::<day_17::Data>(c, num, input),
        18 => run_day::<day_18::Data>(c, num, input),
        19 => run_day::<day_19::Data>(c, num, input),
        20 => run_day::<day_20::Data>(c, num, input),
        21 => run_day::<day_21::Data>(c, num, input),
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

    match num {
        19 => {
            // so, day 19 takes a long time.
            // I'm not waiting for the standard sample size
            group.sample_size(10);
        }
        _ => {}
    }

    group.bench_function("Part 1", |b| b.iter(|| data.part_1()));

    group.bench_function("Part 2", |b| b.iter(|| data.part_2()));

    group.finish()
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
