use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let data = parse(&input);
    println!("Part one answer: {}", part_one(data.clone()));
    println!("Part two answer: {}", part_two(data));
}

#[derive(Debug, Clone)]
struct Data {
    numbers: Vec<u8>,
    boards: Vec<Board>,
}

#[derive(Debug, Clone)]
struct Board {
    idx: usize,
    grid: Vec<Vec<(u8, bool)>>,
}

impl Board {
    fn mark(&mut self, called_num: u8) {
        if let Some(square) = self.grid.iter_mut().flatten().find(|(num, _)| *num == called_num) {
            square.1 = true;
        }
    }
    // initial solution below:
    // fn mark(&mut self, called_num: u8) {
    //     for row in &mut self.grid {
    //         for (num, marked) in row {
    //             if called_num == *num {
    //                 *marked = true;
    //             }
    //         }
    //     }
    // }

    fn check_win(&self) -> bool {
        // this version doesn't do early returns
        let mut row_candidates: HashSet<usize> = (0..5).collect();
        let mut col_candidates: HashSet<usize> = (0..5).collect();

        for row_idx in 0..5 {
            for col_idx in 0..5 {
                // remove invalid result from being eligible
                let result = self.grid[row_idx][col_idx].1;
                if result == false {
                    row_candidates.remove(&row_idx);
                    col_candidates.remove(&col_idx);
                }
            }
        }

        !row_candidates.is_empty() || !col_candidates.is_empty()
    }

    // initial solution below:
    // fn check_win(&self) -> bool {
    //     // check if a row is full
    //     for row in &self.grid {
    //         if row.iter().all(|(_, mark)| *mark) {
    //             return true;
    //         }
    //     }

    //     // make vec of cols. I know, very inefficient, I want to easily iterate through the cols 🤷‍♂️
    //     let mut cols = Vec::new();
    //     for col_idx in 0..5 {
    //         let col: Vec<bool> = self.grid.iter().map(|row| row[col_idx].1).collect();
    //         cols.push(col);
    //     }

    //     // check if a col is full
    //     for col in cols {
    //         if col.iter().all(|mark| *mark) {
    //             return true;
    //         }
    //     }

    //     false
    // }

    fn sum_unmarked(&self) -> u32 {
        self.grid
            .iter()
            .flatten()
            .filter_map(|(num, mark)| if *mark { None } else { Some(*num as u32) })
            .sum()
    }

    // initial solution below:
    // fn sum_unmarked(&self) -> u32 {
    //     let mut sum: u32 = 0;
    //     for row in &self.grid {
    //         for (num, mark) in row {
    //             if !*mark {
    //                 sum += *num as u32;
    //             }
    //         }
    //     }
    //     sum
    // }
}

fn parse(input: &str) -> Data {
    let (numbers, rest) = input.split_once("\n\n").unwrap();
    let numbers = numbers.split(",").filter_map(|s| s.parse().ok()).collect();

    let boards = rest
        .split("\n\n")
        .enumerate()
        .map(|(idx, block)| parse_block(idx, block))
        .collect();

    Data { numbers, boards }
}

fn parse_block(idx: usize, input: &str) -> Board {
    Board {
        idx,
        grid: input.lines().map(parse_row).collect(),
    }
}

fn parse_row(input: &str) -> Vec<(u8, bool)> {
    input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .map(|num| (num, false))
        .collect()
}

fn part_one(mut data: Data) -> u32 {
    for num in data.numbers {
        for board in &mut data.boards {
            board.mark(num);
            if board.check_win() {
                return num as u32 * board.sum_unmarked();
            }
        }
    }

    panic!("No solution was found");
}

fn part_two(mut data: Data) -> u32 {
    let mut winners: HashSet<usize> = HashSet::new();
    let boards_len = data.boards.len();

    for num in data.numbers {
        for board in &mut data.boards {
            board.mark(num);
            if board.check_win() {
                winners.insert(board.idx);
                if winners.len() == boards_len {
                    return num as u32 * board.sum_unmarked();
                }
            }
        }
    }

    panic!("No solution was found");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_one_example() {
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

        let data = parse(input);
        assert_eq!(part_one(data), 4512);
    }

    #[test]
    fn part_two_example() {
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

        let data = parse(input);
        assert_eq!(part_two(data), 1924);
    }
}