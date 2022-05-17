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
    let input_path = format!("inputs/day{:02}.txt", day);
    fs::read_to_string(input_path).unwrap()
}

pub fn get_sample_input(day: u8) -> String {
    let input_path = format!("inputs/day{:02}_sample.txt", day);
    fs::read_to_string(input_path).unwrap()
}

// TODO: figure out a way the parts can return any type that implements Display
pub trait AoCData {
    /// Parse an input string into a Data struct for a specific day
    fn new(input: String) -> Self;

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

pub const DAYS: u8 = 25;

pub fn run_day<T: AoCData>(input: String) -> Solution {
    let data = T::new(input);
    data.solve()
}
