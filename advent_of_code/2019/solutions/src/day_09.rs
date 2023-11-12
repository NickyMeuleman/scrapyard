use std::fmt::Display;

use aoc_core::AoCError;

use crate::{intcode::Computer, AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<i64>);

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        Ok(Self(
            input
                .split(',')
                .filter_map(|s| s.parse().ok())
                .collect(),
        ))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut computer: Computer = Default::default();
        computer.set_memory(self.0.clone());
        computer.input(1);
        computer.run();
        computer
            .last_output()
            .ok_or(AoCError::Solving)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut computer: Computer = Default::default();
        computer.set_memory(self.0.clone());
        computer.input(2);
        computer.run();
        computer
            .last_output()
            .ok_or(AoCError::Solving)
    }
}

#[cfg(test)]
mod test {
    use crate::intcode::Status;

    use super::*;

    #[test]
    fn part_1() {
        let input = "104,1125899906842624,99";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "1125899906842624");

        let input = "1102,34915192,34915192,7,4,7,99,0";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result.len(), 16);

        let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let mut computer: Computer = Default::default();
        let data = Data::try_new(input).unwrap();
        computer.set_memory(data.0.clone());
        computer.input(1);
        while let Ok(Status::Running) = computer.operate() {}
        computer.memory.truncate(data.0.len());
        assert_eq!(format!("{:?}", data.0), format!("{:?}", computer.memory));
    }
}
