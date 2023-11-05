use std::fmt::Display;

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data {
    positions: Vec<u16>,
}

impl Data {
    fn min_max(&self) -> (u16, u16) {
        self.positions
            .iter()
            .fold((u16::MAX, u16::MIN), |(min, max), pos| {
                (*pos.min(&min), *pos.max(&max))
            })
    }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let positions = input
            .trim()
            .split(',')
            .map(|s| s.parse())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { positions })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let (min, max) = self.min_max();

        (min..=max)
            .map(|target_pos| {
                // calculate the total cost if every crab went to target_pos
                self.positions
                    .iter()
                    .map(move |&start_pos| (start_pos, target_pos))
                    // calculate cost for a single crab:
                    // get amount a crab has to move to get to target_pos
                    // get cost for that distance
                    .map(|(to, from)| (to as i32 - from as i32).abs())
                    // sum all costs for each crab
                    .sum::<i32>()
            })
            .min()
            .ok_or(AoCError::Solving)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let (min, max) = self.min_max();

        (min..=max)
            .map(|target_pos| {
                // calculate the total cost if every crab went to target_pos
                self.positions
                    .iter()
                    .map(move |&start_pos| (start_pos, target_pos))
                    // calculate cost for a single crab:
                    .map(|(to, from)| {
                        // get amount a crab has to move to get to target_pos
                        let distance = (to as i32 - from as i32).abs();
                        let steps = distance;
                        let first = 1;
                        let last = distance;
                        // get cost for that distance
                        // This is the sum of an arithmetic sequence: https://en.wikipedia.org/wiki/Arithmetic_progression
                        steps * (first + last) / 2
                    })
                    // sum all costs for each crab
                    .sum::<i32>()
            })
            .min()
            .ok_or(AoCError::Solving)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "37");
    }

    #[test]
    fn part_2() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "168");
    }
}
