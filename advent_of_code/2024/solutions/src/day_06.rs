use crate::{AoCData, AoCResult};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

fn parse(input: &str) -> ((isize, isize), Vec<Vec<char>>) {
    let mut grid = Vec::new();
    let mut start = (0, 0);
    for (row_idx, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (col_idx, mut c) in line.chars().enumerate() {
            if c == '^' {
                start = (row_idx as isize, col_idx as isize);
                c = '.';
            }
            row.push(c);
        }
        grid.push(row);
    }
    (start, grid)
}

fn get_path(
    grid: &[Vec<char>],
    start_pos: (isize, isize),
    start_dir: char,
) -> HashSet<(isize, isize)> {
    let dir_map = HashMap::from([('^', (-1, 0)), ('>', (0, 1)), ('v', (1, 0)), ('<', (0, -1))]);

    let mut dir = start_dir;
    let mut pos = start_pos;
    let mut seen = HashSet::new();

    while (pos.0 >= 0 && pos.0 < grid.len() as isize)
        && (pos.1 >= 0 && pos.1 < grid[0].len() as isize)
    {
        seen.insert(pos);
        let offset = dir_map[&dir];
        let new_pos = ((pos.0 + offset.0), (pos.1 + offset.1));
        if let Some('#') = grid
            .get(new_pos.0 as usize)
            .and_then(|row| row.get(new_pos.1 as usize))
        {
            dir = match dir {
                '^' => '>',
                '>' => 'v',
                'v' => '<',
                '<' => '^',
                _ => panic!("at the disco"),
            }
        } else {
            pos = new_pos;
        }
    }

    seen
}

fn loops(grid: &[Vec<char>], start_pos: (isize, isize), start_dir: char) -> bool {
    let dir_map = HashMap::from([('^', (-1, 0)), ('>', (0, 1)), ('v', (1, 0)), ('<', (0, -1))]);

    let mut dir = start_dir;
    let mut pos = start_pos;
    let mut seen = HashSet::new();

    while (pos.0 >= 0 && pos.0 < grid.len() as isize)
        && (pos.1 >= 0 && pos.1 < grid[0].len() as isize)
    {
        if seen.contains(&(pos, dir)) {
            // loop detected
            return true;
        }
        seen.insert((pos, dir));
        let offset = dir_map[&dir];
        let new_pos = ((pos.0 + offset.0), (pos.1 + offset.1));
        if let Some('#') = grid
            .get(new_pos.0 as usize)
            .and_then(|row| row.get(new_pos.1 as usize))
        {
            dir = match dir {
                '^' => '>',
                '>' => 'v',
                'v' => '<',
                '<' => '^',
                _ => panic!("at the disco"),
            }
        } else {
            pos = new_pos;
        }
    }

    false
}

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        Ok(part1_faster(self.0))
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        Ok(part2_slower(self.0))
    }
}

// fn part1_slower(input: &str) -> usize {
//     let (start, grid) = parse(input);
//     get_path(&grid, start, '^').len()
// }

const OFFSETS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
pub fn part1_faster(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();
    let start = grid
        .iter()
        .enumerate()
        .find_map(|(r, row)| {
            row.iter()
                .position(|&c| c == b'^')
                .map(|c| (r as isize, c as isize))
        })
        .unwrap();

    let mut r = start.0;
    let mut c = start.1;
    let mut dir = 0;
    let cols = grid[0].len();
    let rows = grid.len();
    // a 2D vector is MUCH faster than a hashset
    let mut seen = vec![vec![false; cols]; rows];

    while (r >= 0 && r < grid.len() as isize) && (c >= 0 && c < grid[0].len() as isize) {
        seen[r as usize][c as usize] = true;
        let (dr, dc) = OFFSETS[dir];
        let new_r = r + dr;
        let new_c = c + dc;
        if let Some(b'#') = grid
            .get(new_r as usize)
            .and_then(|row| row.get(new_c as usize))
        {
            dir = (dir + 1) % 4;
        } else {
            r = new_r;
            c = new_c;
        }
    }

    seen.iter()
        .map(|row| row.iter().filter(|&x| *x).count())
        .sum()
}

fn part2_slower(input: &str) -> usize {
    let (start, mut grid) = parse(input);
    let path = get_path(&grid, start, '^');

    let mut sum = 0;
    for path_pos in path {
        // obstruction can't be placed at the guard's starting position
        if path_pos == start {
            continue;
        }

        // place obstruction
        grid[path_pos.0 as usize][path_pos.1 as usize] = '#';

        // detect loop
        if loops(&grid, start, '^') {
            sum += 1;
        }

        // remove obstruction again
        grid[path_pos.0 as usize][path_pos.1 as usize] = '.';
    }
    sum
}

// weirdly enough, this is slower than my unoptimized solution
// pub fn part2_faster(input: &str) -> usize {
//     let mut grid = input
//         .lines()
//         .map(|line| line.as_bytes().to_vec())
//         .collect::<Vec<_>>();
//     let start = grid
//         .iter()
//         .enumerate()
//         .find_map(|(r, row)| {
//             row.iter()
//                 .position(|&c| c == b'^')
//                 .map(|c| (r as isize, c as isize))
//         })
//         .unwrap();
//
//     let mut r = start.0;
//     let mut c = start.1;
//     let mut dir = 0;
//     let cols = grid[0].len();
//     let rows = grid.len();
//     // seen hashmap checked with: seen[row][col][direction]
//     let mut seen = vec![vec![vec![false; 4]; cols]; rows];
//
//     while (r >= 0 && r < grid.len() as isize) && (c >= 0 && c < grid[0].len() as isize) {
//         seen[r as usize][c as usize][dir] = true;
//         let (dr, dc) = OFFSETS[dir];
//         let new_r = r + dr;
//         let new_c = c + dc;
//         if let Some(b'#') = grid
//             .get(new_r as usize)
//             .and_then(|row| row.get(new_c as usize))
//         {
//             dir = (dir + 1) % 4;
//         } else {
//             r = new_r;
//             c = new_c;
//         }
//     }
//
//     seen.iter()
//         .enumerate()
//         .flat_map(|(r, row)| {
//             row.iter()
//                 .enumerate()
//                 .filter_map(move |(c, col)| col.iter().any(|b| *b).then_some((r, c)))
//         })
//         .filter(|&(r, c)| {
//             // obstruction can't be placed at the guard's starting position
//             if r == start.0 as usize && c == start.1 as usize {
//                 return false;
//             }
//             // place obstruction
//             grid[r][c] = b'#';
//             // detect loop
//             let looped = fast_loops(&grid, start, 0);
//             // remove obstruction again
//             grid[r][c] = b'.';
//             looped
//         })
//         .count()
// }
//
// fn fast_loops(grid: &[Vec<u8>], start_pos: (isize, isize), start_dir: usize) -> bool {
//     let mut dir = start_dir;
//     let mut r = start_pos.0;
//     let mut c = start_pos.1;
//     let cols = grid[0].len();
//     let rows = grid.len();
//     let mut seen = vec![vec![vec![false; 4]; cols]; rows];
//     while (r >= 0 && r < grid.len() as isize) && (c >= 0 && c < grid[0].len() as isize) {
//         if seen[r as usize][c as usize][dir] {
//             return true;
//         }
//         seen[r as usize][c as usize][dir] = true;
//         let (dr, dc) = OFFSETS[dir];
//         let new_r = r + dr;
//         let new_c = c + dc;
//         if let Some(b'#') = grid
//             .get(new_r as usize)
//             .and_then(|row| row.get(new_c as usize))
//         {
//             dir = (dir + 1) % 4;
//         } else {
//             r = new_r;
//             c = new_c;
//         }
//     }
//
//     false
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "41");
    }

    #[test]
    fn part_2() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "6");
    }
}
