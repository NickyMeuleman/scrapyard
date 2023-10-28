#![feature(return_position_impl_trait_in_trait)]

use std::io;

pub use aoc_core::{part_helper, Answer, AoCDay, AoCError, Day, Part, Solution, Year, DAYS};

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

pub fn get_input(day: Day) -> io::Result<String> {
    let year = Year::try_new(2023).unwrap();
    aoc_core::get_input(year, day)
}

pub fn print_part(day: Day, part: &Part) {
    let year = Year::try_new(2023).unwrap();
    aoc_core::print_part(year, day, part, solve_part)
}

pub fn solve_part(day: Day, part: &Part, input: &str) -> Result<Answer, AoCError> {
    match day.value() {
        1 => part_helper::<day_01::Data>(day, part, input),
        2 => part_helper::<day_02::Data>(day, part, input),
        3 => part_helper::<day_03::Data>(day, part, input),
        4 => part_helper::<day_04::Data>(day, part, input),
        5 => part_helper::<day_05::Data>(day, part, input),
        6 => part_helper::<day_06::Data>(day, part, input),
        7 => part_helper::<day_07::Data>(day, part, input),
        8 => part_helper::<day_08::Data>(day, part, input),
        9 => part_helper::<day_09::Data>(day, part, input),
        10 => part_helper::<day_10::Data>(day, part, input),
        11 => part_helper::<day_11::Data>(day, part, input),
        12 => part_helper::<day_12::Data>(day, part, input),
        13 => part_helper::<day_13::Data>(day, part, input),
        14 => part_helper::<day_14::Data>(day, part, input),
        15 => part_helper::<day_15::Data>(day, part, input),
        16 => part_helper::<day_16::Data>(day, part, input),
        17 => part_helper::<day_17::Data>(day, part, input),
        18 => part_helper::<day_18::Data>(day, part, input),
        19 => part_helper::<day_19::Data>(day, part, input),
        20 => part_helper::<day_20::Data>(day, part, input),
        21 => part_helper::<day_21::Data>(day, part, input),
        22 => part_helper::<day_22::Data>(day, part, input),
        23 => part_helper::<day_23::Data>(day, part, input),
        24 => part_helper::<day_24::Data>(day, part, input),
        25 => part_helper::<day_25::Data>(day, part, input),
        n => Err(AoCError::Custom(format!(
            "Trying to solve an invalid day, found day: {n}"
        ))),
    }
}
