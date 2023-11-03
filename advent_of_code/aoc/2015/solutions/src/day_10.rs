use std::fmt::Display;

use aoc_core::Solution;
use itertools::Itertools;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

fn look_and_say(s: &str) -> String {
    s.chars()
        .dedup_with_count()
        .map(|(count, c)| format!("{}{}", count, c))
        .collect()
}

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        Ok((0..40)
            .fold(self.0.to_string(), |acc, _i| look_and_say(&acc))
            .len())
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        Ok((0..50)
            .fold(self.0.to_string(), |acc, _i| look_and_say(&acc))
            .len())
    }

    fn solve(self) -> AoCResult<Solution>
    where
        Self: Sized,
    {
        let mut part1 = 0;

        let part2 = (0..50)
            .fold(self.0.to_string(), |acc, i| {
                if i == 40 {
                    part1 = acc.len();
                }
                look_and_say(&acc)
            })
            .len();

        Ok(Solution {
            part1: Box::new(part1),
            part2: Box::new(part2),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn look_and_say_once() {
        assert_eq!(look_and_say("1"), "11");
        assert_eq!(look_and_say("11"), "21");
        assert_eq!(look_and_say("21"), "1211");
        assert_eq!(look_and_say("1211"), "111221");
        assert_eq!(look_and_say("111221"), "312211");
    }

    #[test]
    fn look_and_say_five() {
        let input = "1".to_string();
        let result = (0..5).fold(input, |acc, _i| look_and_say(&acc));
        assert_eq!(result, "312211".to_string());
    }
}
