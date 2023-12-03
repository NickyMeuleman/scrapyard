use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<Vec<char>>);

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        Ok(Self(
            input
                .trim()
                .lines()
                .map(|line| line.trim().chars().collect())
                .collect(),
        ))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut sum = 0;
        let mut current_num = 0;
        let mut has_adjacent_symbol = false;

        for row_idx in 0..self.0.len() {
            for col_idx in 0..self.0[row_idx].len() {
                let value = self.0[row_idx][col_idx];

                // Not a number, not interested
                if !value.is_ascii_digit() {
                    continue;
                }

                // check if any adjacent tile is a symbol
                for row_offset in -1..=1 {
                    for col_offset in -1..=1 {
                        // Skip self
                        if row_offset == 0 && col_offset == 0 {
                            continue;
                        }

                        let adjacent_row_idx = row_idx as i32 + row_offset;
                        let adjacent_col_idx = col_idx as i32 + col_offset;

                        // Out of bounds
                        if adjacent_row_idx < 0
                            || adjacent_row_idx >= self.0.len() as i32
                            || adjacent_col_idx < 0
                            || adjacent_col_idx >= self.0[adjacent_row_idx as usize].len() as i32
                        {
                            continue;
                        }

                        let adjacent_value =
                            self.0[adjacent_row_idx as usize][adjacent_col_idx as usize];
                        if !adjacent_value.is_ascii_digit() && adjacent_value != '.' {
                            has_adjacent_symbol = true;
                        }
                    }
                }

                // Adjust the number currently being built (concatenate a digit using math)
                current_num *= 10;
                current_num += value.to_digit(10).unwrap();

                // If we reached the end of the line, or the next horizontal coordinate is not a digit, the current number is complete
                // check if the number has an adjacent symbol, and reset the temporary values before starting on a new number
                if col_idx + 1 >= self.0[row_idx].len()
                    || !self.0[row_idx][col_idx + 1].is_ascii_digit()
                {
                    if has_adjacent_symbol {
                        sum += current_num;
                    }

                    current_num = 0;
                    has_adjacent_symbol = false;
                }
            }
        }

        Ok(sum)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        // key: star coordinate, val: list of adjacent numbers
        let mut stars: HashMap<(i32, i32), Vec<u32>> = HashMap::new();
        let mut current_num = 0;
        let mut adjacent_star_positions: HashSet<(i32, i32)> = HashSet::new();

        for row_idx in 0..self.0.len() {
            for col_idx in 0..self.0[row_idx].len() {
                let value = self.0[row_idx][col_idx];

                // Not a number, not interested
                if !value.is_ascii_digit() {
                    continue;
                }

                // check if any adjacent tile is a star
                for row_offset in -1..=1 {
                    for col_offset in -1..=1 {
                        // Skip self
                        if row_offset == 0 && col_offset == 0 {
                            continue;
                        }

                        let adjacent_row_idx = row_idx as i32 + row_offset;
                        let adjacent_col_idx = col_idx as i32 + col_offset;

                        // Out of bounds
                        if adjacent_row_idx < 0
                            || adjacent_row_idx >= self.0.len() as i32
                            || adjacent_col_idx < 0
                            || adjacent_col_idx >= self.0[adjacent_row_idx as usize].len() as i32
                        {
                            continue;
                        }

                        if self.0[adjacent_row_idx as usize][adjacent_col_idx as usize] == '*' {
                            adjacent_star_positions.insert((adjacent_row_idx, adjacent_col_idx));
                        }
                    }
                }

                // Adjust the number currently being built (concatenate a digit using math)
                current_num *= 10;
                current_num += value.to_digit(10).unwrap();

                // If we reached the end of the line, or the next horizontal coordinate is not a digit, the current number is complete
                if col_idx + 1 >= self.0[row_idx].len()
                    || !self.0[row_idx][col_idx + 1].is_ascii_digit()
                {
                    // add all stars to the variable keeping track of stars (potential gears)
                    for point in &adjacent_star_positions {
                        stars
                            .entry(*point)
                            .or_default()
                            .push(current_num);
                    }

                    // reset the temporary values before starting on a new number
                    current_num = 0;
                    adjacent_star_positions.clear();
                }
            }
        }

        let sum: u32 = stars
            .values()
            // only stars with exactly 2 adjacent numbers are gears
            .filter(|numbers| numbers.len() == 2)
            .map(|numbers| numbers[0] * numbers[1])
            .sum();

        Ok(sum)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "4361");
    }

    #[test]
    fn part_2() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "467835");
    }
}
