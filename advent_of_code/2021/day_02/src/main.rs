use day_02::Data;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let data: Data = input.parse().unwrap();
    println!("Part one answer: {}", data.part_one());
    println!("Part two answer: {}", data.part_two());
}
