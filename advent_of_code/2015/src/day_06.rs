use std::collections::{HashMap, HashSet};

use crate::AoCData;

#[derive(Debug)]
enum Action {
    On,
    Off,
    Toggle,
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Instruction {
    action: Action,
    top_left: Point,
    bottom_right: Point,
}

pub struct Data(Vec<Instruction>);

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        let mut instructions = Vec::new();
        for line in input.lines() {
            let (first, second) = line.split_once(" through ")?;
            let (x2, y2) = second.split_once(',')?;
            let x2: i32 = x2.parse().ok()?;
            let y2: i32 = y2.parse().ok()?;
            let bottom_right = Point { x: x2, y: y2 };
            let (first, second) = first.rsplit_once(' ')?;
            let (x1, y1) = second.split_once(',')?;
            let x1: i32 = x1.parse().ok()?;
            let y1: i32 = y1.parse().ok()?;
            let top_left = Point { x: x1, y: y1 };
            let action = match first {
                "turn on" => Some(Action::On),
                "turn off" => Some(Action::Off),
                "toggle" => Some(Action::Toggle),
                _ => None,
            }?;
            let instruction = Instruction {
                action,
                top_left,
                bottom_right,
            };
            instructions.push(instruction);
        }
        Some(Self(instructions))
    }

    fn part_1(&self) -> String {
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

        bulbs.len().to_string()
    }

    fn part_2(&self) -> String {
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

        bulbs.values().sum::<u32>().to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(6);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(6);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "");
    }
}
