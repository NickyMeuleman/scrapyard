// Blog writeup with simpler Rust code (I should handle errors here):
// https://nickymeuleman.netlify.app/blog/aoc2025-day03/

use crate::{AoCData, AoCError, AoCResult};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        self.0
            .lines()
            .map(|line| {
                let digits = line
                    .chars()
                    .map(|c| c.to_digit(10).ok_or(AoCError::Parsing))
                    .collect::<AoCResult<Vec<u32>>>()?;
                if digits.len() < 2 {
                    return Err(AoCError::Solving);
                }

                let possibilities_first = &digits[0..(digits.len() - 1)];
                let first = possibilities_first
                    .iter()
                    .max()
                    .ok_or(AoCError::Solving)?;
                let idx = possibilities_first
                    .iter()
                    .position(|d| d == first)
                    .ok_or(AoCError::Solving)?;

                let possibilities_second = &digits[idx + 1..digits.len()];
                let second = possibilities_second
                    .iter()
                    .max()
                    .ok_or(AoCError::Solving)?;

                Ok(first * 10 + second)
            })
            .sum::<AoCResult<u32>>()
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        self.0
            .lines()
            .map(|line| {
                let digits = line
                    .chars()
                    .map(|c| c.to_digit(10).ok_or(AoCError::Parsing))
                    .collect::<AoCResult<Vec<u32>>>()?;
                if digits.len() < 12 {
                    return Err(AoCError::Solving);
                }

                let mut result = 0;
                let mut start = 0;

                for reserved in (0..12).rev() {
                    // first index we cannot pick
                    let end = digits
                        .len()
                        .checked_sub(reserved)
                        .ok_or(AoCError::Solving)?;
                    if start >= end {
                        return Err(AoCError::Solving);
                    }

                    let possibilities = &digits[start..end];
                    let max = possibilities
                        .iter()
                        .copied()
                        .max()
                        .ok_or(AoCError::Solving)?;
                    // Max digit can appear more than once, pick the leftmost one to keep more options for next digit.
                    let idx = possibilities
                        .iter()
                        .position(|&d| d == max)
                        .ok_or(AoCError::Solving)?;

                    result = result * 10 + max as u64;
                    start += idx + 1;
                }

                Ok(result)
            })
            .sum::<AoCResult<u64>>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "357");
    }

    #[test]
    fn part_2() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "3121910778619");
    }
}
