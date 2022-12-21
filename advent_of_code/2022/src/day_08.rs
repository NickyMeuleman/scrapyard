use itertools::Itertools;

use crate::AoCData;

pub struct Data {
    grid: Vec<Vec<u32>>,
}

impl Data {
    // I'd like to refactor this so it returns 4 slices instead of 4 owned Vecs
    fn directions(&self, x: usize, y: usize) -> [Vec<u32>; 4] {
        let row = &self.grid[y];
        let column: Vec<u32> = self.grid.iter().map(|row| row[x]).collect();

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
    fn try_new(input: &str) -> Option<Self> {
        let grid = input
            .lines()
            .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
            .collect();

        Some(Self { grid })
    }

    fn part_1(&self) -> String {
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

        result.to_string()
    }

    fn part_2(&self) -> String {
        let len = self.grid.len();

        (1..len - 1)
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
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(8);
        let data = Data::try_new(&input).unwrap();
        assert_eq!(data.part_1(), "21");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(8);
        let data = Data::try_new(&input).unwrap();
        assert_eq!(data.part_2(), "8");
    }
}
