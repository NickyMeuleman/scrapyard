// https://github.com/Strackeror/aoc_2021_rust/blob/main/src/day23.rs

use day_23::Data;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let data: Data = input.parse().unwrap();
    println!("Part one answer: {}", data.clone().part_one());
    println!("Part two answer: {}", data.part_two());
}