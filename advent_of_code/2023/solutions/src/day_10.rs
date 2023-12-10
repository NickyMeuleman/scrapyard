use std::{collections::HashSet, fmt::Display};

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    // | is a vertical pipe connecting north and south.
    NorthSouth,
    // - is a horizontal pipe connecting east and west.
    EastWest,
    // L is a 90-degree bend connecting north and east.
    NorthEast,
    // J is a 90-degree bend connecting north and west.
    NorthWest,
    // 7 is a 90-degree bend connecting south and west.
    SouthWest,
    // F is a 90-degree bend connecting south and east.
    SouthEast,
    // . is ground; there is no pipe in this tile.
    Ground,
    // S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
    Start,
}
use aoc_core::AoCError;
use Tile::*;
impl Tile {
    fn try_from(c: char) -> AoCResult<Self> {
        match c {
            '|' => Ok(NorthSouth),
            '-' => Ok(EastWest),
            'L' => Ok(NorthEast),
            'J' => Ok(NorthWest),
            '7' => Ok(SouthWest),
            'F' => Ok(SouthEast),
            '.' => Ok(Ground),
            'S' => Ok(Start),
            _ => Err(AoCError::Parsing),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coord {
    row_idx: usize,
    col_idx: usize,
}

impl Coord {
    fn new(row_idx: usize, col_idx: usize) -> Self {
        Self { row_idx, col_idx }
    }

    fn valid_neighbours(&self, map: &[Vec<Tile>]) -> Vec<Coord> {
        let mut neighbours = vec![];
        let max_height = map.len() - 1;
        let max_width = map[0].len() - 1;

        match map[self.row_idx][self.col_idx] {
            Ground => (),
            Start => {
                // north
                if self.row_idx > 0 {
                    let tile = map[self.row_idx - 1][self.col_idx];
                    if matches!(tile, NorthSouth | SouthWest | SouthEast) {
                        neighbours.push(Coord::new(self.row_idx - 1, self.col_idx));
                    }
                }
                // south
                if self.row_idx < max_height {
                    let tile = map[self.row_idx + 1][self.col_idx];
                    if matches!(tile, NorthSouth | NorthWest | NorthEast) {
                        neighbours.push(Coord::new(self.row_idx + 1, self.col_idx))
                    }
                }
                // west
                if self.col_idx > 0 {
                    let tile = map[self.row_idx][self.col_idx - 1];
                    if matches!(tile, EastWest | SouthEast | NorthEast) {
                        neighbours.push(Coord::new(self.row_idx, self.col_idx - 1))
                    }
                }
                // east
                if self.col_idx < max_width {
                    let tile = map[self.row_idx][self.col_idx + 1];
                    if matches!(tile, EastWest | NorthWest | SouthWest) {
                        neighbours.push(Coord::new(self.row_idx, self.col_idx + 1))
                    }
                }
            }
            NorthSouth => {
                // north
                if self.row_idx > 0 {
                    match map[self.row_idx - 1][self.col_idx] {
                        NorthSouth => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        SouthWest => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        SouthEast => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        Start => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        _ => (),
                    }
                }
                // south
                if self.row_idx < max_height && map[self.row_idx + 1][self.col_idx] != Ground {
                    match map[self.row_idx + 1][self.col_idx] {
                        NorthSouth => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        NorthWest => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        NorthEast => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        Start => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        _ => (),
                    }
                }
            }
            EastWest => {
                // west
                if self.col_idx > 0 {
                    match map[self.row_idx][self.col_idx - 1] {
                        EastWest => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        SouthEast => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        NorthEast => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        Start => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        _ => (),
                    }
                }
                // east
                if self.col_idx < max_width {
                    match map[self.row_idx][self.col_idx + 1] {
                        EastWest => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        NorthWest => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        SouthWest => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        Start => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        _ => (),
                    }
                }
            }
            NorthEast => {
                // north
                if self.row_idx > 0 {
                    match map[self.row_idx - 1][self.col_idx] {
                        NorthSouth => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        SouthWest => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        SouthEast => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        Start => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        _ => (),
                    }
                }
                // east
                if self.col_idx < max_width {
                    match map[self.row_idx][self.col_idx + 1] {
                        EastWest => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        NorthWest => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        SouthWest => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        Start => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        _ => (),
                    }
                }
            }
            NorthWest => {
                // north
                if self.row_idx > 0 {
                    match map[self.row_idx - 1][self.col_idx] {
                        NorthSouth => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        SouthWest => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        SouthEast => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        Start => neighbours.push(Coord::new(self.row_idx - 1, self.col_idx)),
                        _ => (),
                    }
                }
                // west
                if self.col_idx > 0 {
                    match map[self.row_idx][self.col_idx - 1] {
                        EastWest => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        SouthEast => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        NorthEast => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        Start => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        _ => (),
                    }
                }
            }
            SouthWest => {
                // south
                if self.row_idx < max_height {
                    match map[self.row_idx + 1][self.col_idx] {
                        NorthSouth => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        NorthWest => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        NorthEast => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        Start => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        _ => (),
                    }
                }
                // west
                if self.col_idx > 0 {
                    match map[self.row_idx][self.col_idx - 1] {
                        EastWest => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        SouthEast => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        NorthEast => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        Start => neighbours.push(Coord::new(self.row_idx, self.col_idx - 1)),
                        _ => (),
                    }
                }
            }
            SouthEast => {
                // south
                if self.row_idx < max_height {
                    match map[self.row_idx + 1][self.col_idx] {
                        NorthSouth => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        NorthWest => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        NorthEast => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        Start => neighbours.push(Coord::new(self.row_idx + 1, self.col_idx)),
                        _ => (),
                    }
                }
                // east
                if self.col_idx < max_width {
                    match map[self.row_idx][self.col_idx + 1] {
                        EastWest => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        NorthWest => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        SouthWest => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        Start => neighbours.push(Coord::new(self.row_idx, self.col_idx + 1)),
                        _ => (),
                    }
                }
            }
        }

        neighbours
    }
}

#[derive(Debug, Clone)]
pub struct Data(Vec<Vec<Tile>>, Coord);

fn build_loop(start: Coord, map: &[Vec<Tile>]) -> HashSet<Coord> {
    let mut loop_coords = HashSet::new();
    loop_coords.insert(start);
    let mut to_visit = start.valid_neighbours(map);

    while let Some(curr_pos) = to_visit.pop() {
        for neighbour in curr_pos.valid_neighbours(map) {
            if !loop_coords.contains(&neighbour) {
                to_visit.push(neighbour);
                loop_coords.insert(neighbour);
            }
        }
    }

    loop_coords
}

fn get_start_pipe(map: &Vec<Vec<Tile>>, start: Coord) -> AoCResult<Tile> {
    let neighbours = start.valid_neighbours(map);
    let north = neighbours
        .iter()
        .find(|coord| coord.row_idx < start.row_idx)
        .is_some();
    let south = neighbours
        .iter()
        .find(|coord| coord.row_idx > start.row_idx)
        .is_some();
    let west = neighbours
        .iter()
        .find(|coord| coord.col_idx < start.col_idx)
        .is_some();
    let east = neighbours
        .iter()
        .find(|coord| coord.col_idx > start.col_idx)
        .is_some();

    match (north, west, south, east) {
        (true, true, _, _) => Ok(NorthWest),
        (true, _, true, _) => Ok(NorthSouth),
        (true, _, _, true) => Ok(NorthEast),
        (_, true, true, _) => Ok(SouthWest),
        (_, _, true, true) => Ok(SouthEast),
        (_, true, _, true) => Ok(EastWest),
        _ => Err(AoCError::Solving),
    }
}

/// replace start with a valid pipe segment, and only keep pipe segments that are part of the loop
fn clean_map(
    start: Coord,
    loop_coords: &HashSet<Coord>,
    map: Vec<Vec<Tile>>,
) -> AoCResult<Vec<Vec<Tile>>> {
    let start_pipe = get_start_pipe(&map, start)?;

    map.into_iter()
        .enumerate()
        .map(|(row_idx, line)| {
            line.into_iter()
                .enumerate()
                .map(|(col_idx, tile)| {
                    Ok(match tile {
                        Start => start_pipe,
                        pipe if loop_coords.contains(&Coord::new(row_idx, col_idx)) => pipe,
                        _ => Ground,
                    })
                })
                .collect()
        })
        .collect()
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut start = Coord::new(0, 0);
        let map = input
            .lines()
            .enumerate()
            .map(|(row_idx, line)| {
                let line = line
                    .chars()
                    .enumerate()
                    .map(|(col_idx, c)| {
                        let tile = Tile::try_from(c)?;
                        if tile == Start {
                            start = Coord::new(row_idx, col_idx)
                        }
                        Ok(tile)
                    })
                    .collect::<AoCResult<Vec<Tile>>>()?;
                Ok(line)
            })
            .collect::<AoCResult<Vec<Vec<Tile>>>>()?;

        Ok(Self(map, start))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let loop_coords = build_loop(self.1, &self.0);
        Ok(loop_coords.len() / 2)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let loop_coords = build_loop(self.1, &self.0);
        let map = clean_map(self.1, &loop_coords, self.0.clone())?;
        // scan from top to bottom and left to right, counting how many tiles are inside the loop.
        // keep track of a boolean that tells me if I'm inside the loop
        // every time I cross a vertical pipe that does not horizontally block the top (the place where I am in the loop), flip that state
        let mut inside = false;
        let count = map
            .into_iter()
            .flatten()
            .filter(|tile| match tile {
                Ground => inside,
                NorthSouth | NorthWest | NorthEast => {
                    inside = !inside;
                    false
                }
                _ => false,
            })
            .count();

        Ok(count)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "8");
    }

    #[test]
    fn part_2() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "10");
    }
}
