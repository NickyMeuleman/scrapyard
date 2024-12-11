// Blog writeup with simpler Rust code (I should handle errors here):
// https://nickymeuleman.netlify.app/blog/aoc2024-day11/

use aoc_core::Solution;

use crate::{AoCData, AoCResult};
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

fn blink(stones: &HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new = HashMap::new();
    for (stone, amount) in stones {
        if *stone == 0 {
            *new.entry(1).or_default() += amount;
        } else {
            let digits = stone.ilog10() + 1;
            if digits % 2 == 0 {
                let magnitude = 10u64.pow(digits / 2);
                *new.entry(stone % magnitude)
                    .or_default() += amount;
                *new.entry(stone / magnitude)
                    .or_default() += amount;
            } else {
                *new.entry(stone * 2024).or_default() += amount;
            }
        }
    }
    new
}

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut stones: HashMap<u64, u64> = HashMap::new();
        for num in self.0.split_ascii_whitespace() {
            let num = num.parse().unwrap();
            *stones.entry(num).or_default() += 1;
        }

        for _ in 0..25 {
            stones = blink(&stones);
        }

        Ok(stones.values().sum::<u64>())
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut stones: HashMap<u64, u64> = HashMap::new();
        for num in self.0.split_ascii_whitespace() {
            let num = num.parse().unwrap();
            *stones.entry(num).or_default() += 1;
        }

        for _ in 0..75 {
            stones = blink(&stones);
        }

        Ok(stones.values().sum::<u64>())
    }

    fn solve(self) -> AoCResult<aoc_core::Solution>
    where
        Self: Sized,
    {
        let mut stones: HashMap<u64, u64> = HashMap::new();
        for num in self.0.split_ascii_whitespace() {
            let num = num.parse().unwrap();
            *stones.entry(num).or_default() += 1;
        }

        let mut p1 = 0;
        for i in 0..75 {
            if i == 25 {
                p1 = stones.values().sum();
            }
            stones = blink(&stones);
        }

        Ok(Solution {
            part1: Box::new(p1),
            part2: Box::new(stones.values().sum::<u64>()),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "125 17";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "55312");
    }
}
