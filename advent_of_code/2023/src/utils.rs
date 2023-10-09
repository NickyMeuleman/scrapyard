use crate::day_01;
use std::{fs, io};
use wasm_bindgen::prelude::*;

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

pub fn get_input(day: u8, sample: bool) -> io::Result<String> {
    let num = format!("{:02}", day);
    let input_path = format!(
        "inputs/{}/day{}.txt",
        if sample { "samples" } else { "full" },
        num
    );

    fs::read_to_string(input_path)
}

// TODO: figure out a way the parts can return any type that implements Display
pub trait AoCData<'a> {
    /// Parse an input string into a Data struct for a specific day
    fn try_new(input: &'a str) -> Option<Self>
    where
        Self: Sized;

    /// part1 solution
    fn part_1(&self) -> String;

    /// part2 solution
    fn part_2(&self) -> String;

    /// both solutions
    fn solve(self) -> Solution
    where
        Self: Sized,
    {
        Solution {
            part1: self.part_1(),
            part2: self.part_2(),
        }
    }
}

pub const DAYS: u8 = 1;

pub fn run_day<'a, T: AoCData<'a>>(input: &'a str) -> Result<Solution, JsError> {
    if let Some(data) = T::try_new(input) {
        Ok(data.solve())
    } else {
        Err(JsError::new("Failed to parse"))
    }
}

pub fn run(day: u8, input: String) -> Result<Solution, JsError> {
    match day {
        1 => run_day::<day_01::Data>(&input),
        // 2 => run_day::<day_02::Data>(&input),
        // 3 => run_day::<day_03::Data>(&input),
        // 4 => run_day::<day_04::Data>(&input),
        // 5 => run_day::<day_05::Data>(&input),
        // 6 => run_day::<day_06::Data>(&input),
        // 7 => run_day::<day_07::Data>(&input),
        // 8 => run_day::<day_08::Data>(&input),
        // 9 => run_day::<day_09::Data>(&input),
        // 10 => run_day::<day_10::Data>(&input),
        // 11 => run_day::<day_11::Data>(&input),
        // 12 => run_day::<day_12::Data>(&input),
        // 13 => run_day::<day_13::Data>(&input),
        // 14 => run_day::<day_14::Data>(&input),
        // 15 => run_day::<day_15::Data>(&input),
        // 16 => run_day::<day_16::Data>(&input),
        // 17 => run_day::<day_17::Data>(&input),
        // 18 => run_day::<day_18::Data>(&input),
        // 19 => run_day::<day_19::Data>(&input),
        // 20 => run_day::<day_20::Data>(&input),
        // 21 => run_day::<day_21::Data>(&input),
        // 22 => run_day::<day_22::Data>(&input),
        // 23 => run_day::<day_23::Data>(&input),
        // 24 => run_day::<day_24::Data>(&input),
        // 25 => run_day::<day_25::Data>(&input),
        _ => panic!("trying to solve invalid day"),
    }
}

pub fn solve_sync(day: u8, input: String) -> Solution {
    run(day, input).unwrap_or(Solution {
        part1: String::new(),
        part2: String::new(),
    })
}
