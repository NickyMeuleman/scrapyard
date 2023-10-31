use std::fmt::Display;

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data {
    instructions: Vec<Instruction>,
}

#[derive(Debug, Clone)]
enum Instruction {
    Up(u8),
    Down(u8),
    Forward(u8),
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let instructions = input
            .trim()
            .lines()
            .map(|line| {
                let parts: Vec<_> = line.split_whitespace().collect();
                let amount = parts
                    .get(1)
                    .ok_or(AoCError::Parsing)?
                    .parse()?;

                match *parts.get(0).ok_or(AoCError::Parsing)? {
                    "up" => Ok(Instruction::Up(amount)),
                    "down" => Ok(Instruction::Down(amount)),
                    "forward" => Ok(Instruction::Forward(amount)),
                    _ => Err(AoCError::Parsing),
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { instructions })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let result: u32 = self
            .instructions
            .iter()
            .fold([0; 2], |[mut horizontal, mut depth], instruction| {
                match instruction {
                    Instruction::Up(amount) => depth -= u32::from(*amount),
                    Instruction::Down(amount) => depth += u32::from(*amount),
                    Instruction::Forward(amount) => horizontal += u32::from(*amount),
                }
                [horizontal, depth]
            })
            .iter()
            .product();

        Ok(result)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let result: u32 = self
            .instructions
            .iter()
            .fold(
                [0; 3],
                |[mut horizontal, mut depth, mut aim], instruction| {
                    match instruction {
                        Instruction::Up(amount) => aim -= u32::from(*amount),
                        Instruction::Down(amount) => aim += u32::from(*amount),
                        Instruction::Forward(amount) => {
                            horizontal += u32::from(*amount);
                            depth += aim * u32::from(*amount);
                        }
                    }
                    [horizontal, depth, aim]
                },
            )
            .iter()
            // don't include aim in the final calculation
            .take(2)
            .product();

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "150");
    }

    #[test]
    fn part_2() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "900");
    }
}
