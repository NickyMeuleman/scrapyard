use crate::{day_01, day_02, day_03, day_04, day_05, day_06, day_07, day_08};

use std::fs;
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

pub fn get_input(day: u8) -> String {
    let num = if day >= 10 {
        day.to_string()
    } else {
        "0".to_string() + &day.to_string()
    };
    let input_path = "inputs/day".to_string() + &num + ".txt";
    fs::read_to_string(input_path).unwrap()
}

pub fn get_sample_input(day: u8) -> String {
    let num = if day >= 10 {
        day.to_string()
    } else {
        "0".to_string() + &day.to_string()
    };
    let input_path = "inputs/day".to_string() + &num + "_sample.txt";
    fs::read_to_string(input_path).unwrap()
}

// TODO: figure out a way the parts can return any type that implements Display
pub trait AoCData {
    /// Parse an input string into a Data struct for a specific day
    // fn new(input: String) -> Self;
    fn try_new(input: String) -> Option<Self>
    where
        Self: Sized;
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

    /// part1 solution
    fn part_1(&self) -> String;

    /// part2 solution
    fn part_2(&self) -> String;
}

pub const DAYS: u8 = 1;

pub fn run_day<T: AoCData>(input: String) -> Result<Solution, JsError> {
    if let Some(data) = T::try_new(input) {
        Ok(data.solve())
    } else {
        Err(JsError::new("Failed to parse"))
    }
}

pub fn run(day: u8, input: String) -> Result<Solution, JsError> {
    match day {
        1 => run_day::<day_01::Data>(input),
        2 => run_day::<day_02::Data>(input),
        3 => run_day::<day_03::Data>(input),
        4 => run_day::<day_04::Data>(input),
        5 => run_day::<day_05::Data>(input),
        6 => run_day::<day_06::Data>(input),
        7 => run_day::<day_07::Data>(input),
        8 => run_day::<day_08::Data>(input),
        // 9 => run_day::<day_09::Data>(input),
        // 10 => run_day::<day_10::Data>(input),
        // 11 => run_day::<day_11::Data>(input),
        // 12 => run_day::<day_12::Data>(input),
        // 13 => run_day::<day_13::Data>(input),
        // 14 => run_day::<day_14::Data>(input),
        // 15 => run_day::<day_15::Data>(input),
        // 16 => run_day::<day_16::Data>(input),
        // 17 => run_day::<day_17::Data>(input),
        // 18 => run_day::<day_18::Data>(input),
        // 19 => run_day::<day_19::Data>(input),
        // 20 => run_day::<day_20::Data>(input),
        // 21 => run_day::<day_21::Data>(input),
        // 22 => run_day::<day_22::Data>(input),
        // 23 => run_day::<day_23::Data>(input),
        // 24 => run_day::<day_24::Data>(input),
        // 25 => run_day::<day_25::Data>(input),
        _ => panic!("trying to solve invalid day"),
    }
}
