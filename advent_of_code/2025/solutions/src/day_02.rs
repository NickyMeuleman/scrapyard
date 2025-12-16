// Blog writeup with simpler Rust code (I should handle errors here):
// https://nickymeuleman.netlify.app/blog/aoc2025-day02/

use crate::{AoCData, AoCError, AoCResult, Solution};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

fn has_repetition(s: &str, pattern_len: usize) -> bool {
    // check if number is perfectly divisible
    if !s.len().is_multiple_of(pattern_len) {
        return false;
    }
    let pattern = &s[..pattern_len];
    // check if any block matches the pattern
    (pattern_len..s.len())
        .step_by(pattern_len)
        .all(|i| &s[i..i + pattern_len] == pattern)
}

fn parse_range(s: &str) -> AoCResult<(i64, i64)> {
    let (a, b) = s
        .split_once('-')
        .ok_or(AoCError::Parsing)?;
    Ok((a.parse()?, b.parse()?))
}

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input.trim()))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut sum = 0;
        for range in self.0.split(',') {
            let (start, end) = parse_range(range)?;
            for num in start..=end {
                let s = num.to_string();
                if s.len() % 2 != 0 {
                    continue;
                }
                let (left, right) = s.split_at(s.len() / 2);
                if left == right {
                    sum += num;
                }
            }
        }
        Ok(sum)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut sum = 0;

        for range in self.0.split(',') {
            let (start, end) = parse_range(range)?;

            for num in start..=end {
                let s = num.to_string();

                for pattern_len in 1..=s.len() / 2 {
                    if has_repetition(&s, pattern_len) {
                        sum += num;
                        // avoid double counting a number, break once a pattern repeats
                        break;
                    }
                }
            }
        }

        Ok(sum)
    }

    fn solve(self) -> AoCResult<aoc_core::Solution>
    where
        Self: Sized,
    {
        let (p1, p2) = self
            .0
            .split(',')
            .map(|range| -> AoCResult<(i64, i64)> {
                let (start, end) = parse_range(range)?;
                let mut sum1 = 0;
                let mut sum2 = 0;

                for num in start..=end {
                    let s = num.to_string();
                    let len = s.len();

                    if len.is_multiple_of(2) && has_repetition(&s, len / 2) {
                        sum1 += num;
                    }
                    if (1..=len / 2).any(|pat_len| has_repetition(&s, pat_len)) {
                        sum2 += num;
                    }
                }

                Ok((sum1, sum2))
            })
            .try_fold((0, 0), |(acc1, acc2), item| -> AoCResult<(i64, i64)> {
                let (sum1, sum2) = item?;
                Ok((acc1 + sum1, acc2 + sum2))
            })?;

        Ok(Solution {
            part1: Box::new(p1),
            part2: Box::new(p2),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "1227775554");
    }

    #[test]
    fn part_2() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "4174379265");
    }
}
