// Blog writeup with simpler Rust code (I should handle errors here):
// https://nickymeuleman.netlify.app/blog/aoc2024-day12/

use crate::{AoCData, AoCResult};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Display;

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

    fn same_neighbours(&self, map: &HashMap<Point, char>, target_crop: char) -> Vec<Self> {
        let mut neighbours = Vec::new();
        for dir in Self::dirs() {
            let next = self.add(&dir);
            if map
                .get(&next)
                .is_some_and(|&crop| crop == target_crop)
            {
                neighbours.push(next);
            }
        }
        neighbours
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

    fn perp(&self) -> Self {
        // does not matter if (row, col) turns into (-col, row) or (col, row) for this algorithm
        Point::new(-self.col, self.row)
    }
}

fn shape(
    start: Point,
    crop: char,
    map: &HashMap<Point, char>,
    seen: &mut HashSet<Point>,
) -> HashSet<Point> {
    let mut q = VecDeque::new();
    let mut shape = HashSet::new();
    q.push_back(start);
    shape.insert(start);

    while let Some(point) = q.pop_front() {
        for neighbour in point.same_neighbours(map, crop) {
            if seen.insert(neighbour) {
                shape.insert(neighbour);
                q.push_back(neighbour);
            }
        }
    }

    shape
}

fn circumference(map: &HashMap<Point, char>, shape: &HashSet<Point>) -> usize {
    shape
        .iter()
        .map(|point| {
            4 - point
                .same_neighbours(map, map[point])
                .len()
        })
        .sum()
}

fn sides(shape: HashSet<Point>) -> usize {
    let mut sides = HashSet::new();
    for point in &shape {
        for dir in Point::dirs() {
            // look for first out of bounds element in dir
            if shape.contains(&point.add(&dir)) {
                continue;
            }
            // perpendicular dir
            let perp = dir.perp();
            let mut curr = *point;

            // keep moving in the perpendicular direction while:
            // - a block in the perpendicular direction exists
            // - a block in the original direction doesn't exist
            while shape.contains(&curr.add(&perp)) && !shape.contains(&curr.add(&dir)) {
                curr = curr.add(&perp);
            }
            // when edge was followed, as this (point, dir) to the sides.
            // include dir because 1 point has 4 sides
            sides.insert((curr, dir));
        }
    }
    sides.len()
}

#[derive(Debug, Clone)]
pub struct Data(HashMap<Point, char>);

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut map = HashMap::new();
        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                let point = Point::new(row as i32, col as i32);
                map.insert(point, c);
            }
        }
        Ok(Self(map))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut seen = HashSet::new();
        let mut sum = 0;
        for (point, crop) in &self.0 {
            if seen.contains(point) {
                continue;
            }
            let shape = shape(*point, *crop, &self.0, &mut seen);
            let area = shape.len();
            let circumference = circumference(&self.0, &shape);
            sum += area * circumference;
        }
        Ok(sum)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut seen = HashSet::new();
        let mut sum = 0;
        for (point, crop) in &self.0 {
            if seen.contains(point) {
                continue;
            }
            let shape = shape(*point, *crop, &self.0, &mut seen);
            let area = shape.len();
            let sides = sides(shape);
            sum += area * sides;
        }
        Ok(sum)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "1930");
    }

    #[test]
    fn part_2() {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "1206");
    }
}
