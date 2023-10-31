use std::{collections::HashSet, fmt::Display};

use aoc_core::AoCError;
use itertools::Itertools;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<Instruction>);

#[derive(Debug, Clone)]
struct Instruction {
    dir: Direction,
    amount: u8,
}

#[derive(Debug, Clone)]
enum Direction {
    L,
    R,
    U,
    D,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Coord {
    x: isize,
    y: isize,
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut instructions = Vec::new();
        for line in input.lines() {
            let (dir, amount) = line
                .split_once(' ')
                .ok_or(AoCError::Parsing)?;
            let dir = match dir {
                "U" => Direction::U,
                "D" => Direction::D,
                "L" => Direction::L,
                "R" => Direction::R,
                _ => return Err(AoCError::Parsing),
            };
            let amount = amount.parse()?;
            instructions.push(Instruction { dir, amount });
        }
        Ok(Self(instructions))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut head = Coord { x: 0, y: 0 };
        let mut tail = Coord { x: 0, y: 0 };
        let mut seen = HashSet::new();
        seen.insert(tail);

        for Instruction { dir, amount } in &self.0 {
            for _ in 0..*amount {
                // move head
                match dir {
                    Direction::U => head.y -= 1,
                    Direction::D => head.y += 1,
                    Direction::L => head.x -= 1,
                    Direction::R => head.x += 1,
                };

                // catch up tail if needed
                let diff = Coord {
                    x: head.x - tail.x,
                    y: head.y - tail.y,
                };
                if diff.x.abs() > 1 || diff.y.abs() > 1 {
                    tail.x += diff.x.signum();
                    tail.y += diff.y.signum();
                    seen.insert(tail);
                }
            }
        }

        Ok(seen.len())
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let start = Coord { x: 0, y: 0 };
        let mut rope = vec![start; 10];
        let mut seen = HashSet::new();
        seen.insert(start);

        for Instruction { dir, amount } in &self.0 {
            for _ in 0..*amount {
                // move head of the whole rope
                match dir {
                    Direction::U => rope[0].y -= 1,
                    Direction::D => rope[0].y += 1,
                    Direction::L => rope[0].x -= 1,
                    Direction::R => rope[0].x += 1,
                };

                // move the rest of the rope
                for (head_idx, tail_idx) in (0..rope.len()).tuple_windows() {
                    let diff = Coord {
                        x: rope[head_idx].x - rope[tail_idx].x,
                        y: rope[head_idx].y - rope[tail_idx].y,
                    };
                    let not_touching = diff.x.abs() > 1 || diff.y.abs() > 1;
                    if not_touching {
                        rope[tail_idx].x += diff.x.signum();
                        rope[tail_idx].y += diff.y.signum();
                        if tail_idx == rope.len() - 1 {
                            seen.insert(rope[rope.len() - 1]);
                        }
                    }
                }

                // alternative that doesn't iterate over pairs of indexes
                // // move the rest of the rope
                // for idx in 1..rope.len() {
                //     let diff = Coord {
                //         x: rope[idx - 1].x - rope[idx].x,
                //         y: rope[idx - 1].y - rope[idx].y,
                //     };
                //     let not_touching = diff.x.abs() > 1 || diff.y.abs() > 1;
                //     if not_touching {
                //         rope[idx].x += diff.x.signum();
                //         rope[idx].y += diff.y.signum();
                //         if idx == rope.len() - 1 {
                //             seen.insert(rope[rope.len() - 1]);
                //         }
                //     }
                // }
            }
        }

        Ok(seen.len())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "13");
    }

    #[test]
    fn part_2() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "1");
    }
}
