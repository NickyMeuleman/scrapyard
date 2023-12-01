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
        self.0
            .lines()
            .map(|line| {
                let Some(first) = line
                    .chars()
                    .find_map(|c| c.to_digit(10))
                else {
                    return Err(AoCError::Solving);
                };
                let Some(last) = line
                    .chars()
                    .rev()
                    .find_map(|c| c.to_digit(10))
                else {
                    return Err(AoCError::Solving);
                };

                Ok(10 * first + last)
            })
            .sum::<AoCResult<u32>>()
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut nums = HashMap::new();
        nums.insert("1", 1);
        nums.insert("2", 2);
        nums.insert("3", 3);
        nums.insert("4", 4);
        nums.insert("5", 5);
        nums.insert("6", 6);
        nums.insert("7", 7);
        nums.insert("8", 8);
        nums.insert("9", 9);
        nums.insert("one", 1);
        nums.insert("two", 2);
        nums.insert("three", 3);
        nums.insert("four", 4);
        nums.insert("five", 5);
        nums.insert("six", 6);
        nums.insert("seven", 7);
        nums.insert("eight", 8);
        nums.insert("nine", 9);

        let mut sum = 0;
        for line in self.0.lines() {
            let mut forwards = line;
            let mut backwards = line;

            let first = 'outer: loop {
                for (prefix, num) in nums.iter() {
                    if forwards.starts_with(prefix) {
                        break 'outer num;
                    }
                }
                forwards = forwards
                    .get(1..)
                    .ok_or(AoCError::Solving)?;
            };

            let last = 'outer: loop {
                for (suffix, num) in nums.iter() {
                    if backwards.ends_with(suffix) {
                        break 'outer num;
                    }
                }
                backwards = backwards
                    .get(..backwards.len() - 1)
                    .ok_or(AoCError::Solving)?;
            };

            let num = first * 10 + last;
            sum += num;
        }

        Ok(sum)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "142");
    }

    #[test]
    fn part_2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "281");
    }
}
