use crate::{
    day_01, day_02, day_03, day_04, day_05, day_06, day_07, day_08, day_09, day_10, day_11, day_12,
    day_13, day_14, day_15, day_16, day_17, day_18, day_19,
};
use std::fs;

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
    fn solve(self) -> (String, String)
    where
        Self: Sized,
    {
        (self.part_1(), self.part_2())
    }

    /// part1 solution
    fn part_1(&self) -> String;

    /// part2 solution
    fn part_2(&self) -> String;
}

pub const DAYS: u8 = 17;

pub fn run(day: u8, input: String) -> (String, String) {
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
        _ => todo!("not implemented yet"),
    }
}

fn run_day<T: AoCData>(input: String) -> (String, String) {
    let data = T::new(input);
    data.solve()
}
