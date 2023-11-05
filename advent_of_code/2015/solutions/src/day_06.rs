use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<Instruction>);

#[derive(Debug, Clone)]
enum Action {
    On,
    Off,
    Toggle,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Instruction {
    action: Action,
    top_left: Point,
    bottom_right: Point,
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut instructions = Vec::new();
        for line in input.lines() {
            let (first, second) = line
                .split_once(" through ")
                .ok_or(AoCError::Parsing)?;
            let (x2, y2) = second
                .split_once(',')
                .ok_or(AoCError::Parsing)?;
            let x2: i32 = x2.parse()?;
            let y2: i32 = y2.parse()?;
            let bottom_right = Point { x: x2, y: y2 };
            let (first, second) = first
                .rsplit_once(' ')
                .ok_or(AoCError::Parsing)?;
            let (x1, y1) = second
                .split_once(',')
                .ok_or(AoCError::Parsing)?;
            let x1: i32 = x1.parse()?;
            let y1: i32 = y1.parse()?;
            let top_left = Point { x: x1, y: y1 };
            let action = match first {
                "turn on" => Ok(Action::On),
                "turn off" => Ok(Action::Off),
                "toggle" => Ok(Action::Toggle),
                _ => Err(AoCError::Parsing),
            }?;
            let instruction = Instruction {
                action,
                top_left,
                bottom_right,
            };
            instructions.push(instruction);
        }
        Ok(Self(instructions))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut bulbs: HashSet<Point> = HashSet::new();
        for ins in self.0.iter() {
            match ins.action {
                Action::Off => {
                    for x in ins.top_left.x..=ins.bottom_right.x {
                        for y in ins.top_left.y..=ins.bottom_right.y {
                            bulbs.remove(&Point { x, y });
                        }
                    }
                }
                Action::On => {
                    for x in ins.top_left.x..=ins.bottom_right.x {
                        for y in ins.top_left.y..=ins.bottom_right.y {
                            bulbs.insert(Point { x, y });
                        }
                    }
                }
                Action::Toggle => {
                    for x in ins.top_left.x..=ins.bottom_right.x {
                        for y in ins.top_left.y..=ins.bottom_right.y {
                            let point = Point { x, y };
                            if bulbs.contains(&point) {
                                bulbs.remove(&point);
                            } else {
                                bulbs.insert(Point { x, y });
                            }
                        }
                    }
                }
            }
        }

        Ok(bulbs.len())
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut bulbs: HashMap<Point, u32> = HashMap::new();
        for ins in self.0.iter() {
            match ins.action {
                Action::Off => {
                    for x in ins.top_left.x..=ins.bottom_right.x {
                        for y in ins.top_left.y..=ins.bottom_right.y {
                            bulbs
                                .entry(Point { x, y })
                                .and_modify(|n| *n = n.saturating_sub(1));
                        }
                    }
                }
                Action::On => {
                    for x in ins.top_left.x..=ins.bottom_right.x {
                        for y in ins.top_left.y..=ins.bottom_right.y {
                            bulbs
                                .entry(Point { x, y })
                                .and_modify(|n| *n += 1)
                                .or_insert(1);
                        }
                    }
                }
                Action::Toggle => {
                    for x in ins.top_left.x..=ins.bottom_right.x {
                        for y in ins.top_left.y..=ins.bottom_right.y {
                            bulbs
                                .entry(Point { x, y })
                                .and_modify(|n| *n += 2)
                                .or_insert(2);
                        }
                    }
                }
            }
        }

        Ok(bulbs.values().sum::<u32>())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "turn on 0,0 through 999,999
toggle 0,0 through 999,0
turn off 499,499 through 500,500";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, (1000 * 1000 - 1000 - 4).to_string());
    }

    #[test]
    fn part_2() {
        let input = "turn on 0,0 through 0,0
toggle 0,0 through 999,999";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, (1 + 2000000).to_string());
    }
}
