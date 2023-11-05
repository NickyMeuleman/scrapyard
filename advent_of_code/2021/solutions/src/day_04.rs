use std::fmt::Display;

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data {
    numbers: Vec<u8>,
    boards: Vec<Board>,
}

#[derive(Debug, Clone)]
struct Board {
    grid: Vec<Square>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Square {
    Mark,
    Num(u8),
}

impl Board {
    fn mark(&mut self, num: u8) {
        if let Some(square) = self
            .grid
            .iter_mut()
            .find(|square| match square {
                Square::Mark => false,
                Square::Num(val) => *val == num,
            })
        {
            *square = Square::Mark;
        }
    }

    fn check_win(&self) -> bool {
        // hardcoded num rows and num cols as 5

        // check row winners
        'row: for row in 0..5 {
            for col in 0..5 {
                if self.grid[row * 5 + col] != Square::Mark {
                    continue 'row;
                }
            }

            return true;
        }

        // check col winners
        'col: for col in 0..5 {
            for row in 0..5 {
                if self.grid[row * 5 + col] != Square::Mark {
                    continue 'col;
                }
            }

            return true;
        }

        false
    }

    fn sum_unmarked(&self) -> u32 {
        self.grid
            .iter()
            .filter_map(|square| match square {
                Square::Mark => None,
                Square::Num(n) => Some(u32::from(*n)),
            })
            .sum()
    }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let (numbers, boards) = input
            .trim()
            .split_once("\n\n")
            .ok_or(AoCError::Parsing)?;
        let numbers = numbers
            .split(',')
            .map(|s| s.parse())
            .collect::<Result<Vec<u8>, _>>()?;

        let boards = boards
            .split("\n\n")
            .map(|block| {
                let numbers = block
                    .lines()
                    .flat_map(|line| {
                        line.split_whitespace()
                            .filter_map(|s| s.parse().ok())
                            .map(Square::Num)
                    })
                    .collect();

                Ok(Board { grid: numbers })
            })
            .collect::<AoCResult<Vec<Board>>>()?;

        Ok(Self { numbers, boards })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut boards = self.boards.clone();
        for num in &self.numbers {
            for board in boards.iter_mut() {
                board.mark(*num);
                if board.check_win() {
                    let result = board.sum_unmarked() * u32::from(*num);
                    return Ok(result);
                }
            }
        }

        Err(AoCError::new("No winning board found"))
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut todo = self.boards.clone();

        for num in &self.numbers {
            // iterate over the indexes of the todo vecor
            // in reverse order, because we call swap_remove which replaces a board with the last in the todo vector
            // because we're looping reversed, that doesn't cause issues, the removed one is always replaced by a board that has already been marked/checked this loop
            for idx in (0..todo.len()).rev() {
                // mark current board
                todo[idx].mark(*num);
                // check if it won
                if todo[idx].check_win() {
                    // check if it's the last one left in the todo vector
                    if todo.len() == 1 {
                        let result = todo[idx].sum_unmarked() * u32::from(*num);
                        return Ok(result);
                    }
                    // remove the board that just won from the todo vector
                    todo.swap_remove(idx);
                }
            }
        }

        // alternative solution:
        // use std::collections::HashSet;
        // let mut boards = self.boards.clone();
        // let mut winners: HashSet<usize> = HashSet::new();
        // let boards_len = boards.len();

        // for num in &self.numbers {
        //     for (idx, board) in boards.iter_mut().enumerate() {
        //         board.mark(*num);
        //         if board.check_win() {
        //             winners.insert(idx);
        //             if winners.len() == boards_len {
        //                 let result = board.sum_unmarked() * u32::from(*num);
        //                 return result.to_string();
        //             }
        //         }
        //     }
        // }

        Err(AoCError::new("No winning board found"))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "4512");
    }

    #[test]
    fn part_2() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "1924");
    }
}
