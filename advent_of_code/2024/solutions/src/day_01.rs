use std::{collections::HashMap, fmt::Display};

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        // let mut left = vec![];
        // let mut right = vec![];
        //
        // for line in input.lines() {
        //     let nums: Vec<usize> = line
        //         .split_whitespace()
        //         .map(|s| s.parse().unwrap())
        //         .collect();
        //     left.push(nums[0]);
        //     right.push(nums[1]);
        // }
        //
        // left.sort();
        // right.sort();
        //
        // let mut sum = 0;
        // for (l, r) in left.iter().zip(right) {
        //     sum += l.abs_diff(r);
        // }
        //
        // sum

        let (mut left, mut right): (Vec<i32>, Vec<i32>) = self
            .0
            .lines()
            .map(|line| {
                let mut nums = line
                    .split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap());
                (nums.next().unwrap(), nums.next().unwrap())
            })
            .unzip();

        left.sort_unstable();
        right.sort_unstable();

        let sum: u32 = left
            .into_iter()
            .zip(right)
            .map(|(l, r)| l.abs_diff(r))
            .sum();

        Ok(sum)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        // let mut left = vec![];
        // let mut right = vec![];
        //
        // for line in input.lines() {
        //     let nums: Vec<usize> = line
        //         .split_whitespace()
        //         .map(|s| s.parse().unwrap())
        //         .collect();
        //     left.push(nums[0]);
        //     right.push(nums[1]);
        // }
        //
        // let mut sum = 0;
        //
        // for l in &left {
        //     let product = l * right.iter().filter(|&r| l == r).count();
        //     sum += product;
        // }
        //
        // sum

        let (left, right): (Vec<i32>, Vec<i32>) = self
            .0
            .lines()
            .map(|line| {
                let mut nums = line
                    .split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap());
                (nums.next().unwrap(), nums.next().unwrap())
            })
            .unzip();

        let counts: HashMap<i32, i32> = right
            .into_iter()
            .fold(HashMap::new(), |mut acc, r| {
                *acc.entry(r).or_default() += 1;
                acc
            });

        let sum: i32 = left
            .into_iter()
            .map(|l| {
                l * counts
                    .get(&l)
                    .copied()
                    .unwrap_or_default()
            })
            .sum();

        Ok(sum)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "11");
    }

    #[test]
    fn part_2() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "31");
    }
}
