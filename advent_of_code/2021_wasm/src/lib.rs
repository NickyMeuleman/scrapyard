pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod utils;

use utils::AoCData;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn solve(day: u8, input: String) -> String {
    format_day(day, &input)
}

fn format_day(day: u8, input: &str) -> String {
    let (part1, part2) = utils::run(day, input);
    format!("{},{}", part1, part2)
}
