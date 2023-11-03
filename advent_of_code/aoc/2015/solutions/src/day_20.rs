use std::fmt::Display;

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(usize);

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        Ok(Self(input.parse()?))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut houses = vec![0; &self.0 / 10 + 1];
        for elf in 1..houses.len() {
            for house in (elf..houses.len()).step_by(elf) {
                houses[house] += elf * 10;
            }
        }

        houses
            .iter()
            .position(|x| x >= &self.0)
            .ok_or(AoCError::Solving)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut houses = vec![0; &self.0 / 10 + 1];
        for elf in 1..houses.len() {
            let max_house = houses.len().min(elf * 50);
            for house in (elf..max_house).step_by(elf) {
                houses[house] += elf * 11;
            }
        }

        houses
            .iter()
            .position(|x| x >= &self.0)
            .ok_or(AoCError::Solving)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "36000000";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "831600");
    }

    #[test]
    fn part_2() {
        let input = "36000000";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "884520");
    }
}
