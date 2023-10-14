#![feature(return_position_impl_trait_in_trait)]

pub mod day_01;
pub mod day_02;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use std::{fmt::Display, fs, io, time::Instant};
use wasm_bindgen::prelude::*;

pub const DAYS: u8 = 2;

// https://github.com/rustwasm/wasm-bindgen/issues/1775
// https://stackoverflow.com/questions/68243940/rust-wasm-bindgen-struct-with-string
// Strings on a struct can not be public
// skip the fields with a wasm_bindgen macro, but implement a getter for them so you can access them in JS.
// a weird disable only to later enable, I know.
// You can do without the wasm_bindgen skip on the fields, but since my other Rust code accesses them, I need them to be public.
pub enum Answer {
    Part(String),
    Both(Solution),
}
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

pub enum Part {
    One,
    Two,
    Both,
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

pub fn print_part(day: u8, part: &Part) {
    let mut ctx = ClipboardContext::new().unwrap();
    println!(
        "Day {}, {}",
        day,
        match part {
            Part::One => "part 1",
            Part::Two => "part 2",
            Part::Both => "both parts",
        }
    );
    println!("{:=<20}", "=");
    let input = get_input(day, false).unwrap();
    let now = Instant::now();
    let result = solve_part(day, &input, &part);
    let elapsed = now.elapsed();
    println!("Runtime:");
    println!("{:?}", elapsed);
    match result {
        Answer::Part(result) => {
            println!("Answer:");
            println!("{result}");
            // ah, a flaky thing that sometimes works, yay
            ctx.set_contents(result).unwrap();
        }
        Answer::Both(solution) => {
            println!("Part 1:");
            println!("{}", solution.part1);
            println!("Part 2:");
            println!("{}", solution.part2);
        }
    }
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

pub fn part_helper<'a, T: AoCData<'a>>(_day: u8, input: &'a str, part: &Part) -> Answer {
    if let Some(data) = T::try_new(input) {
        return match part {
            Part::One => Answer::Part(data.part_1().to_string()),
            Part::Two => Answer::Part(data.part_2().to_string()),
            Part::Both => Answer::Both(data.solve()),
        };
    }
    panic!("At the disco")
}

fn solve_part(day: u8, input: &str, part: &Part) -> Answer {
    match day {
        1 => part_helper::<day_01::Data>(day, input, part),
        2 => part_helper::<day_02::Data>(day, input, part),
        n => panic!("Trying to solve an invalid day, found day {n}"),
    }
}

#[wasm_bindgen]
pub async fn solve(day: u8, input: String) -> Result<Solution, JsError> {
    solve_day(day, input)
}
