#![feature(return_position_impl_trait_in_trait)]

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
use std::{fmt::Display, fs, io, time::Instant};
use wasm_bindgen::prelude::*;

pub const DAYS: u8 = 2;

pub enum Answer {
    Part(String),
    Both(Solution),
}

// https://github.com/rustwasm/wasm-bindgen/issues/1775
// https://stackoverflow.com/questions/68243940/rust-wasm-bindgen-struct-with-string
// Strings on a struct can not be public
// skip the fields with a wasm_bindgen macro, but implement a getter for them so you can access them in JS.
// a weird disable only to later enable, I know.
// You can do without the wasm_bindgen skip on the fields, but since my other Rust code accesses them, I need them to be public.

pub struct Solution {
    part1: Box<dyn Display>,
    part2: Box<dyn Display>,
}

#[wasm_bindgen(getter_with_clone)]
pub struct WasmSolution {
    pub part1: Option<String>,
    pub part2: Option<String>,
}

impl From<Solution> for WasmSolution {
    fn from(value: Solution) -> Self {
        WasmSolution {
            part1: Some(value.part1.to_string()),
            part2: Some(value.part2.to_string()),
        }
    }
}

pub trait AoCData<'a> {
    /// Parse an input string into a Data struct for a specific day
    fn try_new(input: &'a str) -> Option<Self>
    where
        Self: Sized;

    /// part1 solution
    fn part_1(&self) -> impl Display;

    /// part2 solution
    fn part_2(&self) -> impl Display;

    /// both solutions
    fn solve(self) -> Solution
    where
        Self: Sized,
    {
        // have to make sure results that come back from the part functions live long enough,
        // because those might be borrowed things that impl Display
        Solution {
            part1: Box::new(self.part_1().to_string()),
            part2: Box::new(self.part_2().to_string()),
        }
    }
}

#[derive(Debug)]
#[wasm_bindgen]
pub enum Part {
    One = 1,
    Two = 2,
    Both = 3,
}

pub fn get_input(day: u8, sample: bool) -> io::Result<String> {
    let num = format!("{:02}", day);
    let input_path = format!(
        "inputs/{}/day{}.txt",
        if sample { "samples" } else { "full" },
        num
    );

    fs::read_to_string(input_path)
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
    let input = get_input(day, false).unwrap();
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

pub fn part_helper<'a, T: AoCData<'a>>(
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

fn solve_part(day: u8, input: &str, part: &Part) -> Result<Answer, String> {
    match day {
        1 => part_helper::<day_01::Data>(day, input, part),
        2 => part_helper::<day_02::Data>(day, input, part),
        3 => part_helper::<day_03::Data>(day, input, part),
        4 => part_helper::<day_04::Data>(day, input, part),
        5 => part_helper::<day_05::Data>(day, input, part),
        6 => part_helper::<day_06::Data>(day, input, part),
        n => Err(format!("Trying to solve an invalid day, found day: {n}")),
    }
}

#[wasm_bindgen]
pub async fn solve(day: u8, input: String, part: Part) -> Result<WasmSolution, JsError> {
    // wasm bindgen can't handle enums with values yet
    // see: https://github.com/rustwasm/wasm-bindgen/issues/2407
    // so we do some data janitoring and return a Solution for every Answer enum variant and fill the missing field with an empty string (yuck)
    match solve_part(day, &input, &part) {
        Ok(answer) => match answer {
            Answer::Part(result) => match part {
                Part::One => Ok(WasmSolution {
                    part1: Some(result),
                    part2: None,
                }),
                Part::Two => Ok(WasmSolution {
                    part1: None,
                    part2: Some(result),
                }),
                _ => unreachable!(),
            },
            Answer::Both(solution) => Ok(solution.into()),
        },
        Err(error) => Err(JsError::new(&error)),
    }
}
