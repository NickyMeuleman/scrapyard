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
// pub mod day_16;
// pub mod day_17;
// pub mod day_18;
// pub mod day_19;
// pub mod day_20;
// pub mod day_21;
// pub mod day_22;
// pub mod day_23;
// pub mod day_24;
// pub mod day_25;
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
