use std::collections::VecDeque;
use std::{convert::Infallible, str::FromStr};

#[derive(Debug, Clone)]
pub struct Data {
    algorithm: Vec<bool>,
    image: Image,
}

#[derive(Debug, Clone)]
struct Image {
    data: VecDeque<VecDeque<bool>>,
}

impl Image {
    fn new(data: VecDeque<VecDeque<bool>>) -> Self {
        Self { data }
    }

    fn pad(&mut self, state: bool) {
        // a row filled with "state"
        let row: VecDeque<bool> = (0..self.data[0].len()).map(|_| state).collect();
        // add a first row full of "state"
        self.data.push_front(row.clone());
        // add a last row full of "state"
        self.data.push_back(row);

        // add a left column full of "state"
        // add a right column full of "state"
        for row in &mut self.data {
            row.push_front(state);
            row.push_back(state);
        }
    }

    fn enhance(&mut self, algorithm: &Vec<bool>) -> Self {
        // to deal with the infinite grid, expand it with the correct state
        // this state happens to be the same as the state of the very first point
        self.pad(self.data[0][0]);

        // create a new in-progress image for this step
        let mut data = self.data.clone();

        // go through every pixel in the input image, and calculate what the new pixel should be
        for row in 1..self.data.len() - 1 {
            for col in 1..self.data[row].len() - 1 {
                // construct the 3x3 square this step considers
                let square = vec![
                    // up left
                    self.data[row - 1][col - 1],
                    // up
                    self.data[row - 1][col],
                    // up right
                    self.data[row - 1][col + 1],
                    // left
                    self.data[row][col - 1],
                    // center
                    self.data[row][col],
                    // right
                    self.data[row][col + 1],
                    // down left
                    self.data[row + 1][col - 1],
                    // down
                    self.data[row + 1][col],
                    // down right
                    self.data[row + 1][col + 1],
                ];

                // concatenate every point in the square to get the index in the algorithm
                // doing it this way because collecting that square into a string and then parsing that as binary is slow
                let idx = square
                    .into_iter()
                    // move what we have in the accumulater one to the left.
                    // that << is the same as multiplying by 2, but more computer sciency, because binary numbers go brrrrrr
                    .fold(0, |acc, state| {
                        // shift the binary so far one to the left
                        let acc = acc * 2;
                        // add 1 to it if the current state is true
                        acc | if state { 1 } else { 0 }
                    });
                // equivalent would be:
                // .fold(0, |a, b| a << 1 | (b) as usize);

                // the value in the algorithm at that index is the value that replaces the middle point in the in-progress image
                data[row][col] = algorithm[idx];
            }
        }

        // to deal with the infinity, we need to overwrite the edges we padded at the start
        // figure out with what we should replace the padding
        let pad_state = algorithm[vec![data[0][0]; 9]
            .into_iter()
            .fold(0, |acc, state| acc << 1 | state as usize)];

        // store the amount of rows before we mutably borrow self
        let last = data.len() - 1;

        // overwrite the first and last rows
        // overwrite the first and last colums
        for (idx, row) in data.iter_mut().enumerate() {
            row[0] = pad_state;
            row[last] = pad_state;
            if idx == 0 || idx == last {
                for state in row {
                    *state = pad_state
                }
            }
        }

        Image::new(data)
    }

    fn count_on(&self) -> usize {
        self.data
            .iter()
            .map(|row| row.iter().filter(|&v| *v == true).count())
            .sum()
    }
}

impl Data {
    pub fn part_one(&self) -> usize {
        let mut img = self.image.clone();
        // curse you, infinite grid after 1 step
        img.pad(false);

        for _ in 0..2 {
            img = img.enhance(&self.algorithm);
        }

        img.count_on()
    }

    pub fn part_two(&self) -> usize {
        let mut img = self.image.clone();
        // curse you, infinite grid after 1 step
        img.pad(false);

        for _ in 0..50 {
            img = img.enhance(&self.algorithm);
        }

        img.count_on()
    }
}

impl FromStr for Data {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (algorithm, imgage) = input.split_once("\n\n").unwrap();

        let algorithm = algorithm
            .chars()
            .map(|c| match c {
                '.' => false,
                '#' => true,
                _ => unreachable!("invalid input"),
            })
            .collect();

        let image = imgage
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => false,
                        '#' => true,
                        _ => unreachable!("invalid input"),
                    })
                    .collect()
            })
            .collect();

        Ok(Self {
            algorithm,
            image: Image::new(image),
        })
    }
}
// use std::{convert::Infallible, str::FromStr};
// use std::collections::VecDeque;

// #[derive(Debug, Clone)]
// pub struct Data {
//     algorithm: Vec<bool>,
//     image: Image,
// }

// #[derive(Debug, Clone)]
// struct Image {
//     data: Vec<Vec<bool>>,
// }

// impl Image {
//     fn new(data: Vec<Vec<bool>>) -> Self {
//         Self { data }
//     }

//     fn pad(&mut self, state: bool) {
//         // add a first row full of "state"
//         self.data.insert(0, vec![state; self.data[0].len()]);
//         // add a last row full of "state"
//         self.data.push(vec![state; self.data[0].len()]);

//         // add a left column full of "state"
//         // add a right column full of "state"
//         for row in self.data.iter_mut() {
//             row.insert(0, state);
//             row.push(state);
//         }
//     }

//     fn enhance(&mut self, algorithm: &Vec<bool>) {
//         // to deal with the infinite grid, expand it with the correct state
//         // this state happens to be the same as the state of the very first point
//         self.pad(self.data[0][0]);

//         // create a new in-progress image for this step
//         let mut image = self.data.clone();

//         // go through every pixel in the input image, and calculate what the new pixel should be
//         for row in 1..self.data.len() - 1 {
//             for col in 1..self.data[row].len() - 1 {
//                 // construct the 3x3 square this step considers
//                 let square = vec![
//                     // up left
//                     self.data[row - 1][col - 1],
//                     // up
//                     self.data[row - 1][col],
//                     // up right
//                     self.data[row - 1][col + 1],
//                     // left
//                     self.data[row][col - 1],
//                     // center
//                     self.data[row][col],
//                     // right
//                     self.data[row][col + 1],
//                     // down left
//                     self.data[row + 1][col - 1],
//                     // down
//                     self.data[row + 1][col],
//                     // down right
//                     self.data[row + 1][col + 1],
//                 ];

//                 // concatenate every point in the square to get the index in the algorithm
//                 // doing it this way because collecting that square into a string and then parsing that as binary is slow
//                 let idx = square
//                     .into_iter()
//                     // move what we have in the accumulater one to the left.
//                     // that << is the same as multiplying by 2, but more computer sciency, because binary numbers go brrrrrr
//                     .fold(0, |acc, state| {
//                         // shift the binary so far one to the left
//                         let acc = acc * 2;
//                         // add 1 to it if the current state is true
//                         acc | if state { 1 } else { 0 }
//                     });
//                 // equivalent would be:
//                 // .fold(0, |a, b| a << 1 | (b) as usize);

//                 // the value in the algorithm at that index is the value that replaces the middle point in the in-progress image
//                 image[row][col] = algorithm[idx];
//             }
//         }

//         // every change has been made to the in-progress image, swap it into place of the real image
//         std::mem::swap(&mut self.data, &mut image);

//         // to deal with the infinity, we need to overwrite the edges we padded at the start
//         // figure out with what we should replace the padding
//         let pad_state = algorithm[vec![self.data[0][0]; 9]
//             .into_iter()
//             .fold(0, |acc, state| acc << 1 | state as usize)];

//         // store the amount of rows before we mutably borrow self
//         let last = self.data.len() - 1;

//         // overwrite the first and last colums
//         // overwrite the first and last rows
//         for (idx, row) in self.data.iter_mut().enumerate() {
//             row[0] = pad_state;
//             *row.last_mut().unwrap() = pad_state;
//             if idx == 0 || idx == last {
//                 row.iter_mut().for_each(|state| *state = pad_state);
//             }
//         }
//     }

//     fn count_on(&self) -> usize {
//         self.data
//             .iter()
//             .map(|row| row.iter().filter(|&v| *v == true).count())
//             .sum()
//     }
// }

// impl Data {
//     pub fn part_one(&self) -> usize {
//         let mut img = self.image.clone();
//         // curse you, infinite grid after 1 step
//         img.pad(false);

//         for _ in 0..2 {
//             img.enhance(&self.algorithm);
//         }

//         img.count_on()
//     }

//     pub fn part_two(&self) -> usize {
//         let mut img = self.image.clone();
//         // curse you, infinite grid after 1 step
//         img.pad(false);

//         for _ in 0..50 {
//             img.enhance(&self.algorithm);
//         }

//         img.count_on()
//     }
// }

// impl FromStr for Data {
//     type Err = Infallible;

//     fn from_str(input: &str) -> Result<Self, Self::Err> {
//         let (algorithm, imgage) = input.split_once("\n\n").unwrap();

//         let algorithm = algorithm
//             .chars()
//             .map(|c| match c {
//                 '.' => false,
//                 '#' => true,
//                 _ => unreachable!("invalid input"),
//             })
//             .collect();

//         let image = imgage
//             .lines()
//             .map(|line| {
//                 line.chars()
//                     .map(|c| match c {
//                         '.' => false,
//                         '#' => true,
//                         _ => unreachable!("invalid input"),
//                     })
//                     .collect()
//             })
//             .collect();

//         Ok(Self {
//             algorithm,
//             image: Image::new(image),
//         })
//     }
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_one(), 35);
    }

    #[test]
    fn part_two() {
        let input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_two(), 3351);
    }
}
