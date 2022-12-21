use crate::AoCData;
use itertools::Itertools;

#[derive(Debug)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}
pub struct Data {
    stacks: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> Option<Self> {
        let (left, instructions_str) = input.split_once("\n\n")?;
        let (stacks_str, platforms) = left.rsplit_once('\n')?;

        // parse crates
        let num_stacks = platforms.split_whitespace().last()?.parse().ok()?;

        let mut stacks = vec![Vec::new(); num_stacks];
        for line in stacks_str.lines().rev() {
            for (idx, mut chunk) in line.chars().chunks(4).into_iter().enumerate() {
                let second = chunk.nth(1)?;
                if second.is_alphabetic() {
                    stacks[idx].push(second);
                }
            }
        }

        // parse instructions
        let mut instructions = Vec::new();
        for line in instructions_str.lines() {
            let rest = line.strip_prefix("move ")?;
            let (amount, rest) = rest.split_once(" from ")?;
            let (from, to) = rest.split_once(" to ")?;
            let instruction = Instruction {
                amount: amount.parse().ok()?,
                from: from.parse::<usize>().ok()? - 1,
                to: to.parse::<usize>().ok()? - 1,
            };
            instructions.push(instruction);
        }

        Some(Self {
            stacks,
            instructions,
        })
    }

    fn part_1(&self) -> String {
        let mut stacks = self.stacks.clone();
        let instructions = &self.instructions;

        for Instruction { amount, from, to } in instructions {
            for _ in 0..*amount {
                if let Some(removed) = stacks[*from].pop() {
                    stacks[*to].push(removed);
                }
            }
        }

        stacks
            .into_iter()
            .filter_map(|stack| stack.into_iter().last())
            .collect()
    }

    fn part_2(&self) -> String {
        let mut stacks = self.stacks.clone();
        let instructions = &self.instructions;

        for Instruction { amount, from, to } in instructions {
            let from_stack_len = stacks[*from].len();
            let removed = stacks[*from].split_off(from_stack_len - amount);
            stacks[*to].extend(removed);
        }

        stacks
            .into_iter()
            .filter_map(|stack| stack.into_iter().last())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(5);
        let data = Data::try_new(&input).unwrap();
        assert_eq!(data.part_1(), "CMZ");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(5);
        let data = Data::try_new(&input).unwrap();
        assert_eq!(data.part_2(), "MCD");
    }
}
