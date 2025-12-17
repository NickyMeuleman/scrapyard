// Blog writeup with simpler Rust code (I should handle errors here):
// https://nickymeuleman.netlify.app/blog/aoc2025-day09/

use crate::{AoCData, AoCError, AoCResult};
use itertools::Itertools;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Data(Vec<Point>);

#[derive(Debug, Clone, Copy, Hash, Eq, Ord, PartialEq, PartialOrd)]
struct Point {
    row: u64,
    col: u64,
}

impl Point {
    fn area(&self, other: &Self) -> u64 {
        (self.row.abs_diff(other.row) + 1) * (self.col.abs_diff(other.col) + 1)
    }

    fn valid_rect(&self, other: &Self, lines: &[Line]) -> bool {
        lines
            .iter()
            .all(|line| line.outside_rect(self, other))
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, Ord, PartialEq, PartialOrd)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    /// returns true if line is outside rectangle made by 2 points
    /// a line that touches the edges of the rectangle is still considered as outside
    /// returns false if line overlaps or lies inside the rectangle
    fn outside_rect(&self, p1: &Point, p2: &Point) -> bool {
        let min_row = p1.row.min(p2.row);
        let max_row = p1.row.max(p2.row);
        let min_col = p1.col.min(p2.col);
        let max_col = p1.col.max(p2.col);

        let line_min_row = self.start.row.min(self.end.row);
        let line_max_row = self.start.row.max(self.end.row);
        let line_min_col = self.start.col.min(self.end.col);
        let line_max_col = self.start.col.max(self.end.col);

        let left = line_max_col <= min_col;
        let right = line_min_col >= max_col;
        let above = line_max_row <= min_row;
        let below = line_min_row >= max_row;

        left || right || above || below
    }
}
impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let points = input
            .lines()
            .map(|line| {
                let mut parts = line.split(',');
                let row = parts
                    .next()
                    .ok_or(AoCError::Parsing)?
                    .parse()?;
                let col = parts
                    .next()
                    .ok_or(AoCError::Parsing)?
                    .parse()?;
                if parts.next().is_some() {
                    return Err(AoCError::Parsing);
                }
                Ok(Point { row, col })
            })
            .collect::<AoCResult<_>>()?;
        Ok(Self(points))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        self.0
            .iter()
            .tuple_combinations()
            .map(|(p1, p2)| p1.area(p2))
            .max()
            .ok_or(AoCError::Solving)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let lines: Vec<Line> = self
            .0
            .iter()
            .circular_tuple_windows()
            .map(|(p1, p2)| Line {
                start: *p1,
                end: *p2,
            })
            .collect();

        self.0
            .iter()
            .tuple_combinations()
            .filter(|(p1, p2)| p1.valid_rect(p2, &lines))
            .map(|(p1, p2)| p1.area(p2))
            .max()
            .ok_or(AoCError::Solving)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "50");
    }

    #[test]
    fn part_2() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "24");
    }
}
