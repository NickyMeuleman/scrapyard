use std::fmt::Display;

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<(u64, u64)>);

fn part_2_helper(mut ranges: Vec<(u64, u64)>, largest_possible: u64) -> u64 {
    let mut curr = 0;
    let mut count = 0;
    ranges.sort_unstable();
    for (start, end) in ranges.iter() {
        if curr < *start {
            count = count + start - curr;
            curr = end + 1;
            continue;
        }
        if curr >= *start && curr <= *end {
            curr = end + 1;
        }
    }

    if curr <= largest_possible {
        // largest_possible is inclusive, +1
        count += largest_possible - curr + 1;
    }

    count
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut ranges = Vec::new();
        for line in input.trim().lines() {
            let (start, end) = line
                .split_once('-')
                .ok_or(AoCError::Parsing)?;
            let start = start.parse()?;
            let end = end.parse()?;
            ranges.push((start, end));
        }
        Ok(Self(ranges))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut curr = 0;
        let mut ranges = self.0.clone();
        ranges.sort_unstable();
        for (start, end) in ranges.iter() {
            if curr < *start {
                return Ok(curr);
            }
            if curr >= *start && curr <= *end {
                curr = end + 1;
            }
        }
        Err(AoCError::Solving)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        Ok(part_2_helper(self.0.clone(), 4_294_967_295))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "5-8
0-2
4-7";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "3");
    }

    #[test]
    fn part_2() {
        let input = "5-8
0-2
4-7";
        let data = Data::try_new(input).unwrap();
        assert_eq!(part_2_helper(data.0, 9), 2);
    }
}
