use std::fmt::Display;

use aoc_core::{AoCError, Solution};
use itertools::Itertools;

use crate::{AoCData, AoCResult};

const SIZE: usize = 300;

#[derive(Debug, Clone)]
pub struct Data(i32);

// Takes very long to execute
impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        Ok(Self(input.trim().parse()?))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let map: Vec<Vec<_>> = (1..=SIZE as i32)
            .map(|x| {
                (1..=SIZE as i32)
                    .map(move |y| {
                        // Find the fuel cell's rack ID,
                        // which is its X coordinate plus 10.
                        let rack_id = x + 10;
                        // Begin with a power level of the rack ID times the Y coordinate.
                        let mut power_level = rack_id * y;
                        // Increase the power level by the value of the grid serial number (your puzzle input).
                        power_level += self.0;
                        // Set the power level to itself multiplied by the rack ID.
                        power_level *= rack_id;
                        // Keep only the hundreds digit of the power level (so 12345 becomes 3; numbers with no hundreds digit become 0).
                        power_level /= 100;
                        power_level %= 10;
                        // Subtract 5 from the power level.
                        power_level - 5
                    })
                    .collect()
            })
            .collect();

        let (x, y) = (1..=SIZE - 2)
            .cartesian_product(1..=SIZE - 2)
            .max_by_key(|(x, y)| {
                (0..3)
                    .map(|dx| {
                        (0..3)
                            .map(|dy| map[x - 1 + dx][y - 1 + dy])
                            .sum::<i32>()
                    })
                    .sum::<i32>()
            })
            .ok_or(AoCError::Solving)?;

        Ok(format!("{},{}", x, y))
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let map: Vec<Vec<_>> = (1..=SIZE as i32)
            .map(|x| {
                (1..=SIZE as i32)
                    .map(move |y| {
                        // Find the fuel cell's rack ID,
                        // which is its X coordinate plus 10.
                        let rack_id = x + 10;
                        // Begin with a power level of the rack ID times the Y coordinate.
                        let mut power_level = rack_id * y;
                        // Increase the power level by the value of the grid serial number (your puzzle input).
                        power_level += self.0;
                        // Set the power level to itself multiplied by the rack ID.
                        power_level *= rack_id;
                        // Keep only the hundreds digit of the power level (so 12345 becomes 3; numbers with no hundreds digit become 0).
                        power_level /= 100;
                        power_level %= 10;
                        // Subtract 5 from the power level.
                        power_level - 5
                    })
                    .collect()
            })
            .collect();

        let (x, y, size) = (1..=SIZE)
            .flat_map(|size| {
                (1..=SIZE + 1 - size)
                    .flat_map(move |x| (1..=SIZE + 1 - size).map(move |y| (x, y, size)))
            })
            .max_by_key(|&(x, y, size)| {
                (0..size)
                    .map(|dx| {
                        (0..size)
                            .map(|dy| map[x - 1 + dx][y - 1 + dy])
                            .sum::<i32>()
                    })
                    .sum::<i32>()
            })
            .ok_or(AoCError::Solving)?;

        Ok(format!("{},{},{}", x, y, size))
    }

    fn solve(self) -> AoCResult<aoc_core::Solution>
    where
        Self: Sized,
    {
        let map: Vec<Vec<_>> = (1..=SIZE as i32)
            .map(|x| {
                (1..=SIZE as i32)
                    .map(move |y| {
                        // Find the fuel cell's rack ID,
                        // which is its X coordinate plus 10.
                        let rack_id = x + 10;
                        // Begin with a power level of the rack ID times the Y coordinate.
                        let mut power_level = rack_id * y;
                        // Increase the power level by the value of the grid serial number (your puzzle input).
                        power_level += self.0;
                        // Set the power level to itself multiplied by the rack ID.
                        power_level *= rack_id;
                        // Keep only the hundreds digit of the power level (so 12345 becomes 3; numbers with no hundreds digit become 0).
                        power_level /= 100;
                        power_level %= 10;
                        // Subtract 5 from the power level.
                        power_level - 5
                    })
                    .collect()
            })
            .collect();

        let (x, y) = (1..=SIZE - 2)
            .cartesian_product(1..=SIZE - 2)
            .max_by_key(|(x, y)| {
                (0..3)
                    .map(|dx| {
                        (0..3)
                            .map(|dy| map[x - 1 + dx][y - 1 + dy])
                            .sum::<i32>()
                    })
                    .sum::<i32>()
            })
            .ok_or(AoCError::Solving)?;

        let part1 = format!("{},{}", x, y);

        let (x, y, size) = (1..=SIZE)
            .flat_map(|size| {
                (1..=SIZE + 1 - size)
                    .flat_map(move |x| (1..=SIZE + 1 - size).map(move |y| (x, y, size)))
            })
            .max_by_key(|&(x, y, size)| {
                (0..size)
                    .map(|dx| {
                        (0..size)
                            .map(|dy| map[x - 1 + dx][y - 1 + dy])
                            .sum::<i32>()
                    })
                    .sum::<i32>()
            })
            .ok_or(AoCError::Solving)?;

        let part2 = format!("{},{},{}", x, y, size);

        Ok(Solution {
            part1: Box::new(part1),
            part2: Box::new(part2),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "42";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "21,61");
    }

    #[test]
    fn part_2() {
        let input = "42";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "232,251,12");
    }

    #[test]
    fn solve() {
        let input = "42";
        let data = Data::try_new(input).unwrap();
        let Solution { part1, part2 } = data.solve().unwrap();
        assert_eq!(part1.to_string(), "21,61");
        assert_eq!(part2.to_string(), "90,269,16");
    }
}
