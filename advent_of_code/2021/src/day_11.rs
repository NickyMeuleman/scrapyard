use crate::AoCData;

fn neighbours(idx: usize, rows: usize, cols: usize) -> Vec<usize> {
    let (row, col) = (idx / cols, idx % rows);
    let mut neighbours = Vec::new();

    // up
    if row > 0 {
        neighbours.push((row - 1) * cols + col)
    }

    //  up-left
    if row > 0 && col > 0 {
        neighbours.push((row - 1) * cols + (col - 1))
    }

    //  up-right
    if row > 0 && col < cols - 1 {
        neighbours.push((row - 1) * cols + (col + 1))
    }

    // down
    if row < rows - 1 {
        neighbours.push((row + 1) * cols + col)
    }

    // down-left
    if row < rows - 1 && col > 0 {
        neighbours.push((row + 1) * cols + (col - 1))
    }

    // down-right
    if row < rows - 1 && col < cols - 1 {
        neighbours.push((row + 1) * cols + (col + 1))
    }

    // left
    if col > 0 {
        neighbours.push(row * cols + (col - 1))
    }

    // right
    if col < cols - 1 {
        neighbours.push(row * cols + (col + 1))
    }

    neighbours
}

#[derive(Debug, Clone)]
pub struct Data {
    rows: usize,
    cols: usize,
    octopi: Vec<u8>,
}

impl Data {
    /// returns amount of octopi that flashed in a single turn
    fn tick(&mut self) -> u8 {
        let octopi = &mut self.octopi;
        let mut flashes = 0;
        let mut stack = Vec::new();

        // increment all octopi
        for (idx, octopus) in octopi.iter_mut().enumerate() {
            *octopus += 1;
            // push the octopi that should flash to the stack
            if *octopus > 9 {
                stack.push(idx);
            }
        }

        // handle flashes
        while let Some(idx) = stack.pop() {
            // flash and reset energy level of octopus
            flashes += 1;
            octopi[idx] = 0;

            for neighbour_idx in neighbours(idx, self.rows, self.cols) {
                // prevent a neighbour octopus that has already flashed or has already been pushed to the stack from being pushed to the stack
                if octopi[neighbour_idx] == 0 || octopi[neighbour_idx] > 9 {
                    continue;
                }

                // increment all neighbour octopi
                octopi[neighbour_idx] += 1;
                // if any neighbour should flash now, add it to the stack
                if octopi[neighbour_idx] > 9 {
                    stack.push(neighbour_idx);
                }
            }
        }

        flashes
    }

    // Alternative solution with seperate Point struct and a HashSet to keep track of octopi that already flashed
    // /// returns amount of octopi that flashed in a single turn
    // fn tick(&mut self) -> u8 {
    //     use std::collections::HashSet;
    //     let octopi = &mut self.octopi;
    //     let mut flashes = 0;

    //     let mut stack = Vec::new();
    //     let mut flashed = HashSet::new();

    //     // increment energy of every octopus
    //     for row in 0..self.rows {
    //         for col in 0..self.cols {
    //             let idx = row * self.cols + col;
    //             octopi[idx] += 1;
    //             // add octopus that should flash to stack
    //             if octopi[idx] > 9 {
    //                 stack.push(Point { row, col });
    //             }
    //         }
    //     }

    //     // flash ocotpi on the stack if not already flashed, increment energy of neighbours, add octopi that should flash to stack
    //     while !stack.is_empty() {
    //         // flash
    //         let cur = stack.pop().unwrap();
    //         if flashed.contains(&cur) {
    //             continue;
    //         }
    //         flashes += 1;
    //         flashed.insert(cur);

    //         // increment neighbours
    //         for neighbour in cur.neighbours(self.rows, self.cols) {
    //             let neighbour_idx = neighbour.row * self.cols + neighbour.col;
    //             octopi[neighbour_idx] += 1;
    //             // add neighbours with high enough energy to the stack
    //             if octopi[neighbour_idx] > 9 {
    //                 stack.push(neighbour);
    //             }
    //         }
    //     }

    //     // reset energy of all octopi that flashed
    //     for point in flashed {
    //         octopi[point.row * self.cols + point.col] = 0;
    //     }

    //     flashes
    // }
}

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        let input = input.trim();
        let cols = input.lines().count();
        let rows = input.lines().next()?.len();
        let octopi = input
            .lines()
            .flat_map(|line| line.chars().map(|c| c.to_digit(10).map(|digit| digit as u8)))
            .collect::<Option<Vec<u8>>>()?;

        Some(Self { rows, cols, octopi })
    }

    fn part_1(&self) -> String {
        let mut data = self.clone();

        let result: u16 = (0..100).map(|_| u16::from(data.tick())).sum();

        result.to_string()
    }

    fn part_2(&self) -> String {
        let mut data = self.clone();
        let total_num_octopi = u8::try_from(self.rows * self.cols).unwrap();

        let result: u16 = (1..).find(|_| data.tick() == total_num_octopi).unwrap();

        result.to_string()
    }
}

// #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
// struct Point {
//     row: usize,
//     col: usize,
// }

// impl Point {
//     fn neighbours(&self, rows: usize, cols: usize) -> Vec<Point> {
//         let mut neighbours: Vec<Point> = Vec::new();

//         // up
//         if self.row > 0 {
//             neighbours.push(Point {
//                 row: self.row - 1,
//                 col: self.col,
//             })
//         }

//         //  up-left
//         if self.row > 0 && self.col > 0 {
//             neighbours.push(Point {
//                 row: self.row - 1,
//                 col: self.col - 1,
//             })
//         }

//         //  up-right
//         if self.row > 0 && self.col < cols - 1 {
//             neighbours.push(Point {
//                 row: self.row - 1,
//                 col: self.col + 1,
//             })
//         }

//         // down
//         if self.row < rows - 1 {
//             neighbours.push(Point {
//                 row: self.row + 1,
//                 col: self.col,
//             })
//         }

//         // down-left
//         if self.row < rows - 1 && self.col > 0 {
//             neighbours.push(Point {
//                 row: self.row + 1,
//                 col: self.col - 1,
//             })
//         }

//         // down-right
//         if self.row < rows - 1 && self.col < cols - 1 {
//             neighbours.push(Point {
//                 row: self.row + 1,
//                 col: self.col + 1,
//             })
//         }

//         // left
//         if self.col > 0 {
//             neighbours.push(Point {
//                 row: self.row,
//                 col: self.col - 1,
//             })
//         }

//         // right
//         if self.col < cols - 1 {
//             neighbours.push(Point {
//                 row: self.row,
//                 col: self.col + 1,
//             })
//         }

//         neighbours
//     }
// }

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(11);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "1656");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(11);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "195");
    }
}
