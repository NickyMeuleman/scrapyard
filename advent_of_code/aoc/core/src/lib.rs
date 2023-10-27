#![feature(return_position_impl_trait_in_trait)]

use std::{
    env,
    fmt::Display,
    fs, io,
    path::{Path, PathBuf},
    time::Instant,
};

pub const DAYS: u8 = 25;

pub enum Answer {
    Part(String),
    Both(Solution),
}

#[derive(Debug)]
pub enum Part {
    One = 1,
    Two = 2,
    Both = 3,
}

// https://github.com/rustwasm/wasm-bindgen/issues/1775
// https://stackoverflow.com/questions/68243940/rust-wasm-bindgen-struct-with-string
// Strings on a struct can not be public
// skip the fields with a wasm_bindgen macro, but implement a getter for them so you can access them in JS.
// a weird disable only to later enable, I know.
// You can do without the wasm_bindgen skip on the fields, but since my other Rust code accesses them, I need them to be public.

pub struct Solution {
    pub part1: Box<dyn Display>,
    pub part2: Box<dyn Display>,
}

pub trait AoCDay<'a> {
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
        // have to make sure results that come back from the part functions live long enough,
        // because those might be borrowed things that impl Display
        Solution {
            part1: Box::new(self.part_1().to_string()),
            part2: Box::new(self.part_2().to_string()),
        }
    }
}

pub fn get_input(year: u32, day: u8) -> io::Result<String> {
    let mut input_path = workspace_dir();
    input_path.push(year.to_string());
    input_path.push("inputs");
    input_path.push(format!("day{:02}.txt", day));
    fs::read_to_string(input_path)
}

fn workspace_dir() -> PathBuf {
    let output = std::process::Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .unwrap()
        .stdout;
    let cargo_path = Path::new(
        std::str::from_utf8(&output)
            .unwrap()
            .trim(),
    );
    cargo_path
        .parent()
        .unwrap()
        .to_path_buf()
}

pub fn print_part(
    year: u32,
    day: u8,
    part: &Part,
    part_solver: impl Fn(u8, &Part, &str) -> Result<Answer, String>,
) {
    println!(
        "Year {}, Day {}, {}",
        year,
        day,
        match part {
            Part::One => "part 1",
            Part::Two => "part 2",
            Part::Both => "both parts",
        }
    );
    println!("{:=<20}", "=");
    let input = get_input(year, day).unwrap();
    let now = Instant::now();
    let result = part_solver(day, &part, &input);
    let elapsed = now.elapsed();
    println!("Runtime:");
    println!("{:?}", elapsed);
    match result.unwrap_or(Answer::Part("No result!".to_string())) {
        Answer::Part(result) => {
            println!("Answer:");
            println!("{result}");
        }
        Answer::Both(solution) => {
            println!("Part 1:");
            println!("{}", solution.part1);
            println!("Part 2:");
            println!("{}", solution.part2);
        }
    }
}

pub fn part_helper<'a, T: AoCDay<'a>>(
    day: u8,
    part: &Part,
    input: &'a str,
) -> Result<Answer, String> {
    if let Some(data) = T::try_new(input) {
        let answer = match part {
            Part::One => Answer::Part(data.part_1().to_string()),
            Part::Two => Answer::Part(data.part_2().to_string()),
            Part::Both => Answer::Both(data.solve()),
        };
        Ok(answer)
    } else {
        Err(format!("Failed to parse day {day}"))
    }
}
