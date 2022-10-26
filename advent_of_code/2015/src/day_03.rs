use std::collections::HashSet;

use crate::AoCData;

#[derive(Debug)]
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
pub struct Data(Vec<Direction>);

impl AoCData for Data {
    // returns None if parsing fails at any point
    fn try_new(input: String) -> Option<Self> {
        let directions = input
            .chars()
            .map(|c| match c {
                '^' => Some(Direction::North),
                'v' => Some(Direction::South),
                '<' => Some(Direction::East),
                '>' => Some(Direction::West),
                _ => None,
            })
            .collect::<Option<Vec<Direction>>>()?;

        Some(Self(directions))
    }

    fn part_1(&self) -> String {
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

        visited.len().to_string()
    }

    fn part_2(&self) -> String {
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

        visited.len().to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(3);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "4");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(3);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "3");
    }
}
