use std::collections::HashSet;

use itertools::Itertools;

use crate::AoCData;

pub struct Data(Vec<Instruction>);

struct Instruction {
    dir: Direction,
    amount: u8,
}

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

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        let mut instructions = Vec::new();
        for line in input.lines() {
            let (dir, amount) = line.split_once(' ')?;
            let dir = match dir {
                "U" => Direction::U,
                "D" => Direction::D,
                "L" => Direction::L,
                "R" => Direction::R,
                _ => return None,
            };
            let amount = amount.parse().ok()?;
            instructions.push(Instruction { dir, amount });
        }
        Some(Self(instructions))
    }

    fn part_1(&self) -> String {
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

        seen.len().to_string()
    }

    fn part_2(&self) -> String {
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

        seen.len().to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(9);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "13");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(9);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "1");
    }
}
