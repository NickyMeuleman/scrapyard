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
        computer.set_memory(self.0.clone());
        computer.input(1);
        computer.run().unwrap()
    }

    fn part_2(&self) -> impl Display {
        let mut computer = Computer::new();
        computer.set_memory(self.0.clone());
        computer.input(5);
        computer.run().unwrap()
    }
}
