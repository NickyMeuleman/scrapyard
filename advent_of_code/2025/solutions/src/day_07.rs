// Blog writeup with simpler Rust code (I should handle errors here):
// https://nickymeuleman.netlify.app/blog/aoc2025-day07/

use crate::{AoCData, AoCResult};
use aoc_core::AoCError;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, Hash, Eq, Ord, PartialEq, PartialOrd)]
struct Point {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone)]
pub struct Data(Vec<Vec<char>>);

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let grid: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        if grid.is_empty() || grid[0].is_empty() {
            return Err(AoCError::Parsing);
        }
        Ok(Self(grid))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let rows = self.0.len();
        let cols = self
            .0
            .first()
            .ok_or(AoCError::Parsing)?
            .len();
        let start = self
            .0
            .first()
            .ok_or(AoCError::Parsing)?
            .iter()
            .position(|&c| c == 'S')
            .ok_or(AoCError::Parsing)?;

        let mut beams = HashSet::new();
        beams.insert(Point { row: 0, col: start });
        let mut sum = 0;

        while !beams.is_empty() {
            let mut next = HashSet::new();

            for Point { row, col } in beams {
                if row + 1 >= rows {
                    continue;
                }
                let cell = self
                    .0
                    .get(row + 1)
                    .and_then(|r| r.get(col))
                    .ok_or(AoCError::Solving)?;

                match cell {
                    '.' => {
                        let down = Point { row: row + 1, col };
                        next.insert(down);
                    }
                    '^' => {
                        sum += 1;
                        if col > 0 {
                            let down_left = Point {
                                row: row + 1,
                                col: col - 1,
                            };
                            next.insert(down_left);
                        }
                        if col + 1 < cols {
                            let down_right = Point {
                                row: row + 1,
                                col: col + 1,
                            };
                            next.insert(down_right);
                        }
                    }
                    _ => return Err(AoCError::Solving),
                }
            }
            beams = next;
        }

        Ok(sum)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let rows = self.0.len();
        let cols = self
            .0
            .first()
            .ok_or(AoCError::Parsing)?
            .len();
        let start = self
            .0
            .first()
            .ok_or(AoCError::Parsing)?
            .iter()
            .position(|&c| c == 'S')
            .ok_or(AoCError::Parsing)?;

        let mut beams: HashMap<Point, u64> = HashMap::new();
        beams.insert(Point { row: 0, col: start }, 1);
        let mut sum = 0;

        while !beams.is_empty() {
            let mut next = HashMap::new();

            for (Point { row, col }, count) in beams {
                if row + 1 >= rows {
                    sum += count;
                    continue;
                }
                let cell = self
                    .0
                    .get(row + 1)
                    .and_then(|r| r.get(col))
                    .ok_or(AoCError::Solving)?;

                match cell {
                    '.' => {
                        let down = Point { row: row + 1, col };
                        *next.entry(down).or_default() += count;
                    }
                    '^' => {
                        if col > 0 {
                            let down_left = Point {
                                row: row + 1,
                                col: col - 1,
                            };
                            *next.entry(down_left).or_default() += count;
                        }
                        if col + 1 < cols {
                            let down_right = Point {
                                row: row + 1,
                                col: col + 1,
                            };
                            *next.entry(down_right).or_default() += count;
                        }
                    }
                    _ => return Err(AoCError::Solving),
                }
            }
            beams = next;
        }

        Ok(sum)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "21");
    }

    #[test]
    fn part_2() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "40");
    }
}
