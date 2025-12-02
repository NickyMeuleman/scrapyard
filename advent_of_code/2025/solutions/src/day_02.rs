// Blog writeup with simpler Rust code (I should handle errors here):
// https://nickymeuleman.netlify.app/blog/aoc2025-day02/

use aoc_core::Solution;

use crate::{AoCData, AoCResult};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

fn has_repetition(s: &str, pattern_len: usize) -> bool {
    // check if number is perfectly dividible
    if !s.len().is_multiple_of(pattern_len) {
        return false;
    }
    let pattern = &s[0..pattern_len];
    // check if any block matches the pattern
    (pattern_len..s.len())
        .step_by(pattern_len)
        .all(|i| &s[i..i + pattern_len] == pattern)
}

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input.trim()))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut sum = 0;
        for range in self.0.split(',') {
            let (a, b) = range.split_once('-').unwrap();
            let start: i64 = a.parse().unwrap();
            let end: i64 = b.parse().unwrap();
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
            let (a, b) = range.split_once('-').unwrap();
            let start: i64 = a.parse().unwrap();
            let end: i64 = b.parse().unwrap();
            for num in start..=end {
                let s = num.to_string();
                for pattern_len in 1..=s.len() / 2 {
                    if !s.len().is_multiple_of(pattern_len) {
                        continue;
                    }
                    let pattern = &s[0..pattern_len];
                    if (pattern_len..s.len())
                        .step_by(pattern_len)
                        .all(|start_idx| &s[start_idx..start_idx + pattern_len] == pattern)
                    {
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
            .map(|range| {
                let (a, b) = range.split_once('-').unwrap();
                let start: i64 = a.parse().unwrap();
                let end: i64 = b.parse().unwrap();
                (start..=end).fold((0, 0), |(p1, p2), num| {
                    let s = num.to_string();
                    let len = s.len();
                    let p1_add = if len.is_multiple_of(2) && has_repetition(&s, len / 2) {
                        num
                    } else {
                        0
                    };
                    let p2_add = if (1..=len / 2).any(|pat_len| has_repetition(&s, pat_len)) {
                        num
                    } else {
                        0
                    };
                    (p1 + p1_add, p2 + p2_add)
                })
            })
            .fold((0, 0), |(acc1, acc2), (sum1, sum2)| {
                (acc1 + sum1, acc2 + sum2)
            });

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
