use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use crate::{AoCData, Solution};

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    amount: i32,
}
#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}
impl Coord {
    fn manhattan(&self, other: &Coord) -> u32 {
        other.x.abs_diff(self.x) + other.y.abs_diff(self.y)
    }
}

#[derive(Debug)]
pub struct Data(Vec<Vec<Instruction>>);

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> Option<Self> {
        let wires = input
            .lines()
            .map(|line| {
                let parts = line.split(",");
                let instructions = parts
                    .map(|part| {
                        let mut part = part.chars();
                        let direction = part.next().unwrap();
                        let direction = match direction {
                            'U' => Direction::Up,
                            'L' => Direction::Left,
                            'R' => Direction::Right,
                            'D' => Direction::Down,
                            c => panic!("Invalid instruction, {}", c),
                        };
                        let amount = part
                            .collect::<String>()
                            .parse::<i32>()
                            .unwrap();
                        Instruction { direction, amount }
                    })
                    .collect();
                instructions
            })
            .collect();
        Some(Self(wires))
    }

    fn part_1(&self) -> impl Display {
        let origin = Coord { x: 0, y: 0 };
        let mut closest = u32::MAX;
        let mut wire_1 = HashSet::new();

        for i in 0..self.0.len() {
            let mut pos = Coord { x: 0, y: 0 };
            for ins in &self.0[i] {
                for _ in 0..ins.amount {
                    match ins.direction {
                        Direction::Up => pos.y -= 1,
                        Direction::Right => pos.x += 1,
                        Direction::Down => pos.y += 1,
                        Direction::Left => pos.x -= 1,
                    }
                    if i == 0 {
                        wire_1.insert(pos);
                    } else if wire_1.contains(&pos) {
                        closest = closest.min(origin.manhattan(&pos));
                    }
                }
            }
        }
        closest
    }

    fn part_2(&self) -> impl Display {
        let mut closest = u32::MAX;
        let mut wire_1 = HashMap::new();

        for i in 0..self.0.len() {
            let mut pos = Coord { x: 0, y: 0 };
            let mut steps = 0;
            for ins in &self.0[i] {
                for _ in 0..ins.amount {
                    steps += 1;
                    match ins.direction {
                        Direction::Up => pos.y -= 1,
                        Direction::Right => pos.x += 1,
                        Direction::Down => pos.y += 1,
                        Direction::Left => pos.x -= 1,
                    }
                    if i == 0 {
                        wire_1.insert(pos, steps);
                    } else {
                        if let Some(steps_1) = wire_1.get(&pos) {
                            closest = closest.min(steps_1 + steps);
                        }
                    }
                }
            }
        }
        closest
    }

    fn solve(self) -> Solution
    where
        Self: Sized,
    {
        let origin = Coord { x: 0, y: 0 };
        let mut closest = u32::MAX;
        let mut shortest = u32::MAX;
        let mut wire_1 = HashMap::new();

        for i in 0..self.0.len() {
            let mut pos = Coord { x: 0, y: 0 };
            let mut steps = 0;
            for ins in &self.0[i] {
                for _ in 0..ins.amount {
                    steps += 1;
                    match ins.direction {
                        Direction::Up => pos.y -= 1,
                        Direction::Right => pos.x += 1,
                        Direction::Down => pos.y += 1,
                        Direction::Left => pos.x -= 1,
                    }
                    if i == 0 {
                        wire_1.insert(pos, steps);
                    } else {
                        if let Some(steps_1) = wire_1.get(&pos) {
                            closest = closest.min(origin.manhattan(&pos));
                            shortest = shortest.min(steps_1 + steps);
                        }
                    }
                }
            }
        }
        Solution {
            part1: Box::new(closest),
            part2: Box::new(shortest),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::get_input;

    #[test]
    fn part_1() {
        let input = get_input(3, true).unwrap();
        let data = Data::try_new(&input).unwrap();
        let result = data.part_1().to_string();
        assert_eq!(result, "");
    }

    #[test]
    fn part_2() {
        let input = get_input(3, true).unwrap();
        let data = Data::try_new(&input).unwrap();
        let result = data.part_2().to_string();
        assert_eq!(result, "");
    }
}
