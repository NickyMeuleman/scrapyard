#![feature(return_position_impl_trait_in_trait)]

use std::{io, time::Instant};

pub use aoc_core::{Answer, AoCDay, Part, Solution, DAYS};

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

pub fn get_input(day: u8) -> io::Result<String> {
    aoc_core::get_input(day, 2023)
}

pub fn print_part(day: u8, part: &Part) {
    println!(
        "Day {}, {}",
        day,
        match part {
            Part::One => "part 1",
            Part::Two => "part 2",
            Part::Both => "both parts",
        }
    );
    println!("{:=<20}", "=");
    let input = get_input(day).unwrap();
    let now = Instant::now();
    let result = solve_part(day, &input, &part);
    let elapsed = now.elapsed();
    println!("Runtime:");
    println!("{:?}", elapsed);
    match result.unwrap_or(Answer::Part("No result!".to_string())) {
        Answer::Part(result) => {
            println!("Answer:");
            println!("{result}");
        }
        Answer::Both(solution) => {
            println!("Part 1:");
            println!("{}", solution.part1);
            println!("Part 2:");
            println!("{}", solution.part2);
        }
    }
}

pub fn part_helper<'a, T: AoCDay<'a>>(
    day: u8,
    input: &'a str,
    part: &Part,
) -> Result<Answer, String> {
    if let Some(data) = T::try_new(input) {
        let answer = match part {
            Part::One => Answer::Part(data.part_1().to_string()),
            Part::Two => Answer::Part(data.part_2().to_string()),
            Part::Both => Answer::Both(data.solve()),
        };
        Ok(answer)
    } else {
        Err(format!("Failed to parse day {day}"))
    }
}

pub fn solve_part(day: u8, input: &str, part: &Part) -> Result<Answer, String> {
    match day {
        1 => part_helper::<day_01::Data>(day, input, part),
        2 => part_helper::<day_02::Data>(day, input, part),
        3 => part_helper::<day_03::Data>(day, input, part),
        4 => part_helper::<day_04::Data>(day, input, part),
        5 => part_helper::<day_05::Data>(day, input, part),
        6 => part_helper::<day_06::Data>(day, input, part),
        7 => part_helper::<day_07::Data>(day, input, part),
        8 => part_helper::<day_08::Data>(day, input, part),
        9 => part_helper::<day_09::Data>(day, input, part),
        10 => part_helper::<day_10::Data>(day, input, part),
        11 => part_helper::<day_11::Data>(day, input, part),
        12 => part_helper::<day_12::Data>(day, input, part),
        13 => part_helper::<day_13::Data>(day, input, part),
        14 => part_helper::<day_14::Data>(day, input, part),
        15 => part_helper::<day_15::Data>(day, input, part),
        16 => part_helper::<day_16::Data>(day, input, part),
        17 => part_helper::<day_17::Data>(day, input, part),
        18 => part_helper::<day_18::Data>(day, input, part),
        19 => part_helper::<day_19::Data>(day, input, part),
        20 => part_helper::<day_20::Data>(day, input, part),
        21 => part_helper::<day_21::Data>(day, input, part),
        22 => part_helper::<day_22::Data>(day, input, part),
        23 => part_helper::<day_23::Data>(day, input, part),
        24 => part_helper::<day_24::Data>(day, input, part),
        25 => part_helper::<day_25::Data>(day, input, part),
        n => Err(format!("Trying to solve an invalid day, found day: {n}")),
    }
}
