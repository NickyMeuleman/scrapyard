use std::collections::HashSet;

use crate::AoCData;

#[derive(Debug, Clone)]
pub struct Data {
    grid: HashSet<(usize, usize)>,
    num_cols: usize,
    num_rows: usize,
}

impl Data {
    fn is_corner(&self, row_idx: usize, col_idx: usize) -> bool {
        // top left
        if row_idx == 0 && col_idx == 0 {
            return true;
        }
        // top right
        if row_idx == 0 && col_idx == self.num_cols - 1 {
            return true;
        }
        // bottom left
        if row_idx == self.num_rows - 1 && col_idx == 0 {
            return true;
        }
        // bottom right
        if row_idx == self.num_rows - 1 && col_idx == self.num_cols - 1 {
            return true;
        }
        false
    }
    fn tick(self, part2: bool) -> Self {
        let mut new_grid = HashSet::new();
        for row_idx in 0..self.num_rows {
            for col_idx in 0..self.num_cols {
                if part2 && self.is_corner(row_idx, col_idx) {
                    new_grid.insert((row_idx, col_idx));
                    continue;
                };
                let neighbours = &self.neighbours(row_idx, col_idx);
                let is_on = match self.grid.contains(&(row_idx, col_idx)) {
                    true => matches!(neighbours, 2 | 3),
                    false => matches!(neighbours, 3),
                };
                if is_on {
                    new_grid.insert((row_idx, col_idx));
                }
            }
        }
        Self {
            grid: new_grid,
            num_cols: self.num_cols,
            num_rows: self.num_rows,
        }
    }

    fn neighbours(&self, row_idx: usize, col_idx: usize) -> u8 {
        let mut count = 0;
        for delta_row in [-1i32, 0, 1].iter().cloned() {
            for delta_col in [-1i32, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let neighbour_row = row_idx as i32 + delta_row;
                let neighbour_col = col_idx as i32 + delta_col;
                if neighbour_row < 0 || neighbour_col < 0 {
                    continue;
                }
                if self
                    .grid
                    .contains(&(neighbour_row as usize, neighbour_col as usize))
                {
                    count += 1;
                }
            }
        }
        count
    }

    fn print(&self) {
        for row_idx in 0..self.num_rows {
            for col_idx in 0..self.num_cols {
                if self.grid.contains(&(row_idx, col_idx)) {
                    print!("#")
                } else {
                    print!(".")
                }
            }
            println!();
        }
    }
}

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        let num_rows = input.lines().count();
        let num_cols = input.lines().next()?.len();
        let mut grid = HashSet::new();
        for (row_idx, line) in input.lines().enumerate() {
            for (col_idx, c) in line.chars().enumerate() {
                if c == '#' {
                    grid.insert((row_idx, col_idx));
                }
            }
        }

        Some(Self {
            grid,
            num_cols,
            num_rows,
        })
    }

    fn part_1(&self) -> String {
        let mut data = self.clone();
        for _ in 0..100 {
            data = data.tick(false);
        }
        data.grid.len().to_string()
    }

    fn part_2(&self) -> String {
        let mut data = self.clone();
        for _ in 0..100 {
            data = data.tick(true);
        }
        data.grid.len().to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(18);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "d");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(18);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "");
    }
}