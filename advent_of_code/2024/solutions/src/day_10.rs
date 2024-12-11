// Blog writeup with simpler Rust code (I should handle errors here):
// https://nickymeuleman.netlify.app/blog/aoc2024-day10/

use aoc_core::Solution;

use crate::{AoCData, AoCResult};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord, Debug)]
struct Point {
    row: i32,
    col: i32,
}

impl Point {
    fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }

    fn is_in_bounds(&self, rows: i32, cols: i32) -> bool {
        (self.row >= 0 && self.row < rows) && (self.col >= 0 && self.col < cols)
    }

    fn add(&self, other: &Self) -> Self {
        Self::new(self.row + other.row, self.col + other.col)
    }

    fn neighbours(&self, rows: i32, cols: i32) -> Vec<Self> {
        let mut neighbours = Vec::new();
        let dirs = [
            // up
            Point::new(-1, 0),
            // right
            Point::new(0, 1),
            // down
            Point::new(1, 0),
            // left
            Point::new(0, -1),
        ];
        for dir in dirs {
            let next = self.add(&dir);
            if next.is_in_bounds(rows, cols) {
                neighbours.push(next);
            }
        }
        neighbours
    }
}

#[derive(Debug, Clone)]
pub struct Data {
    rows: i32,
    cols: i32,
    map: HashMap<Point, u32>,
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let rows = input.lines().count() as i32;
        let cols = input
            .lines()
            .next()
            .unwrap()
            .chars()
            .count() as i32;

        let mut map = HashMap::new();
        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                let point = Point::new(row as i32, col as i32);
                let height = c.to_digit(10).unwrap();
                map.insert(point, height);
            }
        }
        Ok(Self { rows, cols, map })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let starts = self
            .map
            .iter()
            .filter_map(|(point, &height)| (height == 0).then_some(*point));

        let mut sum = 0;
        for start in starts {
            // (point, height)
            let mut q = VecDeque::new();
            let mut seen = HashSet::new();
            q.push_back((start, 0));
            seen.insert(start);

            while let Some((point, height)) = q.pop_front() {
                if height == 9 {
                    sum += 1;
                    continue;
                }
                for neighbour in point.neighbours(self.rows, self.cols) {
                    let neighbour_height = self.map[&neighbour];
                    if neighbour_height != height + 1 {
                        continue;
                    }
                    if seen.insert(neighbour) {
                        q.push_back((neighbour, neighbour_height));
                    }
                }
            }
        }

        Ok(sum)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let starts = self
            .map
            .iter()
            .filter_map(|(point, &height)| (height == 0).then_some(*point));

        let mut sum = 0;
        for start in starts {
            // (point, height)
            let mut q = VecDeque::new();
            q.push_back((start, 0));

            while let Some((point, height)) = q.pop_front() {
                if height == 9 {
                    sum += 1;
                    continue;
                }
                for neighbour in point.neighbours(self.rows, self.cols) {
                    let neighbour_height = self.map[&neighbour];
                    if neighbour_height != height + 1 {
                        continue;
                    }
                    q.push_back((neighbour, neighbour_height));
                }
            }
        }

        Ok(sum)
    }

    fn solve(self) -> AoCResult<aoc_core::Solution>
    where
        Self: Sized,
    {
        let starts = self
            .map
            .iter()
            .filter_map(|(point, &height)| (height == 0).then_some(*point));

        let mut nines = 0;
        let mut routes = 0;
        for start in starts {
            // (point, height)
            let mut q = VecDeque::new();
            let mut endings = Vec::new();
            q.push_back((start, 0));

            while let Some((point, height)) = q.pop_front() {
                if height == 9 {
                    endings.push(point);
                    continue;
                }
                for neighbour in point.neighbours(self.rows, self.cols) {
                    let neighbour_height = self.map[&neighbour];
                    if neighbour_height != height + 1 {
                        continue;
                    }
                    q.push_back((neighbour, neighbour_height));
                }
            }
            routes += endings.len();
            nines += endings
                .iter()
                .collect::<HashSet<_>>()
                .len();
        }

        Ok(Solution {
            part1: Box::new(nines),
            part2: Box::new(routes),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "36");
    }

    #[test]
    fn part_2() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "81");
    }
}
