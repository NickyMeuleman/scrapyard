use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use crate::{AoCData, Solution};

#[derive(Debug, Clone)]
struct Instruction {
    direction: Direction,
    amount: i32,
}
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

    #[test]
    fn part_1() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";
        let data = Data::try_new(&input).unwrap();
        let result = data.part_1().to_string();
        assert_eq!(result, "159");

        let input = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        let data = Data::try_new(&input).unwrap();
        let result = data.part_1().to_string();
        assert_eq!(result, "135");
    }

    #[test]
    fn part_2() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";
        let data = Data::try_new(&input).unwrap();
        let result = data.part_2().to_string();
        assert_eq!(result, "610");

        let input = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        let data = Data::try_new(&input).unwrap();
        let result = data.part_2().to_string();
        assert_eq!(result, "410");
    }

    #[test]
    fn solve() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";
        let data = Data::try_new(&input).unwrap();
        let Solution { part1, part2 } = data.solve();
        assert_eq!(part1.to_string(), "159");
        assert_eq!(part2.to_string(), "610");

        let input = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        let data = Data::try_new(&input).unwrap();
        let Solution { part1, part2 } = data.solve();
        assert_eq!(part1.to_string(), "135");
        assert_eq!(part2.to_string(), "410");
    }
}
