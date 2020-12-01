use std::error;
use std::fs;

const TARGET: i32 = 2020;

fn main() -> Result<(), Box<dyn error::Error>> {
    let input = fs::read_to_string("./input.txt")?;
    let nums: Vec<i32> = input.lines().map(|line| line.parse().unwrap()).collect();
    println!("part one answer: {}", part_one(&nums));
    println!("part two answer: {}", part_two(&nums));
    Ok(())
}

fn find_2(nums: &Vec<i32>, target: i32) -> Option<(i32, i32)> {
    for num in nums {
        if let Some(needed) = nums.iter().find(|&&num2| num2 == target - num) {
            return Some((*num, *needed));
        }
    }
    None
}

fn find_3(nums: &Vec<i32>, target: i32) -> Option<(i32, i32, i32)> {
    for num in nums.iter() {
        if let Some(tuple) = find_2(&nums, TARGET - num) {
            let (one, two) = tuple;
            return Some((*num, one, two));
        }
    }
    None
}

fn part_one(nums: &Vec<i32>) -> i32 {
    let (one, two) = find_2(nums, TARGET).unwrap();
    one * two
}

fn part_two(nums: &Vec<i32>) -> i32 {
    let (one, two, three) = find_3(&nums, TARGET).unwrap();
    one * two * three
}

// TODO: tests, optimizations (like removing the starting number from the array in find_3)