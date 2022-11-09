use std::collections::HashSet;

use crate::AoCData;

pub struct Data(Vec<Instruction>);

struct Instruction {
    dir: Direction,
    num: i32,
}

struct Me {
    location: (i32, i32),
    facing: Facing,
}

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

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        Some(Self(
            input
                .trim()
                .split(", ")
                .filter_map(|s| {
                    let (dir, num) = s.split_at(1);
                    let num = num.parse().ok()?;
                    let dir = match dir {
                        "L" => Direction::L,
                        "R" => Direction::R,
                        _ => panic!("Invalid input"),
                    };
                    Some(Instruction { dir, num })
                })
                .collect(),
        ))
    }

    fn part_1(&self) -> String {
        let mut me = Me {
            facing: Facing::North,
            location: (0, 0),
        };

        for Instruction { dir, num } in &self.0 {
           me.turn(dir);
           me.forward(*num);
        }

        let res = me.location.0.abs() + me.location.1.abs();
        res.to_string()
    }

    fn part_2(&self) -> String {
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
        res.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_input(1);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "242");
    }

    #[test]
    fn part_2() {
        let input = utils::get_input(1);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "150");
    }
}
