use std::fmt::Display;

use itertools::Itertools;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<Vec<Cell>>);

#[derive(Debug, Clone, PartialEq)]
enum Cell {
    Taken,
    Empty,
    Floor,
}

fn find_neighbours(seats: &Vec<Vec<Cell>>, seat_idx: (usize, usize)) -> Vec<(usize, usize)> {
    let offsets: Vec<(i32, i32)> = (-1..=1)
        .cartesian_product(-1..=1)
        .filter(|&item| item != (0, 0))
        .collect();
    let neighbours: Vec<(usize, usize)> = offsets
        .iter()
        .map(|offset| (seat_idx.0 as i32 + offset.0, seat_idx.1 as i32 + offset.1))
        .filter(|&tup| is_within_bounds((seats.len() - 1, seats[0].len() - 1), tup))
        .map(|tup| (tup.0 as usize, tup.1 as usize))
        .collect();
    neighbours
}
fn find_seen_neighbours(seats: &Vec<Vec<Cell>>, seat_idx: (usize, usize)) -> Vec<(usize, usize)> {
    // take current seat grid and return vec of tuples with index of seen neighbours
    let mut seen_neighbours: Vec<(usize, usize)> = Vec::new();
    // UP, UP-RIGHT, RIGHT, DOWN-RIGHT, DOWN, DOWN-LEFT, LEFT, UP-LEFT
    let directions: [(i32, i32); 8] = [
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
    ];
    for direction in &directions {
        let mut curr_seat = (
            seat_idx.0 as i32 + direction.0,
            seat_idx.1 as i32 + direction.1,
        );
        loop {
            if is_within_bounds((seats.len() - 1, seats[0].len() - 1), curr_seat) {
                match seats[curr_seat.0 as usize][curr_seat.1 as usize] {
                    Cell::Floor => {
                        // continue going in that direction
                        curr_seat = (curr_seat.0 + direction.0, curr_seat.1 + direction.1);
                        continue;
                    }
                    Cell::Taken | Cell::Empty => {
                        // add first seen seat to vec of seen neighbours and move to next direction
                        seen_neighbours.push((curr_seat.0 as usize, curr_seat.1 as usize));
                        break;
                    }
                }
            } else {
                // break out of infinite loop and pick next direction
                break;
            }
        }
    }
    seen_neighbours
}

fn is_within_bounds(bounds: (usize, usize), indexes: (i32, i32)) -> bool {
    match indexes {
        (row_idx, col_idx) => {
            if row_idx < 0 || col_idx < 0 {
                return false;
            } else {
                if row_idx > bounds.0 as i32 || col_idx > bounds.1 as i32 {
                    return false;
                } else {
                    return true;
                }
            }
        }
    }
}

fn find_final_seats(seats: &Vec<Vec<Cell>>, part: u8) -> Vec<Vec<Cell>> {
    match cycle_once(seats, part) {
        Err(final_seats) => return final_seats,
        Ok(new_seats) => find_final_seats(&new_seats, part),
    }
}

fn cycle_once(seats: &Vec<Vec<Cell>>, part: u8) -> Result<Vec<Vec<Cell>>, Vec<Vec<Cell>>> {
    // take in old generation, run logic once, return new seat arrangement inside a Result
    // Ok for a changed arrangement, Err for unchanged
    let mut new_seats: Vec<Vec<Cell>> = Vec::new();
    let mut is_changed = false;
    for (line_idx, line) in seats.iter().enumerate() {
        // initialize line with a placeholder so it doesn't panic when we mutate this spot later
        new_seats.push(Vec::new());

        for (column_idx, seat) in line.iter().enumerate() {
            // initialize column with a placeholder so it doesn't panic when we mutate this spot later
            new_seats[line_idx].push(Cell::Empty);

            let neighbours = match part {
                1 => find_neighbours(seats, (line_idx, column_idx)),
                2 => find_seen_neighbours(seats, (line_idx, column_idx)),
                _ => panic!("invalid part configuration"),
            };

            let new_seat = match seat {
                Cell::Floor => Cell::Floor,
                Cell::Empty => {
                    if neighbours.iter().all(|&(row, col)| {
                        seats[row][col] == Cell::Empty || seats[row][col] == Cell::Floor
                    }) {
                        is_changed = true;
                        Cell::Taken
                    } else {
                        Cell::Empty
                    }
                }
                Cell::Taken => {
                    if neighbours
                        .iter()
                        .filter(|&(row, col)| seats[*row][*col] == Cell::Taken)
                        .count()
                        >= match part {
                            1 => 4,
                            2 => 5,
                            _ => panic!("invalid part configuration"),
                        }
                    {
                        is_changed = true;
                        Cell::Empty
                    } else {
                        Cell::Taken
                    }
                }
            };
            // insert new_seat
            // this is where the error is happening: thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0',
            // do I need to initialize the two dimensional new_seats vector?
            new_seats[line_idx][column_idx] = new_seat;
        }
    }
    if is_changed {
        Ok(new_seats)
    } else {
        Err(new_seats)
    }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        fn parse_line(line: &str) -> Vec<Cell> {
            line.chars()
                .map(|c| parse_c(c))
                .collect()
        }
        fn parse_c(c: char) -> Cell {
            match c {
                'L' => Cell::Empty,
                '.' => Cell::Floor,
                '#' => Cell::Taken,
                _ => panic!("invalid seat option found"),
            }
        }

        Ok(Self(
            input
                .lines()
                .map(|line| parse_line(line))
                .collect(),
        ))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let result = find_final_seats(&self.0, 1)
            .iter()
            .flatten()
            .filter(|&seat| *seat == Cell::Taken)
            .count();

        Ok(result)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let result = find_final_seats(&self.0, 2)
            .iter()
            .flatten()
            .filter(|&seat| *seat == Cell::Taken)
            .count();

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "37");
    }

    #[test]
    fn part_2() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "26");
    }
}
