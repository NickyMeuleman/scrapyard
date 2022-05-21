use crate::AoCData;

pub struct Data {
    instructions: Vec<Instruction>,
}

enum Instruction {
    Up(u8),
    Down(u8),
    Forward(u8),
}

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        let instructions = input
            .trim()
            .lines()
            .map(|line| {
                let parts: Vec<_> = line.split_whitespace().collect();
                let amount = parts.get(1)?.parse().ok()?;

                match *parts.get(0)? {
                    "up" => Some(Instruction::Up(amount)),
                    "down" => Some(Instruction::Down(amount)),
                    "forward" => Some(Instruction::Forward(amount)),
                    _ => None,
                }
            })
            .collect::<Option<Vec<_>>>()?;

        Some(Self { instructions })
    }

    fn part_1(&self) -> String {
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
            .product::<u32>()
            .to_string()
    }

    fn part_2(&self) -> String {
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
            .product::<u32>()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(2);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "150");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(2);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "150");
    }
}
