use std::{collections::VecDeque, fmt::Display};

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Ash,
    Rock,
}

#[derive(Debug, Clone)]
pub struct Data(Vec<VecDeque<Vec<Tile>>>);

fn reflects_at(grid: &VecDeque<Vec<Tile>>, smudges: usize) -> Option<usize> {
    (1..grid.len()).find(|&offset| {
        let half1 = grid.iter().take(offset).rev();
        let half2 = grid.iter().skip(offset);
        let combined = half1.zip(half2); // the shortest half determines how long this is!
        let found_smudges: usize = combined
            .map(|(row1, row2)| {
                row1.iter()
                    .zip(row2.iter())
                    .filter(|(a, b)| a != b)
                    .count()
            })
            .sum();

        found_smudges == smudges
    })
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let blocks = input
            .split("\n\n")
            .map(|block| {
                block
                    .lines()
                    .map(|line| {
                        line.chars()
                            .map(|c| match c {
                                '.' => Ok(Tile::Ash),
                                '#' => Ok(Tile::Rock),
                                _ => Err(AoCError::Parsing),
                            })
                            .collect::<AoCResult<Vec<_>>>()
                    })
                    .collect::<AoCResult<VecDeque<_>>>()
            })
            .collect::<AoCResult<_>>()?;

        Ok(Self(blocks))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let sum: usize = self
            .0
            .iter()
            .map(|grid| {
                // check horizontal
                if let Some(i) = reflects_at(grid, 0) {
                    return i * 100;
                }

                // check vertical
                let cols = (0..grid[0].len())
                    .map(|i| grid.iter().map(|row| row[i]).collect())
                    .collect();
                if let Some(i) = reflects_at(&cols, 0) {
                    return i;
                }

                // no reflection found
                0
            })
            .sum();

        Ok(sum)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let sum: usize = self
            .0
            .iter()
            .map(|grid| {
                // check horizontal
                if let Some(i) = reflects_at(grid, 1) {
                    return i * 100;
                }

                // check vertical
                let cols = (0..grid[0].len())
                    .map(|i| grid.iter().map(|row| row[i]).collect())
                    .collect();
                if let Some(i) = reflects_at(&cols, 1) {
                    return i;
                }

                // no reflection found
                0
            })
            .sum();

        Ok(sum)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "405");
    }

    #[test]
    fn part_2() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "400");
    }
}
