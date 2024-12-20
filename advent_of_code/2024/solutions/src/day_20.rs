// Blog writeup with simpler Rust code (I should handle errors here):
// https://nickymeuleman.netlify.app/blog/aoc2024-day20/

use crate::{AoCData, AoCError, AoCResult};
use aoc_core::Solution;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Data {
    start: Point,
    end: Point,
    grid: HashMap<Point, Tile>,
}

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

    fn neighbours(&self, grid: &HashMap<Point, Tile>) -> Vec<Self> {
        let mut neighbours = Vec::new();
        for dir in Self::dirs() {
            let next = self.add(&dir);
            if grid.contains_key(&next) {
                neighbours.push(next);
            }
        }
        neighbours
    }

    fn manhattan(&self, other: &Self) -> u32 {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
}

fn count(distmap: &HashMap<Point, u32>, max_skip: u32) -> Result<u32, AoCError> {
    // if any step between 2 allowed points has a larger savings than 100, increment count
    // let mut count = 0;
    // for ((point1, cost1), (point2, cost2)) in distmap.iter().tuple_combinations() {
    //     let skip_size = point1.manhattan(point2);
    //     if skip_size > max_skip {
    //         continue;
    //     }
    //     let saved = cost1.abs_diff(*cost2) - skip_size;
    //     if saved >= 100 {
    //         count += 1;
    //     }
    // }
    // count

    distmap
        .iter()
        // get all possible 2 point pairs
        .tuple_combinations()
        .filter_map(|((p1, c1), (p2, c2))| {
            // confirm the distance between the 2 points is small enough
            let skip_size = p1.manhattan(p2);
            if skip_size <= max_skip {
                let saved = c1.abs_diff(*c2) - skip_size;
                // confirm the savings after applying that skip would be large enough
                if saved >= 100 {
                    return Some(saved);
                }
            }
            None
        })
        .count()
        .try_into()
        .map_err(|_| AoCError::Solving)
}

fn build_distmap(
    grid: &HashMap<Point, Tile>,
    start: Point,
    end: Point,
) -> Result<HashMap<Point, u32>, AoCError> {
    let mut q = VecDeque::new();
    let mut distmap = HashMap::new();

    q.push_back((start, 0));

    while let Some((pos, cost)) = q.pop_front() {
        // update map of lowest cost distances to a position
        if distmap.contains_key(&pos) {
            continue;
        }
        distmap.insert(pos, cost);
        // stop if popped item is the end as there is only 1 route through the grid
        if pos == end {
            return Ok(distmap);
        }
        for neighbour in pos.neighbours(grid) {
            if grid[&neighbour] != Tile::Wall {
                q.push_back((neighbour, cost + 1));
            }
        }
    }

    Err(AoCError::Solving)
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut start = Point::new(0, 0);
        let mut end = Point::new(0, 0);
        let mut grid = HashMap::new();

        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                let point = Point::new(row as i32, col as i32);
                let tile = match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Wall,
                    'S' => {
                        start.row = row as i32;
                        start.col = col as i32;
                        Tile::Empty
                    }
                    'E' => {
                        end.row = row as i32;
                        end.col = col as i32;
                        Tile::Empty
                    }
                    _ => return Err(AoCError::Parsing),
                };
                grid.insert(point, tile);
            }
        }
        Ok(Self { start, end, grid })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let distmap = build_distmap(&self.grid, self.start, self.end)?;
        count(&distmap, 2)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let distmap = build_distmap(&self.grid, self.start, self.end)?;
        count(&distmap, 20)
    }

    fn solve(self) -> AoCResult<aoc_core::Solution>
    where
        Self: Sized,
    {
        let distmap = build_distmap(&self.grid, self.start, self.end)?;
        Ok(Solution {
            part1: Box::new(count(&distmap, 2)?),
            part2: Box::new(count(&distmap, 20)?),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // the tests in the example tests return 0 as sum, the question text lists all skips that are
    // possible
    #[test]
    fn part_1() {
        let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "0");
    }

    #[test]
    fn part_2() {
        let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "0");
    }
}
