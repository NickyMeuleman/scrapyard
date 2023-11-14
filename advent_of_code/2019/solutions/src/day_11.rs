use std::{collections::HashMap, fmt::Display};

use aoc_core::AoCError;

use crate::{
    intcode::{Computer, Status},
    AoCData, AoCResult,
};

#[derive(Debug, Clone)]
pub struct Data(Vec<i64>);

#[derive(Clone, Hash, Copy, PartialEq)]
enum Panel {
    Black,
    White,
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Clone, Hash)]
enum Face {
    Up,
    Left,
    Right,
    Down,
}

#[derive(Clone, Hash, Copy)]
enum Turn {
    Left,
    Right,
}

struct Robot {
    computer: Computer,
    position: Coord,
    facing: Face,
}

impl Robot {
    fn new() -> Self {
        Self {
            computer: Default::default(),
            position: Coord { x: 0, y: 0 },
            facing: Face::Up,
        }
    }

    fn forward(&mut self) {
        match self.facing {
            Face::Up => self.position.y -= 1,
            Face::Left => self.position.x -= 1,
            Face::Right => self.position.x += 1,
            Face::Down => self.position.y += 1,
        }
    }

    fn turn(&mut self, turn: &Turn) {
        let new_face = match (&self.facing, turn) {
            (Face::Up, Turn::Left) => Face::Left,
            (Face::Up, Turn::Right) => Face::Right,
            (Face::Left, Turn::Left) => Face::Down,
            (Face::Left, Turn::Right) => Face::Up,
            (Face::Right, Turn::Left) => Face::Up,
            (Face::Right, Turn::Right) => Face::Down,
            (Face::Down, Turn::Left) => Face::Right,
            (Face::Down, Turn::Right) => Face::Left,
        };
        self.facing = new_face;
    }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        Ok(Self(
            input
                .split(',')
                .filter_map(|s| s.parse().ok())
                .collect(),
        ))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut robot = Robot::new();
        let mut hull = HashMap::new();

        robot
            .computer
            .set_memory(self.0.clone());

        'outer: loop {
            let color = *hull
                .get(&robot.position)
                .unwrap_or(&Panel::Black);
            let input = if color == Panel::Black { 0 } else { 1 };
            robot.computer.input(input);
            while let Ok(status) = robot.computer.operate() {
                match status {
                    Status::Halted => break 'outer,
                    _ => {
                        if robot.computer.outputs.len() == 2 {
                            break;
                        }
                    }
                }
            }

            let new_color = match robot.computer.consume_output() {
                Some(0) => Panel::Black,
                Some(1) => Panel::White,
                _ => return Err(AoCError::Solving),
            };
            let direction = match robot.computer.consume_output() {
                Some(0) => Turn::Left,
                Some(1) => Turn::Right,
                _ => return Err(AoCError::Solving),
            };

            hull.insert(robot.position.clone(), new_color);
            robot.turn(&direction);
            robot.forward();
        }

        Ok(hull.len())
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut robot = Robot::new();
        let mut hull = HashMap::new();

        robot
            .computer
            .set_memory(self.0.clone());
        hull.insert(robot.position.clone(), Panel::White);

        'outer: loop {
            let color = *hull
                .get(&robot.position)
                .unwrap_or(&Panel::Black);
            let input = if color == Panel::Black { 0 } else { 1 };
            robot.computer.input(input);
            while let Ok(status) = robot.computer.operate() {
                match status {
                    Status::Halted => break 'outer,
                    _ => {
                        if robot.computer.outputs.len() == 2 {
                            break;
                        }
                    }
                }
            }

            let new_color = match robot.computer.consume_output() {
                Some(0) => Panel::Black,
                Some(1) => Panel::White,
                _ => return Err(AoCError::Solving),
            };
            let direction = match robot.computer.consume_output() {
                Some(0) => Turn::Left,
                Some(1) => Turn::Right,
                _ => return Err(AoCError::Solving),
            };

            hull.insert(robot.position.clone(), new_color);
            robot.turn(&direction);
            robot.forward();
        }

        let (max_x, max_y) = hull
            .keys()
            .fold((0, 0), |(max_x, max_y), Coord { x, y }| {
                (max_x.max(*x), max_y.max(*y))
            });
        let mut result = String::new();
        for y in 0..=max_y {
            for x in 0..=max_x {
                let color = hull
                    .get(&Coord { x, y })
                    .unwrap_or(&Panel::Black);
                match color {
                    Panel::Black => result.push(' '),
                    Panel::White => result.push('■'),
                }
            }
            result.push('\n');
        }
        Ok(result)
    }
}
