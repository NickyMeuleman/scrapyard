use std::fmt::Display;

use aoc_core::Solution;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut result = 0;
        for line in self.0.lines() {
            let literal = line.len();
            // skip opening and closing quotes
            let mut in_memory = 0;
            let line = &line[1..literal - 1];
            let mut chars = line.chars().peekable();
            while let Some(c) = chars.next() {
                in_memory += 1;
                if c == '\\' {
                    let next = chars.peek();
                    if let Some('x') = next {
                        // hexadecimal char found, skip 3 chars after backslash, \xab counts as 1 in memory
                        chars.next();
                        chars.next();
                        chars.next();
                    } else {
                        // skip char after backslash, \\ counts as 1 in memory
                        chars.next();
                    }
                }
            }
            result += literal - in_memory;
        }

        Ok(result)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        Ok(self.0.lines().fold(0, |acc, line| {
            let literal = line
                .chars()
                // map every char to the size of the literal that represents it
                // \ turns into \\ for size 2, " turns into \" for size 2, everything else is size 1
                .map(|c| match c {
                    '\\' | '"' => 2, // escape it, add a backslash
                    _ => 1,          // no escaping needed
                })
                .sum::<usize>()
                + 2; // add 2 for the starting and ending "
            acc + (literal - line.len())
        }))
    }

    /// way less readable version that does the 2 parts in one pass
    fn solve(self) -> AoCResult<Solution>
    where
        Self: Sized,
    {
        let mut result_1 = 0;
        let mut result_2 = 0;

        for line in self.0.lines() {
            let input_len = line.len();
            let mut in_memory = 0; // size in memory
            let mut literal = 6; // literal size after escaping, start at 6 to escape the surrounding "
            let line = &line[1..line.len() - 1];
            let mut chars = line.chars().peekable();

            while let Some(c) = chars.next() {
                in_memory += 1;
                literal += 1;
                if c == '\\' {
                    let next = chars.peek();
                    if let Some('x') = next {
                        // hexadecimal char found, skip 3 chars after backslash
                        chars.next();
                        chars.next();
                        chars.next();
                        literal += 4;
                    } else {
                        // skip char after backslash
                        chars.next();
                        literal += 3;
                        // note: a " always follows a backslash, so we there is no special logic for it
                    }
                }
            }

            result_1 += input_len - in_memory;
            result_2 += literal - input_len;
        }

        Ok(Solution {
            part1: Box::new(result_1),
            part2: Box::new(result_2),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = r#"""
"abc"
"aaa\"aaa"
"\x27""#;
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "12");
    }

    #[test]
    fn part_2() {
        let input = r#"""
"abc"
"aaa\"aaa"
"\x27""#;
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "19");
    }

    #[test]
    fn solve() {
        let input = r#"""
"abc"
"aaa\"aaa"
"\x27""#;
        let data = Data::try_new(input).unwrap();
        let Solution { part1, part2 } = data.solve().unwrap();
        assert_eq!(part1.to_string(), "12");
        assert_eq!(part2.to_string(), "19");
    }
}
