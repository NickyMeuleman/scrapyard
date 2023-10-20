use std::fmt::Display;

use crate::{intcode::Computer, AoCData};

#[derive(Debug, Clone)]
pub struct Data(Vec<i32>);

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> Option<Self> {
        Some(Self(
            input
                .split(",")
                .filter_map(|s| s.parse().ok())
                .collect(),
        ))
    }

    fn part_1(&self) -> impl Display {
        let mut computer = Computer::new();
        let mut memory = self.0.clone();
        memory[1] = 12;
        memory[2] = 2;
        computer.set_memory(memory);
        computer.run();
        computer.memory[0]
    }

    fn part_2(&self) -> impl Display {
        for noun in 0..=99 {
            for verb in 0..=99 {
                let mut computer = Computer::new();
                let mut memory = self.0.clone();
                memory[1] = noun;
                memory[2] = verb;
                computer.set_memory(memory);
                computer.run();
                if computer.memory[0] == 19690720 {
                    return (100 * noun + verb).to_string();
                }
            }
        }
        "No solution was found".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "1,0,0,0,99";
        let data = Data::try_new(&input).unwrap();
        let mut computer = Computer::new();
        computer.set_memory(data.0);
        computer.run();
        assert_eq!(computer.memory, [2, 0, 0, 0, 99]);

        let input = "2,3,0,3,99";
        let data = Data::try_new(&input).unwrap();
        let mut computer = Computer::new();
        computer.set_memory(data.0);
        computer.run();
        assert_eq!(computer.memory, [2, 3, 0, 6, 99]);

        let input = "2,4,4,5,99,0";
        let data = Data::try_new(&input).unwrap();
        let mut computer = Computer::new();
        computer.set_memory(data.0);
        computer.run();
        assert_eq!(computer.memory, [2, 4, 4, 5, 99, 9801]);

        let input = "1,1,1,4,99,5,6,0,99";
        let data = Data::try_new(&input).unwrap();
        let mut computer = Computer::new();
        computer.set_memory(data.0);
        computer.run();
        assert_eq!(computer.memory, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
