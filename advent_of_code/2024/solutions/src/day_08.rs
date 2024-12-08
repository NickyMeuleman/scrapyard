use crate::{AoCData, AoCResult};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

#[derive(Debug, Clone)]
pub struct Data {
    rows: i32,
    cols: i32,
    antennas: HashMap<char, Vec<Point>>,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
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

    fn dist(&self, other: &Self) -> Self {
        Point::new(self.row - other.row, self.col - other.col)
    }

    fn add(&self, other: &Self) -> Self {
        Point::new(self.row + other.row, self.col + other.col)
    }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let rows = input.lines().count();
        let cols = input
            .lines()
            .next()
            .unwrap()
            .chars()
            .count();

        let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c != '.' {
                    let point = Point::new(row as i32, col as i32);
                    antennas
                        .entry(c)
                        .or_default()
                        .push(point);
                }
            }
        }
        Ok(Self {
            rows: rows as i32,
            cols: cols as i32,
            antennas,
        })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut antinodes = HashSet::new();
        for points in self.antennas.values() {
            for p1 in points {
                for p2 in points {
                    if p1 == p2 {
                        continue;
                    }
                    let dist = p1.dist(p2);
                    let new = p1.add(&dist);
                    if new.is_in_bounds(self.rows, self.cols) {
                        antinodes.insert(new);
                    }
                }
            }
        }

        Ok(antinodes.len())
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut antinodes = HashSet::new();
        for points in self.antennas.values() {
            for p1 in points {
                for p2 in points {
                    if p1 == p2 {
                        continue;
                    }
                    let dist = p1.dist(p2);
                    let mut new = *p1;
                    while new.is_in_bounds(self.rows, self.cols) {
                        antinodes.insert(new);
                        new = new.add(&dist);
                    }
                }
            }
        }
        Ok(antinodes.len())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "14");
    }

    #[test]
    fn part_2() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "34");
    }
}
