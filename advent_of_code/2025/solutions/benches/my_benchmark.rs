use aoc2025::{
    AoCData, Day, day_01, day_02, day_03, day_04, day_05, day_06, day_07, day_08, day_09, day_10,
    day_11, day_12, get_input,
};
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::time::Duration;

fn bench_main(c: &mut Criterion) {
    for num in 1..=12 {
        let day = Day::try_new(num).unwrap();
        bench_day(c, &day)
    }
}

pub fn bench_day(c: &mut Criterion, day: &Day) {
    let input = get_input(day).expect("Getting input failed");
    match day.value() {
        1 => day_helper::<day_01::Data>(c, day, &input),
        2 => day_helper::<day_02::Data>(c, day, &input),
        3 => day_helper::<day_03::Data>(c, day, &input),
        4 => day_helper::<day_04::Data>(c, day, &input),
        5 => day_helper::<day_05::Data>(c, day, &input),
        6 => day_helper::<day_06::Data>(c, day, &input),
        7 => day_helper::<day_07::Data>(c, day, &input),
        8 => day_helper::<day_08::Data>(c, day, &input),
        9 => day_helper::<day_09::Data>(c, day, &input),
        10 => day_helper::<day_10::Data>(c, day, &input),
        11 => day_helper::<day_11::Data>(c, day, &input),
        12 => day_helper::<day_12::Data>(c, day, &input),
        n => panic!("Trying to bench an invalid day, found day {n}"),
    }
}

fn day_helper<'a, T: AoCData<'a> + Clone>(c: &mut Criterion, day: &Day, input: &'a str) {
    let mut group = c.benchmark_group(format!("Day {:02}", day.value()));
    group.sample_size(50);
    group.measurement_time(Duration::from_secs(5));

    group.bench_function("Parsing", |b| {
        b.iter(|| T::try_new(black_box(input)).unwrap())
    });

    let data = T::try_new(input).unwrap();

    group.bench_function("Part 1", |b| b.iter(|| black_box(data.part_1())));
    group.bench_function("Part 2", |b| b.iter(|| black_box(data.part_2())));

    group.bench_function("Both parts", |b| {
        b.iter_batched(
            || data.clone(),
            |d| black_box(d.solve()),
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group!(benches, bench_main);
criterion_main!(benches);
