#![feature(return_position_impl_trait_in_trait)]

pub use aoc_core::{
    part_helper, Answer, AoCData, AoCError, AoCResult, Day, Part, Solution, Year, DAYS,
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
pub mod day_13;
pub mod day_14;
pub mod day_15;
pub mod day_16;
pub mod day_17;
pub mod day_18;
pub mod day_19;
pub mod day_20;
pub mod day_21;
pub mod day_22;
pub mod day_23;
pub mod day_24;
pub mod day_25;

pub fn get_input(day: &Day) -> AoCResult<String> {
    let year = Year::try_new(2021)?;
    aoc_core::get_input(&year, day)
}

pub fn print_part(day: &Day, part: &Part) {
    let year = Year::try_new(2021).unwrap();
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
        13 => part_helper::<day_13::Data>(part, input),
        14 => part_helper::<day_14::Data>(part, input),
        15 => part_helper::<day_15::Data>(part, input),
        16 => part_helper::<day_16::Data>(part, input),
        17 => part_helper::<day_17::Data>(part, input),
        18 => part_helper::<day_18::Data>(part, input),
        19 => part_helper::<day_19::Data>(part, input),
        20 => part_helper::<day_20::Data>(part, input),
        21 => part_helper::<day_21::Data>(part, input),
        22 => part_helper::<day_22::Data>(part, input),
        23 => part_helper::<day_23::Data>(part, input),
        24 => part_helper::<day_24::Data>(part, input),
        25 => part_helper::<day_25::Data>(part, input),
        n => Err(AoCError::new(format!(
            "Trying to solve an invalid day, found day: {n}"
        ))),
    }
}
