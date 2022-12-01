use aoc2015::{
    day_01, day_02, day_03, day_04,
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
        1 => bench_day::<day_01::Data>(c, num, input),
        2 => bench_day::<day_02::Data>(c, num, input),
        3 => bench_day::<day_03::Data>(c, num, input),
        4 => bench_day::<day_04::Data>(c, num, input),
        // 5 => bench_day::<day_05::Data>(c, num, input),
        // 6 => bench_day::<day_06::Data>(c, num, input),
        // 7 => bench_day::<day_07::Data>(c, num, input),
        // 8 => bench_day::<day_08::Data>(c, num, input),
        // 9 => bench_day::<day_09::Data>(c, num, input),
        // 10 => bench_day::<day_10::Data>(c, num, input),
        // 11 => bench_day::<day_11::Data>(c, num, input),
        // 12 => bench_day::<day_12::Data>(c, num, input),
        // 13 => bench_day::<day_13::Data>(c, num, input),
        // 14 => bench_day::<day_14::Data>(c, num, input),
        // 15 => bench_day::<day_15::Data>(c, num, input),
        // 16 => bench_day::<day_16::Data>(c, num, input),
        // 17 => bench_day::<day_17::Data>(c, num, input),
        // 18 => bench_day::<day_18::Data>(c, num, input),
        // 19 => bench_day::<day_19::Data>(c, num, input),
        // 20 => bench_day::<day_20::Data>(c, num, input),
        // 21 => bench_day::<day_21::Data>(c, num, input),
        // 22 => bench_day::<day_22::Data>(c, num, input),
        // 23 => bench_day::<day_23::Data>(c, num, input),
        // 24 => bench_day::<day_24::Data>(c, num, input),
        // 25 => bench_day::<day_25::Data>(c, num, input),
        _ => panic!("trying to bench invalid day"),
    }

    println!("\n\n");
}

fn bench_day<T: AoCData>(c: &mut Criterion, num: u8, input: String) {
    println!("Running benchmarks for day{:02}", num);

    let data = T::try_new(input).unwrap();

    let day = if num >= 10 {
        num.to_string()
    } else {
        "0".to_string() + &num.to_string()
    };
    let mut group = c.benchmark_group("Day ".to_string() + &day);

    // TODO: benchmark for parsing
    // put todo because I couldn't figure out how to get the correct struct since data is an impl Trait and not a specific struct

    match num {
        19 | 24 => {
            // so, these days takes a long time.
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
