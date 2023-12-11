use std::fmt::Display;

use aoc_core::AoCError;
use itertools::Itertools;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Galaxy,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coord {
    row: u64,
    col: u64,
}

impl Coord {
    fn manhattan_dist(&self, other: &Self) -> u64 {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }
}

#[derive(Debug, Clone)]
pub struct Data(Vec<Vec<Tile>>);

fn empty_rows(grid: &[Vec<Tile>]) -> Vec<usize> {
    grid.iter()
        .enumerate()
        .filter_map(|(idx, row)| {
            if !row.contains(&Tile::Galaxy) {
                Some(idx)
            } else {
                None
            }
        })
        .collect()
}

fn empty_cols(grid: &[Vec<Tile>]) -> Vec<usize> {
    // this song and dance is only here so I can loop over columns
    let mut cols: Vec<Vec<Tile>> = vec![vec![Tile::Empty; grid.len()]; grid[0].len()];
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, c) in row.iter().enumerate() {
            cols[col_idx][row_idx] = *c;
        }
    }

    empty_rows(&cols)
}

fn galaxy_coordinates(grid: &[Vec<Tile>], expansion: u64) -> Vec<Coord> {
    let empty_rows = empty_rows(&grid);
    let empty_cols = empty_cols(&grid);

    let mut galaxies = Vec::new();
    let mut curr_row = 0;
    let mut curr_col = 0;

    for (row_idx, row) in grid.iter().enumerate() {
        if empty_rows.contains(&row_idx) {
            curr_row += expansion;
            continue;
        }
        for (col_idx, c) in row.iter().enumerate() {
            if empty_cols.contains(&col_idx) {
                curr_col += expansion;
                continue;
            }
            if *c == Tile::Galaxy {
                galaxies.push(Coord {
                    row: curr_row,
                    col: curr_col,
                });
            }
            curr_col += 1
        }
        curr_col = 0;
        curr_row += 1;
    }

    galaxies
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let map = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Ok(Tile::Empty),
                        '#' => Ok(Tile::Galaxy),
                        _ => Err(AoCError::Parsing),
                    })
                    .collect()
            })
            .collect::<AoCResult<Vec<Vec<Tile>>>>()?;

        Ok(Self(map))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let galaxies = galaxy_coordinates(&self.0, 2);

        let sum: u64 = galaxies
            .iter()
            .combinations(2)
            .map(|pair| pair[0].manhattan_dist(pair[1]))
            .sum();

        Ok(sum)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let galaxies = galaxy_coordinates(&self.0, 1_000_000);

        let sum: u64 = galaxies
            .iter()
            .combinations(2)
            .map(|pair| pair[0].manhattan_dist(pair[1]))
            .sum();

        Ok(sum)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "374");
    }

    #[test]
    fn part_2_10times() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let data = Data::try_new(input).unwrap();
        let galaxies = galaxy_coordinates(&data.0, 10);

        let sum: u64 = galaxies
            .iter()
            .combinations(2)
            .map(|pair| pair[0].manhattan_dist(pair[1]))
            .sum();
        assert_eq!(sum, 1030);
    }

    #[test]
    fn part_2_100times() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let data = Data::try_new(input).unwrap();
        let galaxies = galaxy_coordinates(&data.0, 100);

        let sum: u64 = galaxies
            .iter()
            .combinations(2)
            .map(|pair| pair[0].manhattan_dist(pair[1]))
            .sum();
        assert_eq!(sum, 8410);
    }
}
