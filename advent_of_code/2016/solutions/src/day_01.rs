use std::{collections::HashSet, fmt::Display};

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<Instruction>);

#[derive(Debug, Clone)]
struct Instruction {
    dir: Direction,
    num: i32,
}

struct Me {
    location: (i32, i32),
    facing: Facing,
}

#[derive(Debug, Clone)]
enum Direction {
    L,
    R,
}

impl Me {
    fn turn(&mut self, dir: &Direction) {
        self.facing = match (&self.facing, dir) {
            (Facing::North, Direction::L) => Facing::West,
            (Facing::North, Direction::R) => Facing::East,
            (Facing::East, Direction::L) => Facing::North,
            (Facing::East, Direction::R) => Facing::South,
            (Facing::South, Direction::L) => Facing::East,
            (Facing::South, Direction::R) => Facing::West,
            (Facing::West, Direction::L) => Facing::South,
            (Facing::West, Direction::R) => Facing::North,
        };
    }

    fn forward(&mut self, num: i32) {
        let location = match &self.facing {
            Facing::North => (self.location.0, self.location.1 + num),
            Facing::East => (self.location.0 + num, self.location.1),
            Facing::South => (self.location.0, self.location.1 - num),
            Facing::West => (self.location.0 - num, self.location.1),
        };
        self.location = location;
    }
}
enum Facing {
    North,
    East,
    South,
    West,
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        Ok(Self(
            input
                .trim()
                .split(", ")
                .filter_map(|s| {
                    let (dir, num) = s.split_at(1);
                    let num = num.parse().ok()?;
                    let dir = match dir {
                        "L" => Direction::L,
                        "R" => Direction::R,
                        _ => return None,
                    };
                    Some(Instruction { dir, num })
                })
                .collect(),
        ))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut me = Me {
            facing: Facing::North,
            location: (0, 0),
        };

        for Instruction { dir, num } in &self.0 {
            me.turn(dir);
            me.forward(*num);
        }

        let res = me.location.0.abs() + me.location.1.abs();

        Ok(res)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut me = Me {
            facing: Facing::North,
            location: (0, 0),
        };
        let mut visited = HashSet::new();
        visited.insert(me.location);

        'outer: for Instruction { dir, num } in &self.0 {
            me.turn(dir);
            for _ in 0..*num {
                me.forward(1);
                if !visited.insert(me.location) {
                    break 'outer;
                }
            }
        }

        let res = me.location.0.abs() + me.location.1.abs();

        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "L5, R1, R4, L5, L4, R3, R1, L1, R4, R5, L1, L3, R4, L2, L4, R2, L4, L1, R3, R1, R1, L1, R1, L5, R5, R2, L5, R2, R1, L2, L4, L4, R191, R2, R5, R1, L1, L2, R5, L2, L3, R4, L1, L1, R1, R50, L1, R1, R76, R5, R4, R2, L5, L3, L5, R2, R1, L1, R2, L3, R4, R2, L1, L1, R4, L1, L1, R185, R1, L5, L4, L5, L3, R2, R3, R1, L5, R1, L3, L2, L2, R5, L1, L1, L3, R1, R4, L2, L1, L1, L3, L4, R5, L2, R3, R5, R1, L4, R5, L3, R3, R3, R1, R1, R5, R2, L2, R5, L5, L4, R4, R3, R5, R1, L3, R1, L2, L2, R3, R4, L1, R4, L1, R4, R3, L1, L4, L1, L5, L2, R2, L1, R1, L5, L3, R4, L1, R5, L5, L5, L1, L3, R1, R5, L2, L4, L5, L1, L1, L2, R5, R5, L4, R3, L2, L1, L3, L4, L5, L5, L2, R4, R3, L5, R4, R2, R1, L5";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "242");
    }

    #[test]
    fn part_2() {
        let input = "L5, R1, R4, L5, L4, R3, R1, L1, R4, R5, L1, L3, R4, L2, L4, R2, L4, L1, R3, R1, R1, L1, R1, L5, R5, R2, L5, R2, R1, L2, L4, L4, R191, R2, R5, R1, L1, L2, R5, L2, L3, R4, L1, L1, R1, R50, L1, R1, R76, R5, R4, R2, L5, L3, L5, R2, R1, L1, R2, L3, R4, R2, L1, L1, R4, L1, L1, R185, R1, L5, L4, L5, L3, R2, R3, R1, L5, R1, L3, L2, L2, R5, L1, L1, L3, R1, R4, L2, L1, L1, L3, L4, R5, L2, R3, R5, R1, L4, R5, L3, R3, R3, R1, R1, R5, R2, L2, R5, L5, L4, R4, R3, R5, R1, L3, R1, L2, L2, R3, R4, L1, R4, L1, R4, R3, L1, L4, L1, L5, L2, R2, L1, R1, L5, L3, R4, L1, R5, L5, L5, L1, L3, R1, R5, L2, L4, L5, L1, L1, L2, R5, R5, L4, R3, L2, L1, L3, L4, L5, L5, L2, R4, R3, L5, R4, R2, R1, L5";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "150");
    }
}
