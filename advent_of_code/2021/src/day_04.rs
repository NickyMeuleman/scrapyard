use crate::AoCData;

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
        if let Some(square) = self.grid.iter_mut().find(|square| match square {
            Square::Mark => false,
            Square::Num(val) => *val == num,
        }) {
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

impl AoCData for Data {
    fn new(input: String) -> Self {
        let (numbers, boards) = input.trim().split_once("\n\n").unwrap();

        let numbers = numbers.split(',').filter_map(|s| s.parse().ok()).collect();

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

                Board { grid: numbers }
            })
            .collect();

        Self { numbers, boards }
    }

    fn part_1(&self) -> String {
        let mut boards = self.boards.clone();
        for num in &self.numbers {
            for board in boards.iter_mut() {
                board.mark(*num);
                if board.check_win() {
                    let result = board.sum_unmarked() * u32::from(*num);
                    return result.to_string();
                }
            }
        }

        unreachable!("No winning board found")
    }

    fn part_2(&self) -> String {
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
                        return result.to_string();
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

        unreachable!("No winning board found")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(4);
        let data = Data::new(input);
        assert_eq!(data.part_1(), "4512");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(4);
        let data = Data::new(input);
        assert_eq!(data.part_2(), "1924");
    }
}
