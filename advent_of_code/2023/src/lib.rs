#![feature(return_position_impl_trait_in_trait)]

pub mod day_01;
pub mod day_02;
use std::{fmt::Display, fs, io};
use wasm_bindgen::prelude::*;

pub const DAYS: u8 = 2;

// https://github.com/rustwasm/wasm-bindgen/issues/1775
// https://stackoverflow.com/questions/68243940/rust-wasm-bindgen-struct-with-string
// Strings on a struct can not be public
// skip the fields with a wasm_bindgen macro, but implement a getter for them so you can access them in JS.
// a weird disable only to later enable, I know.
// You can do without the wasm_bindgen skip on the fields, but since my other Rust code accesses them, I need them to be public.
#[wasm_bindgen]
pub struct Solution {
    #[wasm_bindgen(skip)]
    pub part1: String,
    #[wasm_bindgen(skip)]
    pub part2: String,
}

#[wasm_bindgen]
impl Solution {
    #[wasm_bindgen(getter)]
    pub fn part1(&self) -> String {
        self.part1.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn part2(&self) -> String {
        self.part2.clone()
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
        Solution {
            part1: self.part_1().to_string(),
            part2: self.part_2().to_string(),
        }
    }
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

pub fn print_day(num: u8) {
    println!("Day{:02}", num);
    let input = get_input(num, false).unwrap();
    let Solution { part1, part2 } = solve_day(num, input).unwrap_or(Solution {
        part1: String::new(),
        part2: String::new(),
    });
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

pub fn day_helper<'a, T: AoCData<'a>>(day: u8, input: &'a str) -> Result<Solution, JsError> {
    if let Some(data) = T::try_new(input) {
        Ok(data.solve())
    } else {
        Err(JsError::new(&format!("Failed to parse day {day}")))
    }
}

fn solve_day(day: u8, input: String) -> Result<Solution, JsError> {
    match day {
        1 => day_helper::<day_01::Data>(day, &input),
        2 => day_helper::<day_02::Data>(day, &input),
        n => Err(JsError::new(&format!(
            "Trying to solve an invalid day, found day {n}"
        ))),
    }
}

#[wasm_bindgen]
pub async fn solve(day: u8, input: String) -> Result<Solution, JsError> {
    solve_day(day, input)
}
