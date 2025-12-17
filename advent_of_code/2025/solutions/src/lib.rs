pub use aoc_core::{
    Answer, AoCData, AoCError, AoCResult, DAYS, Day, Part, Solution, Year, part_helper,
};
pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;
pub mod day_10;
pub mod day_11;
pub mod day_12;

pub fn get_input(day: &Day) -> AoCResult<String> {
    let year = Year::try_new(2025)?;
    aoc_core::get_input(&year, day)
}

pub fn print_part(day: &Day, part: &Part) {
    let year = Year::try_new(2025).unwrap();
    aoc_core::print_part(&year, day, part, solve_part)
}

pub fn solve_part(day: &Day, part: &Part, input: &str) -> AoCResult<Answer> {
    match day.value() {
        1 => part_helper::<day_01::Data>(part, input),
        2 => part_helper::<day_02::Data>(part, input),
        3 => part_helper::<day_03::Data>(part, input),
        4 => part_helper::<day_04::Data>(part, input),
        5 => part_helper::<day_05::Data>(part, input),
        6 => part_helper::<day_06::Data>(part, input),
        7 => part_helper::<day_07::Data>(part, input),
        8 => part_helper::<day_08::Data>(part, input),
        9 => part_helper::<day_09::Data>(part, input),
        10 => part_helper::<day_10::Data>(part, input),
        11 => part_helper::<day_11::Data>(part, input),
        12 => part_helper::<day_12::Data>(part, input),
        n => Err(AoCError::new(format!(
            "Trying to solve an invalid day, found day: {n}"
        ))),
    }
}
