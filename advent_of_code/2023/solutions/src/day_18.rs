use std::fmt::Display;

use aoc_core::AoCError;
use itertools::Itertools;

use crate::{AoCData, AoCResult};

struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    pub fn advance(&self, direction: &Dir, amount: i64) -> Self {
        match direction {
            Dir::Up => Self {
                x: self.x + amount,
                y: self.y,
            },
            Dir::Down => Self {
                x: self.x - amount,
                y: self.y,
            },
            Dir::Left => Self {
                x: self.x,
                y: self.y - amount,
            },
            Dir::Right => Self {
                x: self.x,
                y: self.y + amount,
            },
        }
    }
}

struct Instr {
    dir: Dir,
    amount: i64,
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn calc_area(mut instructions: impl Iterator<Item = AoCResult<Instr>>) -> AoCResult<i64> {
    let (area, perimeter, _) = instructions.fold_ok(
        (0, 0, Coord { x: 0, y: 0 }),
        |(area, perimeter, pos), Instr { dir, amount }| {
            let new_pos = pos.advance(&dir, amount);
            let new_area = area + (pos.x * new_pos.y - new_pos.x * pos.y);
            let new_perimeter = (new_pos.x - pos.x).abs() + (new_pos.y - pos.y).abs() + perimeter;
            (new_area, new_perimeter, new_pos)
        },
    )?;

    Ok((area.abs() + perimeter) / 2 + 1)
}

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let instructions = self.0.lines().map(|line| {
            let (instr, _) = line
                .split_once(" (")
                .ok_or(AoCError::Parsing)?;
            let (dir, amount) = instr
                .split_once(" ")
                .ok_or(AoCError::Parsing)?;
            let dir = match dir {
                "U" => Dir::Up,
                "D" => Dir::Down,
                "L" => Dir::Left,
                "R" => Dir::Right,
                _ => return Err(AoCError::Parsing),
            };
            let amount = amount
                .parse()
                .map_err(|_| AoCError::Parsing)?;

            Ok(Instr { dir, amount })
        });

        calc_area(instructions)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let instructions = self.0.lines().map(|line| {
            let line = line
                .strip_suffix(")")
                .ok_or(AoCError::Parsing)?;
            let (_, hex) = line
                .split_once("(#")
                .ok_or(AoCError::Parsing)?;
            let (amount, dir) = hex.split_at(5);
            let amount = i64::from_str_radix(amount, 16).map_err(|_| AoCError::Parsing)?;
            let dir = match dir {
                "3" => Dir::Up,
                "1" => Dir::Down,
                "2" => Dir::Left,
                "0" => Dir::Right,
                _ => return Err(AoCError::Parsing),
            };
            Ok(Instr { dir, amount })
        });

        calc_area(instructions)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "62");
    }

    #[test]
    fn part_2() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "952408144115");
    }
}
