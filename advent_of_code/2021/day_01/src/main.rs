use std::cmp::Ordering;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let data = parse(&input);
    println!("Part one answer: {}", part_one(data.clone()));
    println!("Part two answer: {}", part_two(data));
}

fn parse(input: &str) -> Vec<u32> {
    input.lines().filter_map(|s| s.parse().ok()).collect()
}

fn part_one(data: Vec<u32>) -> usize {
    data.windows(2)
        .filter(|window| window[0].cmp(&window[1]) == Ordering::Less)
        .count()
}

fn part_two(data: Vec<u32>) -> usize {
    // turn data into sums of three-measurement windows
    data.windows(3)
        .map(|window| window.iter().sum())
        .collect::<Vec<u32>>()
        // count the amount of times a three-measurement sum increases
        .windows(2)
        .filter(|window| window[0].cmp(&window[1]) == Ordering::Less)
        .count()
}
