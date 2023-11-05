use std::fmt::Display;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<Vec<Cell>>);

#[derive(PartialEq, Debug, Clone)]
enum Cell {
    Tree,
    Empty,
}

fn traverse_map(map: &Vec<Vec<Cell>>, slope: (usize, usize)) -> u32 {
    let map_height = map.len();
    let map_width = map[0].len();
    // the / operator gives the rounded down integer, if the result is 2.999999999 it will become 2
    let steps: usize = map_height / slope.1;
    let mut count = 0;
    for num in 0..steps {
        let column_idx: usize = (num * slope.0) % map_width;
        let row_idx: usize = num * slope.1;
        let cell = &map[row_idx][column_idx];
        if *cell == Cell::Tree {
            count += 1;
        }
    }
    count
}

fn traverse_map_2(map: &Vec<Vec<Cell>>, slope: (usize, usize)) -> u32 {
    let map_height = map.len();
    let map_width = map[0].len();
    // the / operator gives the rounded down integer, if the result is 2.999999999 it will become 2
    let steps = map_height / slope.1;
    (0..steps)
        .map(|num| {
            let column_idx = (num * slope.0) % map_width;
            let row_idx = num * slope.1;
            &map[row_idx][column_idx]
        })
        .filter(|&cell| *cell == Cell::Tree)
        .count()
        .try_into()
        .unwrap()
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        fn parse_line(line: &str) -> Vec<Cell> {
            line.chars()
                .map(|c| parse_char(c))
                .collect()
        }
        fn parse_char(c: char) -> Cell {
            match c {
                '#' => Cell::Tree,
                _ => Cell::Empty,
            }
        }
        let res = input
            .lines()
            .map(|line| parse_line(line))
            .collect();
        Ok(Self(res))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        Ok(traverse_map(&self.0, (3, 1)))
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let slopes: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        // overflowing i32, ohno
        let result: u64 = slopes
            .iter()
            .map(|&slope| traverse_map_2(&self.0, slope))
            .map(|num| u64::from(num))
            .product();

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "7");
    }

    #[test]
    fn part_2() {
        let input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "336");
    }
}
