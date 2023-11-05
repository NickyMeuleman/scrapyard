use std::{collections::HashSet, fmt::Display};

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<Direction>);

#[derive(Debug, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn step(&mut self, dir: &Direction) {
        match dir {
            Direction::North => self.y += 1,
            Direction::South => self.y -= 1,
            Direction::East => self.x -= 1,
            Direction::West => self.x += 1,
        };
    }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let directions = input
            .chars()
            .map(|c| match c {
                '^' => Some(Direction::North),
                'v' => Some(Direction::South),
                '<' => Some(Direction::East),
                '>' => Some(Direction::West),
                _ => None,
            })
            .collect::<Option<Vec<Direction>>>()
            .ok_or(AoCError::Parsing)?;

        Ok(Self(directions))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut santa = Point { x: 0, y: 0 };
        let mut visited: HashSet<Point> = HashSet::new();
        // visit starting point
        visited.insert(santa);
        for dir in self.0.iter() {
            // move to a house
            santa.step(dir);
            // visit a house
            visited.insert(santa);
        }

        Ok(visited.len())
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut santa = Point { x: 0, y: 0 };
        let mut robo_santa = Point { x: 0, y: 0 };
        let mut visited: HashSet<Point> = HashSet::new();
        visited.insert(santa);
        visited.insert(robo_santa);
        for (idx, dir) in self.0.iter().enumerate() {
            if idx % 2 == 0 {
                santa.step(dir);
                visited.insert(santa);
            } else {
                robo_santa.step(dir);
                visited.insert(robo_santa);
            }
        }

        Ok(visited.len())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "^>v<";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "4");
    }

    #[test]
    fn part_2() {
        let input = "^>v<";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "3");
    }
}
