use std::fmt::Display;

use aoc_core::AoCError;
use itertools::Itertools;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data {
    start: i32,
    end: i32,
}

fn meets_part_1_criteria(num: i32) -> bool {
    let num = num.to_string();
    let mut increasing = true;
    let mut has_double = false;
    for (c1, c2) in num.chars().tuple_windows() {
        if c1 == c2 {
            has_double = true;
        }
        if c2 < c1 {
            increasing = false;
        }
    }

    has_double && increasing
}

fn meets_part_2_criteria(num: i32) -> bool {
    let num = num.to_string();
    let mut increasing = true;
    let mut has_double = false;
    let mut combo = 0;
    for (c1, c2) in num.chars().tuple_windows() {
        if c2.to_digit(10) < c1.to_digit(10) {
            increasing = false;
            break;
        }
        if c1 == c2 {
            combo += 1;
            continue;
        } else {
            if combo == 1 {
                has_double = true;
            }
            combo = 0;
        }
    }
    // check if the double was at the end of the number
    if combo == 1 {
        has_double = true
    }

    has_double && increasing
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let (start, end) = input
            .split_once("-")
            .ok_or(AoCError::Parsing)?;
        let start = start.parse()?;
        let end = end.parse()?;
        Ok(Self { start, end })
    }

    // TODO: use math instead of string logic
    fn part_1(&self) -> AoCResult<impl Display> {
        let mut count = 0;
        for num in self.start..=self.end {
            if meets_part_1_criteria(num) {
                count += 1;
            }
        }
        Ok(count)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut count = 0;
        for num in self.start..=self.end {
            if meets_part_2_criteria(num) {
                count += 1;
            }
        }
        Ok(count)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        assert!(meets_part_1_criteria(111111));
        assert!(!meets_part_1_criteria(223450));
        assert!(!meets_part_1_criteria(123789));
    }

    #[test]
    fn part_2() {
        assert!(meets_part_2_criteria(112233));
        assert!(!meets_part_2_criteria(123444));
        assert!(meets_part_1_criteria(111122));
    }
}
