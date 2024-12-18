// Blog writeup with simpler Rust code (I should handle errors here):
// https://nickymeuleman.netlify.app/blog/aoc2024-day18/

use crate::{AoCData, AoCResult};
use aoc_core::AoCError;
use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

#[derive(Debug, Clone)]
pub struct Data(Vec<Point>);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    row: i32,
    col: i32,
}

impl Point {
    fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }

    fn add(&self, other: &Self) -> Self {
        Self::new(self.row + other.row, self.col + other.col)
    }

    fn dirs() -> [Point; 4] {
        [
            // up
            Self::new(-1, 0),
            // right
            Self::new(0, 1),
            // down
            Self::new(1, 0),
            // left
            Self::new(0, -1),
        ]
    }

    fn is_in_bounds(&self, rows: i32, cols: i32) -> bool {
        (self.row >= 0 && self.row < rows) && (self.col >= 0 && self.col < cols)
    }

    fn neighbours(&self, rows: i32, cols: i32) -> Vec<Self> {
        let mut neighbours = Vec::new();
        for dir in Self::dirs() {
            let next = self.add(&dir);
            if next.is_in_bounds(rows, cols) {
                neighbours.push(next);
            }
        }
        neighbours
    }
}

fn search(
    corrupted: &HashSet<&Point>,
    start: Point,
    end: Point,
    rows: i32,
    cols: i32,
) -> Option<u32> {
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();
    q.push_back((start, 0));
    seen.insert(start);

    while let Some((pos, cost)) = q.pop_front() {
        if pos == end {
            return Some(cost);
        }
        for neighbour in pos.neighbours(rows, cols) {
            if corrupted.contains(&neighbour) {
                continue;
            }
            if seen.insert(neighbour) {
                q.push_back((neighbour, cost + 1));
            }
        }
    }

    None
}

fn p1_helper(falling: &[Point], rows: i32, cols: i32, size: usize) -> Result<u32, AoCError> {
    let corrupted: HashSet<&Point> = falling.iter().take(size).collect();
    let start = Point::new(0, 0);
    let end = Point::new(rows - 1, cols - 1);
    search(&corrupted, start, end, rows, cols).ok_or(AoCError::Solving)
}

fn p2_helper(falling: &[Point], rows: i32, cols: i32) -> Result<String, AoCError> {
    let start = Point::new(0, 0);
    let end = Point::new(rows - 1, cols - 1);

    let mut low = 0;
    let mut high = falling.len() - 1;

    while low < high {
        let mid = (low + high) / 2;
        let corrupted: HashSet<&Point> = falling.iter().take(mid + 1).collect();
        if search(&corrupted, start, end, rows, cols).is_some() {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    falling
        .get(low)
        .map(|point| format!("{},{}", point.col, point.row))
        .ok_or(AoCError::Solving)
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        Ok(Self(
            input
                .lines()
                .map(|line| {
                    let (col, row) = line.split_once(',').unwrap();
                    Point::new(row.parse().unwrap(), col.parse().unwrap())
                })
                .collect(),
        ))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        p1_helper(&self.0, 71, 71, 1024)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        p2_helper(&self.0, 71, 71)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        let data = Data::try_new(input).unwrap();
        let result = p1_helper(&data.0, 7, 7, 12).unwrap();
        assert_eq!(result, 22);
    }

    #[test]
    fn part_2() {
        let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        let data = Data::try_new(input).unwrap();
        let result = p2_helper(&data.0, 7, 7).unwrap();
        assert_eq!(result, "6,1");
    }
}
