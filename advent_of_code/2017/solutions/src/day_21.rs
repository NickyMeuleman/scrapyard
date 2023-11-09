use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;

use crate::{AoCData, AoCResult};

type Grid = Vec<Vec<bool>>;
#[derive(Debug, Clone)]
pub struct Data(HashMap<Grid, Grid>);

fn flip_horizontal(grid: &Grid) -> Grid {
    grid.iter()
        .cloned()
        .map(|row| row.into_iter().rev().collect())
        .collect()
}

// fn show(grid: &Grid) {
//     for row in grid {
//         for b in row {
//             if *b {
//                 print!("#")
//             } else {
//                 print!(".")
//             }
//         }
//         println!()
//     }
// }

// rotate 90 degrees counterclockwise: (x, y) -> (-y, x)
// rotate 90 degrees clockwise: (x, y) -> (y, -x)
fn clockwise(grid: &Grid) -> Grid {
    let size = grid.len();
    let mut rotated = vec![vec![false; size]; size];
    for row in 0..size {
        for col in 0..size {
            rotated[col][size - 1 - row] = grid[row][col];
        }
    }
    rotated
}

fn all_flips(grid: &Grid) -> [Grid; 8] {
    let orig = grid.to_vec();
    let orig_90 = clockwise(&orig);
    let orig_180 = clockwise(&orig_90);
    let orig_270 = clockwise(&orig_180);

    let flip = flip_horizontal(&orig);
    let flip_90 = clockwise(&flip);
    let flip_180 = clockwise(&flip_90);
    let flip_270 = clockwise(&flip_180);

    [
        orig, orig_90, orig_180, orig_270, flip, flip_90, flip_180, flip_270,
    ]
}

fn split_up(grid: Grid) -> Vec<Vec<Grid>> {
    // If the size is evenly divisible by 2, break the pixels up into 2x2 squares
    // Otherwise, the size is evenly divisible by 3; break the pixels up into 3x3 squares
    let size = if grid.len() % 2 == 0 {
        2
    } else {
        assert_eq!(grid.len() % 3, 0);
        3
    };
    // create an empty square size grid.len()/size 2D vector of square Grid size "size"
    let mut grids = vec![vec![vec![vec![false; size]; size]; grid.len() / size]; grid.len() / size];
    for grids_row in 0..(grid.len() / size) {
        for grids_col in 0..(grid.len() / size) {
            let target = &mut grids[grids_row][grids_col];
            for row in 0..size {
                for col in 0..size {
                    let origin_col = grids_col * size + col;
                    let origin_row = grids_row * size + row;
                    target[row][col] = grid[origin_row][origin_col];
                }
            }
        }
    }
    grids
}

fn combine(grids: Vec<Vec<Grid>>) -> Grid {
    // grids: 2D vec of Grid
    // grids_size = amount of Grids on a row/col (square, so row.len()==col.len())
    let grids_size = grids.len();
    // grid_size = size of a singular grid being merged
    let grid_size = grids[0][0].len();
    let mut combined = vec![vec![false; grids_size * grid_size]; grids_size * grid_size];
    for grids_row in 0..grids_size {
        for grids_col in 0..grids_size {
            for row in 0..grid_size {
                for col in 0..grid_size {
                    let source_grid = &grids[grids_row][grids_col];
                    let combined_col = grids_col * grid_size + col;
                    let combined_row = grids_row * grid_size + row;
                    combined[combined_row][combined_col] = source_grid[row][col];
                }
            }
        }
    }
    combined
}

fn iteration(grid: Grid, rules: &HashMap<Grid, Grid>) -> Grid {
    let split_up = split_up(grid);
    let enhanced_pieces = split_up
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|grid| {
                    let flips = all_flips(&grid);
                    flips
                        .into_iter()
                        .find_map(|flip| rules.get(&flip))
                        .unwrap()
                        .clone()
                })
                .collect_vec()
        })
        .collect_vec();
    let combined = combine(enhanced_pieces);
    combined
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        fn parse_grid(s: &str) -> Grid {
            let mut grid = Vec::new();
            for row_str in s.split("/") {
                let mut row = Vec::new();
                for c in row_str.chars() {
                    let pixel = match c {
                        '#' => true,
                        '.' => false,
                        _ => panic!("invalid input"),
                    };
                    row.push(pixel);
                }
                grid.push(row);
            }
            grid
        }
        Ok(Self(
            input
                .lines()
                .map(|line| {
                    let (from, to) = line.split_once(" => ").unwrap();
                    let from = parse_grid(from);
                    let to = parse_grid(to);
                    (from, to)
                })
                .collect(),
        ))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let rules = &self.0;
        // each key in the rules map can be rotated and flipped
        let rules: HashMap<Vec<Vec<bool>>, Vec<Vec<bool>>> = rules
            .into_iter()
            .flat_map(|(k, v)| {
                all_flips(&k)
                    .into_iter()
                    .map(move |from| (from, v.clone()))
            })
            .collect();
        let mut grid = vec![
            vec![false, true, false],
            vec![false, false, true],
            vec![true, true, true],
        ];

        for _ in 0..5 {
            grid = iteration(grid, &rules);
        }

        let result: usize = grid
            .into_iter()
            .map(|row| row.into_iter().filter(|&b| b).count())
            .sum();

        Ok(result)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let rules = &self.0;
        // each key in the rules map can be rotated and flipped
        let rules: HashMap<Vec<Vec<bool>>, Vec<Vec<bool>>> = rules
            .into_iter()
            .flat_map(|(k, v)| {
                all_flips(&k)
                    .into_iter()
                    .map(move |from| (from, v.clone()))
            })
            .collect();
        let mut grid = vec![
            vec![false, true, false],
            vec![false, false, true],
            vec![true, true, true],
        ];

        for _ in 0..18 {
            grid = iteration(grid, &rules);
        }

        let result: usize = grid
            .into_iter()
            .map(|row| row.into_iter().filter(|&b| b).count())
            .sum();

        Ok(result)
    }
}
