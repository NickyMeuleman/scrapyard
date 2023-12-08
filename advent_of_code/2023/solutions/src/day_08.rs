use std::{collections::HashMap, fmt::Display};

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

enum Instruction {
    Left,
    Right,
}

struct Destinations<'a> {
    left: &'a str,
    right: &'a str,
}

struct Ghost<'a> {
    pos: &'a str,
    cycles: Option<u64>,
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let (instructions, map) = self
            .0
            .split_once("\n\n")
            .ok_or(AoCError::Parsing)?;
        let instructions = instructions
            .chars()
            .filter_map(|c| match c {
                // todo: figure out how to early return an Err from the part1 function while in an iterator chain and keeping the resulting iterator
                'L' => Some(Instruction::Left),
                'R' => Some(Instruction::Right),
                _ => None,
            });
        let map = map
            .lines()
            .map(|line| {
                let (source, destinations) = line
                    .split_once(" = ")
                    .ok_or(AoCError::Parsing)?;
                let destinations = destinations
                    .strip_prefix("(")
                    .ok_or(AoCError::Parsing)?
                    .strip_suffix(")")
                    .ok_or(AoCError::Parsing)?;
                let destinations = destinations
                    .split_once(", ")
                    .ok_or(AoCError::Parsing)?;
                Ok((
                    source,
                    Destinations {
                        left: destinations.0,
                        right: destinations.1,
                    },
                ))
            })
            .collect::<AoCResult<HashMap<&str, Destinations>>>()?;

        let mut instructions = instructions.cycle();
        let mut steps = 0;
        let mut curr = "AAA";

        while curr != "ZZZ" {
            let ins = instructions
                .next()
                .ok_or(AoCError::Solving)?;
            let Destinations { left, right } = map.get(curr).ok_or(AoCError::Solving)?;
            curr = match ins {
                Instruction::Left => left,
                Instruction::Right => right,
            };
            steps += 1;
        }

        Ok(steps)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let (instructions, map) = self
            .0
            .split_once("\n\n")
            .ok_or(AoCError::Parsing)?;
        let instructions: Vec<Instruction> = instructions
            .chars()
            .filter_map(|c| match c {
                // todo: figure out how to early return an Err from the part1 function while in an iterator chain and keeping the resulting iterator
                'L' => Some(Instruction::Left),
                'R' => Some(Instruction::Right),
                _ => None,
            })
            .collect();
        let map = map
            .lines()
            .map(|line| {
                let (source, destinations) = line
                    .split_once(" = ")
                    .ok_or(AoCError::Parsing)?;
                let destinations = destinations
                    .strip_prefix("(")
                    .ok_or(AoCError::Parsing)?
                    .strip_suffix(")")
                    .ok_or(AoCError::Parsing)?;
                let destinations = destinations
                    .split_once(", ")
                    .ok_or(AoCError::Parsing)?;
                Ok((
                    source,
                    Destinations {
                        left: destinations.0,
                        right: destinations.1,
                    },
                ))
            })
            .collect::<AoCResult<HashMap<&str, Destinations>>>()?;

        let mut cycle_count = 0;
        let mut ghosts: Vec<Ghost> = map
            .keys()
            // start from all positions ending in 'A'
            .filter(|source| source.ends_with('A'))
            // map every location to a location with a cycle count
            .map(|pos| Ghost { pos, cycles: None })
            .collect();

        while ghosts
            .iter()
            .any(|ghost| ghost.cycles.is_none())
        {
            // Do a full cycle of instructions
            for ins in &instructions {
                for Ghost { pos, cycles } in ghosts.iter_mut() {
                    if cycles.is_some() {
                        // this loop already has a known cycle length, no need to simulate further
                        continue;
                    }
                    let Destinations { left, right } = map.get(pos).ok_or(AoCError::Solving)?;
                    *pos = match ins {
                        Instruction::Left => left,
                        Instruction::Right => right,
                    };
                }
            }
            cycle_count += 1;

            // after a full cycle of instructions, save any found cycles (ghosts that arrived at a destination)
            for Ghost { pos, cycles: cycle } in ghosts.iter_mut() {
                if cycle.is_some() {
                    // already has a known cycle, no need to update
                    continue;
                }
                if pos.ends_with('Z') {
                    *cycle = Some(cycle_count);
                }
            }
        }

        let min_shared_cycles = ghosts
            .into_iter()
            .filter_map(|ghost| ghost.cycles)
            .fold(1, |acc, item| lcm(acc, item));

        Ok(min_shared_cycles * instructions.len() as u64)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "2");
    }

    #[test]
    fn part_2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "6");
    }
}
