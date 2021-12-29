use crate::{day_01, day_02, day_03, day_04, day_05, day_06, day_07};
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
    fn new(input: &str) -> Self;

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

pub const DAYS: u8 = 7;

pub fn run(day: u8, input: &str) -> (String, String) {
    match day {
        1 => run_day::<day_01::Data>(input),
        2 => run_day::<day_02::Data>(input),
        3 => run_day::<day_03::Data>(input),
        4 => run_day::<day_04::Data>(input),
        5 => run_day::<day_05::Data>(input),
        6 => run_day::<day_06::Data>(input),
        7 => run_day::<day_07::Data>(input),
        _ => todo!("not implemented yet"),
    }
}

fn run_day<T: AoCData>(input: &str) -> (String, String) {
    let data = T::new(input);
    data.solve()
}
