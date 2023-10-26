#![feature(return_position_impl_trait_in_trait)]

use std::{
    env,
    fmt::Display,
    fs, io,
    path::{Path, PathBuf},
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

pub fn get_input(day: u8, year: u32) -> io::Result<String> {
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
