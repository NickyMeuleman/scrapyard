// Blog writeup with simpler Rust code (I should handle errors here):
// https://nickymeuleman.netlify.app/blog/aoc2025-day06/

use crate::{AoCData, AoCError, AoCResult};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

fn calc(nums: &[Vec<u64>], ops: &[char]) -> AoCResult<u64> {
    nums.iter()
        .zip(ops)
        .map(|(nums, op)| match op {
            '+' => Ok(nums.iter().sum::<u64>()),
            '*' => Ok(nums.iter().product::<u64>()),
            _ => Err(AoCError::Solving),
        })
        .sum()
}

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let (top, bottom) = self
            .0
            .trim()
            .rsplit_once('\n')
            .ok_or(AoCError::Parsing)?;

        let num_problems = top
            .lines()
            .next()
            .ok_or(AoCError::Parsing)?
            .split_whitespace()
            .count();
        if num_problems == 0 {
            return Err(AoCError::Parsing);
        }
        let mut nums = vec![Vec::new(); num_problems];

        for line in top.lines() {
            for (idx, s) in line.split_whitespace().enumerate() {
                nums.get_mut(idx)
                    .ok_or(AoCError::Solving)?
                    .push(s.parse()?);
            }
        }

        let ops: Vec<char> = bottom
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();

        calc(&nums, &ops)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let (top, bottom) = self
            .0
            .trim()
            .rsplit_once('\n')
            .ok_or(AoCError::Parsing)?;

        let lines: Vec<&str> = top.lines().collect();
        let cols = lines
            .first()
            .ok_or(AoCError::Parsing)?
            .len();

        let mut problems = Vec::new();
        let mut curr = Vec::new();

        for col in 0..cols {
            let num = lines
                .iter()
                .filter_map(|line| line.chars().nth(col)?.to_digit(10))
                .reduce(|acc, d| acc * 10 + d);

            match num {
                Some(n) => curr.push(n as u64),
                None => {
                    // completely empty column marks the end of the current problem
                    problems.push(curr);
                    curr = Vec::new();
                }
            }
        }
        problems.push(curr);

        let ops: Vec<char> = bottom
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();

        calc(&problems, &ops)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "4277556");
    }

    #[test]
    fn part_2() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "3263827");
    }
}
