use day_13::Data;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input_ucla_posc.txt").unwrap();
    // let input = fs::read_to_string("./input.txt").unwrap();
    let data: Data = input.parse().unwrap();
    println!("Part one answer: {}", data.clone().part_one());
    println!("Part two answer: {}", data.part_two());
}
