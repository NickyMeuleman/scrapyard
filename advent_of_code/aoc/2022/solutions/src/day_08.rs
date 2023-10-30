use std::fmt::Display;

use aoc_core::AoCError;
use itertools::Itertools;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data {
    grid: Vec<Vec<u32>>,
}

impl Data {
    // I'd like to refactor this so it returns 4 slices instead of 4 owned Vecs
    fn directions(&self, x: usize, y: usize) -> [Vec<u32>; 4] {
        let row = &self.grid[y];
        let column: Vec<u32> = self
            .grid
            .iter()
            .map(|row| row[x])
            .collect();

        let (left, right) = row.split_at(x);
        let (up, down) = column.split_at(y);

        let up = up.iter().copied().rev().collect();
        let left = left.iter().copied().rev().collect();
        let right = right[1..].to_vec();
        let down = down[1..].to_vec();

        [up, down, left, right]
    }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let grid = input
            .lines()
            .map(|line| {
                line.chars()
                    .filter_map(|c| c.to_digit(10))
                    .collect()
            })
            .collect();

        Ok(Self { grid })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let len = self.grid.len();

        // All of the trees around the edge of the grid are visible
        // this loop ignores all edge-trees, we add them at the end
        let result = (1..len - 1)
            .cartesian_product(1..len - 1)
            .map(|(y, x)| {
                let height = self.grid[y][x];
                self.directions(x, y)
                    .iter()
                    // A tree is visible if all of the other trees between it and an edge of the grid are shorter than it
                    .map(|direction| direction.iter().all(|h| *h < height))
                    // count a tree that is visible from multiple directions only once
                    .any(|visible| visible)
            })
            .filter(|visible| *visible)
            .count()
            + (len - 1) * 4;

        Ok(result)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let len = self.grid.len();

        let result: u32 = (1..len - 1)
            .cartesian_product(1..len - 1)
            .map(|(y, x)| {
                let height = self.grid[y][x];
                self.directions(x, y)
                    .iter()
                    .map(|direction| {
                        direction
                            .iter()
                            // stop if you reach an edge or at the first tree that is the same height or taller than the tree under consideration.
                            .position(|h| *h >= height)
                            .map(|p| p + 1)
                            .unwrap_or_else(|| direction.len())
                    })
                    .product::<usize>()
            })
            .max()
            .ok_or(AoCError::new("No maximum found"))?
            .try_into()?;

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "30373
25512
65332
33549
35390";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "21");
    }

    #[test]
    fn part_2() {
        let input = "30373
25512
65332
33549
35390";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "8");
    }
}
