#![feature(test)]

pub mod day_01;
pub mod utils;

use utils::{run, AoCData, Solution};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn solve(day: u8, input: String) -> Result<Solution, JsError> {
    run(day, input)
}

pub fn solve_sync(day: u8, input: String) -> Solution {
    run(day, input).unwrap_or(Solution {
        part1: String::new(),
        part2: String::new(),
    })
}
