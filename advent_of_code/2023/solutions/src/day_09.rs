use std::fmt::Display;

use aoc_core::AoCError;
use itertools::Itertools;

use crate::{AoCData, AoCResult};

// commented out helper functions for the recursive versions that are also commented out

// fn differences(nums: &[i32]) -> Vec<i32> {
//     nums.iter()
//         .tuple_windows()
//         .map(|(left, right)| right - left)
//         .collect()
// }

// fn next_num(nums: &[i32]) -> i32 {
//     if nums.iter().all(|&n| n == 0) {
//         return 0;
//     }
//     let differences: Vec<i32> = differences(nums);
//     next_num(&differences) + nums.iter().last().unwrap()
// }

// fn prev_num(nums: &[i32]) -> i32 {
//     if nums.iter().all(|&n| n == 0) {
//         return 0;
//     }
//     let differences = differences(nums);
//     nums[0] - prev_num(&differences)
// }

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    // [447.49 µs 449.62 µs 451.93 µs]
    // fn part_1(&self) -> AoCResult<impl Display> {
    //     let sum: i32 = self
    //         .0
    //         .lines()
    //         .map(|line| {
    //             let nums: Vec<i32> = line
    //                 .split_whitespace()
    //                 .filter_map(|s| s.parse().ok())
    //                 .collect();
    //             next_num(&nums)
    //         })
    //         .sum();
    //     Ok(sum)
    // }

    // much faster than the recursive solution
    // without error handling: [146.38 µs 146.96 µs 147.61 µs]
    // with error handling: [193.49 µs 193.91 µs 194.31 µs]
    fn part_1(&self) -> AoCResult<impl Display> {
        let mut nums: Vec<i32> = Vec::new();
        let mut differences: Vec<i32> = Vec::new();
        let mut edge: Vec<i32> = Vec::new();

        self.0
            .lines()
            .map(|line| {
                nums.clear();
                edge.clear();

                for num in line.split_ascii_whitespace() {
                    nums.push(
                        num.parse()
                            .map_err(|_| AoCError::Parsing)?,
                    );
                }

                loop {
                    differences.clear();

                    for (left, right) in nums.iter().tuple_windows() {
                        differences.push(right - left);
                    }
                    edge.push(*nums.last().ok_or(AoCError::Solving)?);
                    if differences.iter().all(|&x| x == 0) {
                        let sum: i32 = edge.iter().sum();
                        break Ok(sum);
                    }

                    std::mem::swap(&mut nums, &mut differences);
                }
            })
            .sum::<AoCResult<i32>>()
    }

    // [437.46 µs 438.67 µs 439.79 µs]
    // fn part_2(&self) -> AoCResult<impl Display> {
    //     let sum: i32 = self
    //         .0
    //         .lines()
    //         .map(|line| {
    //             let nums: Vec<i32> = line
    //                 .split_whitespace()
    //                 .filter_map(|s| s.parse().ok())
    //                 .collect();
    //             prev_num(&nums)
    //         })
    //         .sum();
    //     Ok(sum)
    // }

    // much faster than the recursive solution
    // without error handling: [171.26 µs 171.58 µs 171.88 µs]
    // with error handling: [188.26 µs 188.83 µs 189.38 µs]
    fn part_2(&self) -> AoCResult<impl Display> {
        let mut nums: Vec<i32> = Vec::new();
        let mut differences: Vec<i32> = Vec::new();
        let mut edge: Vec<i32> = Vec::new();

        self.0
            .lines()
            .map(|line| {
                nums.clear();
                edge.clear();

                for num in line.split_ascii_whitespace() {
                    nums.push(
                        num.parse()
                            .map_err(|_| AoCError::Parsing)?,
                    );
                }

                loop {
                    differences.clear();

                    for (left, right) in nums.iter().tuple_windows() {
                        differences.push(left - right);
                    }
                    edge.push(*nums.first().ok_or(AoCError::Solving)?);
                    if differences.iter().all(|&x| x == 0) {
                        let sum: i32 = edge.iter().sum();
                        break Ok(sum);
                    }

                    std::mem::swap(&mut nums, &mut differences);
                }
            })
            .sum::<AoCResult<i32>>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "114");
    }

    #[test]
    fn part_2() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "2");
    }
}
