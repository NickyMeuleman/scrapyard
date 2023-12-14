use std::fmt::Display;

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Tile {
    Round,
    Square,
    Empty,
}

#[derive(Debug, Clone)]
pub struct Data(Vec<Vec<Tile>>);

fn slide_north(grid: &mut Vec<Vec<Tile>>) {
    for col in 0..grid[0].len() {
        let mut empty_or_round_row = 0;
        for row in 0..grid.len() {
            let curr = grid[row][col];
            match curr {
                Tile::Square => empty_or_round_row = row + 1,
                Tile::Round => {
                    // swap the current tile with the empty_or_round one
                    let replace_with = std::mem::replace(&mut grid[empty_or_round_row][col], curr);
                    let _ = std::mem::replace(&mut grid[row][col], replace_with);
                    empty_or_round_row += 1;
                }
                Tile::Empty => (),
            }
        }
    }
}

fn weight(grid: &Vec<Vec<Tile>>) -> usize {
    grid.iter()
        .rev()
        .enumerate()
        .map(|(i, row)| {
            let round_rocks = row
                .iter()
                .filter(|&tile| *tile == Tile::Round)
                .count();
            round_rocks * (i + 1)
        })
        .sum()
}

// rotate 90 degrees clockwise: (x, y) -> (y, -x)
fn clockwise(grid: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let size = grid.len();
    let mut rotated = vec![vec![Tile::Empty; size]; size];
    for row in 0..size {
        for col in 0..size {
            rotated[col][size - 1 - row] = grid[row][col];
        }
    }
    rotated
}

fn cycle(mut grid: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    for _ in 0..4 {
        slide_north(&mut grid);
        let rotated = clockwise(&grid);
        grid = rotated;
    }
    grid
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let grid = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Ok(Tile::Empty),
                        '#' => Ok(Tile::Square),
                        'O' => Ok(Tile::Round),
                        _ => Err(AoCError::Parsing),
                    })
                    .collect::<AoCResult<Vec<_>>>()
            })
            .collect::<AoCResult<Vec<_>>>()?;

        Ok(Self(grid))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut grid = self.0.clone();
        slide_north(&mut grid);
        Ok(weight(&grid))
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut grid = self.0.clone();
        let mut seen = vec![grid.clone()];

        loop {
            grid = cycle(grid);
            // check if the cycled map has already been seen
            if let Some(idx) = seen.iter().position(|x| x == &grid) {
                // figure out length of cycle (watch out: a cycle might only start after a number of steps)
                let cycle_length = seen.len() - idx;
                // use cycle length to figure out the index of the final step in the seen list
                let final_idx = idx + (1_000_000_000 - idx) % cycle_length;
                return Ok(weight(&seen[final_idx]));
            }
            seen.push(grid.clone());
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "136");
    }

    #[test]
    fn part_2() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "64");
    }
}
