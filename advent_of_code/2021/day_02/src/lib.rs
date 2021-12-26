use std::{convert::Infallible, str::FromStr};

pub struct Data {
    instructions: Vec<Instruction>,
}

impl Data {
    pub fn part_one(&self) -> u32 {
        self.instructions
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
            .product()
    }

    pub fn part_two(&self) -> u32 {
        self.instructions
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
            .product()
    }
}

impl FromStr for Data {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            instructions: input
                .trim()
                .lines()
                .map(|line| {
                    let parts: Vec<_> = line.split_whitespace().collect();
                    let amount = parts[1].parse().unwrap();

                    match parts[0] {
                        "up" => Instruction::Up(amount),
                        "down" => Instruction::Down(amount),
                        "forward" => Instruction::Forward(amount),
                        _ => unreachable!("invalid input data"),
                    }
                })
                .collect(),
        })
    }
}

enum Instruction {
    Up(u8),
    Down(u8),
    Forward(u8),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_one(), 150);
    }

    #[test]
    fn part_two_example() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_two(), 900);
    }
}
