use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(usize, usize, HashSet<Coord>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coord {
    row: i32,
    col: i32,
}

impl Coord {
    fn turn(&mut self, turn: &Turn) {
        match turn {
            Turn::Left => {
                // -1,0 U 0,-1 L
                // 0,-1 L 1,0 D
                // 1,0 D 0,1 R
                // 0,1 R -1,0 U
                let tmp = self.col;
                self.col = self.row;
                self.row = -tmp;
            }
            Turn::Right => {
                // -1,0 U 0,1 R
                // 0,-1 L -1,0 U
                // 1,0 D 0,-1 L
                // 0,1 R 1,0 D
                let tmp = self.row;
                self.row = self.col;
                self.col = -tmp;
            }
        }
    }

    fn forward(&mut self, dir: &Self) {
        self.row += dir.row;
        self.col += dir.col;
    }
}

enum Turn {
    Left,
    Right,
}

fn burst(carrier_pos: &mut Coord, carrier_dir: &mut Coord, map: &mut HashSet<Coord>) -> bool {
    let mut infected = false;
    // turn
    let turn = if map.contains(carrier_pos) {
        Turn::Right
    } else {
        Turn::Left
    };
    carrier_dir.turn(&turn);
    // toggle
    if map.contains(carrier_pos) {
        map.remove(carrier_pos);
    } else {
        map.insert(*carrier_pos);
        infected = true;
    }
    // forward
    carrier_pos.forward(carrier_dir);

    infected
}

enum Kind {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

fn burst2(
    carrier_pos: &mut Coord,
    carrier_dir: &mut Coord,
    map: &mut HashMap<Coord, Kind>,
) -> bool {
    let mut infected = false;
    // turn
    let kind = map
        .get(carrier_pos)
        .unwrap_or(&Kind::Clean);
    match kind {
        Kind::Clean => carrier_dir.turn(&Turn::Left),
        Kind::Weakened => {}
        Kind::Infected => carrier_dir.turn(&Turn::Right),
        Kind::Flagged => {
            // reverse
            carrier_dir.turn(&Turn::Left);
            carrier_dir.turn(&Turn::Left);
        }
    }
    // modify
    let new = match kind {
        Kind::Clean => Kind::Weakened,
        Kind::Weakened => {
            infected = true;
            Kind::Infected
        }
        Kind::Infected => Kind::Flagged,
        Kind::Flagged => Kind::Clean,
    };
    map.insert(*carrier_pos, new);
    // forward
    carrier_pos.forward(&carrier_dir);
    infected
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let map = input
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_col, c)| *c == '#')
                    .map(move |(col, _c)| Coord {
                        row: row as i32,
                        col: col as i32,
                    })
            })
            .collect();
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();

        Ok(Self(width, height, map))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let width = self.0;
        let height = self.1;
        let mut map = self.2.clone();
        let mut carrier_coord = Coord {
            row: height as i32 / 2,
            col: width as i32 / 2,
        };
        let mut carrier_dir = Coord { row: -1, col: 0 };
        let mut count = 0;
        for _ in 0..10_000 {
            if burst(&mut carrier_coord, &mut carrier_dir, &mut map) {
                count += 1;
            }
        }
        Ok(count)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let width = self.0;
        let height = self.1;
        let map = &self.2;
        let mut map: HashMap<Coord, Kind> = map
            .into_iter()
            .map(|coord| (*coord, Kind::Infected))
            .collect();
        let mut carrier_coord = Coord {
            row: height as i32 / 2,
            col: width as i32 / 2,
        };
        let mut carrier_dir = Coord { row: -1, col: 0 };
        let mut count = 0;
        for _ in 0..10_000_000 {
            if burst2(&mut carrier_coord, &mut carrier_dir, &mut map) {
                count += 1;
            }
        }
        Ok(count)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "..#
#..
...";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "5587");
    }

    #[test]
    fn part_2() {
        let input = "..#
#..
...";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "2511944");
    }
}
