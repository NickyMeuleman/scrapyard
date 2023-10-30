use std::fmt::Display;

use aoc_core::AoCError;
use itertools::Itertools;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

#[derive(Debug, Clone)]
pub struct Data {
    stacks: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let (left, instructions_str) = input
            .split_once("\n\n")
            .ok_or(AoCError::new("Parsing Failed"))?;
        let (stacks_str, platforms) = left
            .rsplit_once('\n')
            .ok_or(AoCError::new("Parsing Failed"))?;

        // parse crates
        let num_stacks = platforms
            .split_whitespace()
            .last()
            .ok_or(AoCError::new("Parsing Failed"))?
            .parse()?;

        let mut stacks = vec![Vec::new(); num_stacks];
        for line in stacks_str.lines().rev() {
            for (idx, mut chunk) in line
                .chars()
                .chunks(4)
                .into_iter()
                .enumerate()
            {
                let second = chunk
                    .nth(1)
                    .ok_or(AoCError::new("Parsing Failed"))?;
                if second.is_alphabetic() {
                    stacks[idx].push(second);
                }
            }
        }

        // parse instructions
        let mut instructions = Vec::new();
        for line in instructions_str.lines() {
            let rest = line
                .strip_prefix("move ")
                .ok_or(AoCError::new("Parsing Failed"))?;
            let (amount, rest) = rest
                .split_once(" from ")
                .ok_or(AoCError::new("Parsing Failed"))?;
            let (from, to) = rest
                .split_once(" to ")
                .ok_or(AoCError::new("Parsing Failed"))?;
            let instruction = Instruction {
                amount: amount.parse()?,
                from: from.parse::<usize>()? - 1,
                to: to.parse::<usize>()? - 1,
            };
            instructions.push(instruction);
        }

        Ok(Self {
            stacks,
            instructions,
        })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut stacks = self.stacks.clone();
        let instructions = &self.instructions;

        for Instruction { amount, from, to } in instructions {
            for _ in 0..*amount {
                if let Some(removed) = stacks[*from].pop() {
                    stacks[*to].push(removed);
                }
            }
        }

        let result: String = stacks
            .into_iter()
            .filter_map(|stack| stack.into_iter().last())
            .collect();

        Ok(result)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut stacks = self.stacks.clone();
        let instructions = &self.instructions;

        for Instruction { amount, from, to } in instructions {
            let from_stack_len = stacks[*from].len();
            let removed = stacks[*from].split_off(from_stack_len - amount);
            stacks[*to].extend(removed);
        }

        let result: String = stacks
            .into_iter()
            .filter_map(|stack| stack.into_iter().last())
            .collect();

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn part_2() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "MCD");
    }
}
