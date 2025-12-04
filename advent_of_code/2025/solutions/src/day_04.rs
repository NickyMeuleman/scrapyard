// Blog writeup with simpler Rust code (I should handle errors here):
// https://nickymeuleman.netlify.app/blog/aoc2025-day04/

use crate::{AoCData, AoCResult};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Data(Vec<Vec<char>>);

#[derive(Debug, Clone, Copy, Hash, Eq, Ord, PartialEq, PartialOrd)]
struct Point {
    row: usize,
    col: usize,
}

const DELTAS: [(isize, isize); 8] = [
    (-1, 0),  // up
    (-1, 1),  // up right
    (0, 1),   // right
    (1, 1),   // down right
    (1, 0),   // down
    (1, -1),  // down left
    (0, -1),  // left
    (-1, -1), // up left
];

impl Point {
    fn neighbours(&self) -> Vec<Point> {
        DELTAS
            .iter()
            .filter_map(|&(dr, dc)| {
                Some(Point {
                    row: self.row.checked_add_signed(dr)?,
                    col: self.col.checked_add_signed(dc)?,
                })
            })
            .collect()
    }

    fn count_neighbours(&self, map: &[Vec<char>]) -> usize {
        self.neighbours()
            .iter()
            .filter(|point| {
                matches!(
                    map.get(point.row)
                        .and_then(|row| row.get(point.col)),
                    Some('@')
                )
            })
            .count()
    }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let map: Vec<Vec<char>> = input
            .lines()
            .map(|l| l.chars().collect())
            .collect();
        Ok(Self(map))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut sum = 0;
        for (row, line) in self.0.iter().enumerate() {
            for (col, c) in line.iter().enumerate() {
                if *c != '@' {
                    continue;
                }
                let point = Point { row, col };
                if point.count_neighbours(&self.0) < 4 {
                    sum += 1;
                }
            }
        }

        Ok(sum)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut map = self.0.clone();

        let mut sum = 0;
        loop {
            let mut to_remove = Vec::new();
            for (row, line) in map.iter().enumerate() {
                for (col, c) in line.iter().enumerate() {
                    if *c != '@' {
                        continue;
                    }
                    let point = Point { row, col };
                    if point.count_neighbours(&map) < 4 {
                        to_remove.push(point);
                    }
                }
            }

            if to_remove.is_empty() {
                break;
            }
            sum += to_remove.len();
            for point in to_remove {
                map[point.row][point.col] = '.';
            }
        }

        Ok(sum)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "13");
    }

    #[test]
    fn part_2() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "43");
    }
}
