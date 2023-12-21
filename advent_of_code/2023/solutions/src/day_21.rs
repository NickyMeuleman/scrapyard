use std::{collections::HashSet, fmt::Display};

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Coord {
    col: i64,
    row: i64,
}

impl Coord {
    fn neighbours(&self, rows: i64, cols: i64) -> Vec<Self> {
        let mut res = Vec::new();
        // up
        if self.row > 0 {
            res.push(Coord {
                col: self.col,
                row: self.row - 1,
            });
        }
        // down
        if self.row < rows - 1 {
            res.push(Coord {
                col: self.col,
                row: self.row + 1,
            });
        }
        // left
        if self.col > 0 {
            res.push(Coord {
                col: self.col - 1,
                row: self.row,
            });
        }
        // right
        if self.col < cols - 1 {
            res.push(Coord {
                col: self.col + 1,
                row: self.row,
            })
        };

        res
    }

    fn infinite_neighbours(&self) -> Vec<Self> {
        let mut res = Vec::new();
        // up
        res.push(Coord {
            col: self.col,
            row: self.row - 1,
        });
        // down
        res.push(Coord {
            col: self.col,
            row: self.row + 1,
        });
        // left
        res.push(Coord {
            col: self.col - 1,
            row: self.row,
        });
        // right
        res.push(Coord {
            col: self.col + 1,
            row: self.row,
        });
        res
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    Garden,
    Rock,
}

#[derive(Debug, Clone)]
pub struct Data(Vec<Vec<Tile>>, Coord);

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut start = Coord { col: 0, row: 0 };
        let mut grid = Vec::new();
        for (y, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                let tile = match c {
                    '.' => Tile::Garden,
                    '#' => Tile::Rock,
                    'S' => {
                        start.col = x as i64;
                        start.row = y as i64;
                        Tile::Garden
                    }
                    _ => return Err(AoCError::Parsing),
                };
                row.push(tile);
            }
            grid.push(row);
        }

        Ok(Self(grid, start))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let rows = self.0.len();
        let cols = self.0[0].len();

        let mut set = HashSet::new();
        set.insert(self.1.clone());

        for _ in 0..64 {
            let mut new_set = HashSet::new();
            for pos in set {
                for n in pos
                    .neighbours(rows as i64, cols as i64)
                    .into_iter()
                    .filter(|pos| self.0[pos.row as usize][pos.col as usize] == Tile::Garden)
                {
                    new_set.insert(n);
                }
            }
            set = new_set
        }

        Ok(set.len())
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let goal: u64 = 26_501_365;
        let size = self.0.len() as u64;
        // the amount of steps it takes to reach an edge of the map (all tiles in the same row and column as start are gardens)
        let to_edge = size / 2;

        let mut factors = Vec::new();
        let mut set = HashSet::new();
        set.insert(self.1.clone());

        for count in 1.. {
            let mut new_set = HashSet::new();

            for pos in set {
                for n in pos
                    .infinite_neighbours()
                    .into_iter()
                    .filter(|pos| {
                        let y = pos.row.rem_euclid(size as i64) as usize;
                        let x = pos.col.rem_euclid(size as i64) as usize;
                        self.0[y][x] == Tile::Garden
                    })
                {
                    new_set.insert(n);
                }
            }
            set = new_set;

            if count == to_edge + size * factors.len() as u64 {
                factors.push(set.len() as u64);

                if factors.len() == 3 {
                    let delta0 = factors[0];
                    let delta1 = factors[1] - factors[0];
                    let delta2 = factors[2] - 2 * factors[1] + factors[0];

                    return Ok(delta0
                        + delta1 * (goal / size)
                        + delta2 * ((goal / size) * ((goal / size) - 1) / 2));
                }
            }
        }

        Err(AoCError::Solving)
    }
}
