use std::{collections::HashSet, fmt::Display};

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data {
    rows: usize,
    cols: usize,
    map: Vec<u8>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    // version that uses DFS (depth first search) to visit all points in a basin
    fn get_basin_size(&self, map: &[u8], rows: usize, cols: usize) -> u32 {
        let mut seen = HashSet::new();
        let mut stack = vec![*self];
        seen.insert(*self);

        while !stack.is_empty() {
            let cur = stack.pop().unwrap();
            for neighbour in &cur.neighbours(rows, cols) {
                if seen.contains(neighbour) || map[neighbour.row * cols + neighbour.col] == 9 {
                    continue;
                }
                seen.insert(*neighbour);
                stack.push(*neighbour);
            }
        }

        u32::try_from(seen.len()).unwrap()
    }

    // alternative solution that uses recursion
    // fn get_basin_size(
    //     &self,
    //     map: &[u8],
    //     rows: usize,
    //     cols: usize,
    //     seen: &mut HashSet<Point>,
    // ) -> u32 {
    //     // don't count points that are already part of a basin
    //     if seen.contains(self) {
    //         return 0;
    //     }
    //     // don't count points with a height of 9
    //     if map[self.row * cols + self.col] == 9 {
    //         return 0;
    //     }

    //     // add the current point to the basin
    //     seen.insert(self.clone());

    //     // total size = 1 (the current point) + the size of the rest of the points in the same basin
    //     // done with recursion (google: floodfill algorithm)
    //     1 + self
    //         .neighbours(rows, cols)
    //         .iter()
    //         .map(|point| point.get_basin_size(map, rows, cols, seen))
    //         .sum::<u32>()
    // }

    fn neighbours(&self, rows: usize, cols: usize) -> Vec<Point> {
        let mut neighbours: Vec<Point> = Vec::new();

        // up
        if self.row > 0 {
            neighbours.push(Point {
                row: self.row - 1,
                col: self.col,
            })
        }

        // down
        if self.row < rows - 1 {
            neighbours.push(Point {
                row: self.row + 1,
                col: self.col,
            })
        }

        // left
        if self.col > 0 {
            neighbours.push(Point {
                row: self.row,
                col: self.col - 1,
            })
        }

        // right
        if self.col < cols - 1 {
            neighbours.push(Point {
                row: self.row,
                col: self.col + 1,
            })
        }

        neighbours
    }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let input = input.trim();
        let rows = input.lines().count();
        let cols = input
            .lines()
            .next()
            .ok_or(AoCError::Parsing)?
            .len();
        let map = input
            .lines()
            .flat_map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).map(|digit| digit as u8))
            })
            .collect::<Option<Vec<_>>>()
            .ok_or(AoCError::Parsing)?;

        Ok(Self { rows, cols, map })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let result: u32 = (0..self.rows)
            // iterate over all (row, col) combinations
            .flat_map(|row| (0..self.cols).map(move |col| (row, col)))
            // keep the height of the points that are lower than all neighbours
            .filter_map(|(row, col)| {
                let height = self.map[row * self.cols + col];

                // up
                if row > 0 && height >= self.map[(row - 1) * self.cols + col] {
                    return None;
                }

                // down
                if row < (self.rows - 1) && height >= self.map[(row + 1) * self.cols + col] {
                    return None;
                }

                // left
                if col > 0 && height >= self.map[row * self.cols + (col - 1)] {
                    return None;
                }

                // right
                if col < (self.cols - 1) && height >= self.map[row * self.cols + (col + 1)] {
                    return None;
                }

                Some(height)
            })
            // the danger level for a point is 1 + height
            .map(|height| 1 + u32::from(height))
            .sum();

        Ok(result)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut sizes: Vec<u32> = (0..self.rows)
            // iterate over all (row, col) combinations
            .flat_map(|row| (0..self.cols).map(move |col| (row, col)))
            // keep the indexes of the points that are lower than all neighbours
            .filter_map(|(row, col)| {
                let height = self.map[row * self.cols + col];

                // up
                if row > 0 && height >= self.map[(row - 1) * self.cols + col] {
                    return None;
                }

                // down
                if row < (self.rows - 1) && height >= self.map[(row + 1) * self.cols + col] {
                    return None;
                }

                // left
                if col > 0 && height >= self.map[row * self.cols + (col - 1)] {
                    return None;
                }

                // right
                if col < (self.cols - 1) && height >= self.map[row * self.cols + (col + 1)] {
                    return None;
                }

                Some(Point { row, col })
            })
            // get the size of the basin starting at these low points
            .map(|point: Point| {
                point.get_basin_size(&self.map, self.rows, self.cols)
                // or, the recursive version below:
                // point.get_basin_size(&self.map, self.rows, self.cols, &mut HashSet::new())
            })
            .collect();

        // take the product of the 3 largest basins
        sizes.sort_unstable();
        let result: u32 = sizes.iter().rev().take(3).product();

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "15");
    }

    #[test]
    fn part_2() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "1134");
    }
}
