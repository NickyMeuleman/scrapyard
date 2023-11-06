use std::{collections::HashMap, fmt::Display};

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<Instruction>);

#[derive(Debug, Clone)]
enum Instruction {
    Spin(usize),
    Exchange(usize, usize),
    Parner(char, char),
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let result = input
            .trim()
            .split(",")
            .map(|s| {
                let (kind, rest) = s.split_at(1);
                match kind {
                    "s" => {
                        let num = rest.parse()?;
                        Ok(Instruction::Spin(num))
                    }
                    "x" => {
                        let (left, right) = rest
                            .split_once("/")
                            .ok_or(AoCError::Parsing)?;
                        let left = left.parse()?;
                        let right = right.parse()?;
                        Ok(Instruction::Exchange(left, right))
                    }
                    "p" => {
                        let (left, right) = rest
                            .split_once("/")
                            .ok_or(AoCError::Parsing)?;
                        Ok(Instruction::Parner(
                            left.chars()
                                .next()
                                .ok_or(AoCError::Parsing)?,
                            right
                                .chars()
                                .next()
                                .ok_or(AoCError::Parsing)?,
                        ))
                    }
                    _ => return Err(AoCError::Parsing),
                }
            })
            .collect::<AoCResult<_>>()?;

        Ok(Self(result))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut programs: Vec<char> = ('a'..='p').collect();
        let len = programs.len();
        for inst in self.0.iter() {
            match inst {
                Instruction::Spin(n) => programs.rotate_left(len - n),
                Instruction::Exchange(a, b) => programs.swap(*a, *b),
                Instruction::Parner(a, b) => {
                    let a = programs
                        .iter()
                        .position(|c| c == a)
                        .ok_or(AoCError::Solving)?;
                    let b = programs
                        .iter()
                        .position(|c| c == b)
                        .ok_or(AoCError::Solving)?;
                    programs.swap(a, b);
                }
            }
        }

        Ok(programs.iter().collect::<String>())
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let target = 1_000_000_000;
        let mut count = 0;
        let mut programs: Vec<char> = ('a'..='p').collect();
        let len = programs.len();
        let mut seen = HashMap::new();
        while count < target {
            for inst in self.0.iter() {
                match inst {
                    Instruction::Spin(n) => programs.rotate_left(len - n),
                    Instruction::Exchange(a, b) => programs.swap(*a, *b),
                    Instruction::Parner(a, b) => {
                        let a = programs
                            .iter()
                            .position(|c| c == a)
                            .ok_or(AoCError::Solving)?;
                        let b = programs
                            .iter()
                            .position(|c| c == b)
                            .ok_or(AoCError::Solving)?;
                        programs.swap(a, b);
                    }
                }
            }
            // look for cycle
            let key: String = programs.iter().collect();
            if let Some(prev_count) = seen.get(&key) {
                let delta = count - prev_count;
                let repeats = (target - count) / delta;
                count += repeats * delta;
            }
            // update seen map
            seen.entry(key).or_insert(count);
            count += 1;
        }

        Ok(programs.iter().collect::<String>())
    }
}
