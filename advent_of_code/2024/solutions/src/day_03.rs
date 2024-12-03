use aoc_core::Solution;

use crate::{AoCData, AoCResult};
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let memory = self.0.as_bytes();
        let mut idx = 0;
        let mut sum = 0;

        while idx < memory.len() {
            // go to next mul instruction
            if !memory[idx..].starts_with(b"mul(") {
                idx += 1;
                continue;
            }
            idx += 4;

            // parse num 1
            let mut num_1 = 0;
            let start_idx = idx;
            while memory[idx].is_ascii_digit() {
                num_1 *= 10;
                num_1 += (memory[idx] - b'0') as u32;
                idx += 1;
            }
            // check if number was between 1 and 3 digits
            if idx == start_idx || idx > start_idx + 3 {
                continue;
            }

            // skip ,
            if memory[idx] != b',' {
                continue;
            }
            idx += 1;

            // parse num 2
            let mut num_2 = 0;
            let start_idx = idx;
            while memory[idx].is_ascii_digit() {
                num_2 *= 10;
                num_2 += (memory[idx] - b'0') as u32;
                idx += 1;
            }
            // check if number was between 1 and 3 digits
            if idx == start_idx || idx > start_idx + 3 {
                continue;
            }

            // skip )
            if memory[idx] != b')' {
                continue;
            }
            idx += 1;

            // add product to sum
            sum += num_1 * num_2;
        }

        Ok(sum)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let memory = self.0.as_bytes();
        let mut on = true;
        let mut idx = 0;
        let mut sum = 0;

        while idx < memory.len() {
            // go to next instruction
            match &memory[idx..] {
                b if b.starts_with(b"mul(") => {
                    idx += 4;
                    if !on {
                        continue;
                    }
                }
                b if b.starts_with(b"don't()") => {
                    on = false;
                    idx += 7;
                    continue;
                }
                b if b.starts_with(b"do()") => {
                    on = true;
                    idx += 4;
                    continue;
                }
                _ => {
                    idx += 1;
                    continue;
                }
            }
            // parse num 1
            let mut num_1 = 0;
            let start_idx = idx;
            while memory[idx].is_ascii_digit() {
                num_1 *= 10;
                num_1 += (memory[idx] - b'0') as u32;
                idx += 1;
            }
            // check if number was between 1 and 3 digits
            if idx > start_idx + 3 {
                continue;
            }

            // skip ,
            if memory[idx] != b',' {
                continue;
            }
            idx += 1;

            // parse num 2
            let mut num_2 = 0;
            let start_idx = idx;
            while memory[idx].is_ascii_digit() {
                num_2 *= 10;
                num_2 += (memory[idx] - b'0') as u32;
                idx += 1;
            }
            // check if number was between 1 and 3 digits
            if idx > start_idx + 3 {
                continue;
            }

            // skip )
            if memory[idx] != b')' {
                continue;
            }
            idx += 1;

            // add product to sum
            sum += num_1 * num_2;
        }

        Ok(sum)
    }

    fn solve(self) -> AoCResult<aoc_core::Solution>
    where
        Self: Sized,
    {
        let memory = self.0.as_bytes();
        let mut on = true;
        let mut idx = 0;
        let mut sum_1 = 0;
        let mut sum_2 = 0;

        while idx < memory.len() {
            // go to next instruction
            match &memory[idx..] {
                b if b.starts_with(b"mul(") => {
                    idx += 4;
                }
                b if b.starts_with(b"don't()") => {
                    on = false;
                    idx += 7;
                    continue;
                }
                b if b.starts_with(b"do()") => {
                    on = true;
                    idx += 4;
                    continue;
                }
                _ => {
                    idx += 1;
                    continue;
                }
            }

            // parse num 1
            let mut num_1 = 0;
            let start_idx = idx;
            while memory[idx].is_ascii_digit() {
                num_1 *= 10;
                num_1 += (memory[idx] - b'0') as u32;
                idx += 1;
            }
            // check if number was between 1 and 3 digits
            if idx > start_idx + 3 {
                continue;
            }

            // skip ,
            if memory[idx] != b',' {
                continue;
            }
            idx += 1;

            // parse num 2
            let mut num_2 = 0;
            let start_idx = idx;
            while memory[idx].is_ascii_digit() {
                num_2 *= 10;
                num_2 += (memory[idx] - b'0') as u32;
                idx += 1;
            }
            // check if number was between 1 and 3 digits
            if idx > start_idx + 3 {
                continue;
            }

            // skip )
            if memory[idx] != b')' {
                continue;
            }
            idx += 1;

            // add product to sum
            let prod = num_1 * num_2;
            sum_1 += prod;
            if on {
                sum_2 += prod
            }
        }

        Ok(Solution {
            part1: Box::new(sum_1),
            part2: Box::new(sum_2),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "161");
    }

    #[test]
    fn part_2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "48");
    }
}
