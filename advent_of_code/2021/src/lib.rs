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
pub mod utils;

use utils::{AoCData, Solution, run_day};
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn solve(day: u8, input: String) -> Solution {
    match day {
        1 => run_day::<day_01::Data>(input),
        2 => run_day::<day_02::Data>(input),
        3 => run_day::<day_03::Data>(input),
        4 => run_day::<day_04::Data>(input),
        5 => run_day::<day_05::Data>(input),
        6 => run_day::<day_06::Data>(input),
        7 => run_day::<day_07::Data>(input),
        8 => run_day::<day_08::Data>(input),
        9 => run_day::<day_09::Data>(input),
        10 => run_day::<day_10::Data>(input),
        11 => run_day::<day_11::Data>(input),
        12 => run_day::<day_12::Data>(input),
        13 => run_day::<day_13::Data>(input),
        14 => run_day::<day_14::Data>(input),
        15 => run_day::<day_15::Data>(input),
        16 => run_day::<day_16::Data>(input),
        17 => run_day::<day_17::Data>(input),
        18 => run_day::<day_18::Data>(input),
        19 => run_day::<day_19::Data>(input),
        20 => run_day::<day_20::Data>(input),
        21 => run_day::<day_21::Data>(input),
        22 => run_day::<day_22::Data>(input),
        23 => run_day::<day_23::Data>(input),
        24 => run_day::<day_24::Data>(input),
        25 => run_day::<day_25::Data>(input),
        _ => panic!("trying to solve invalid day"),
    }
}
