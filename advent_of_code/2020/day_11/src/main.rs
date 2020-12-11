use itertools::Itertools;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let seats: Vec<Vec<Cell>> = parse(&input);
    println!("Part one answer: {}", part_one(&seats));
    // println!("Part two answer: {}", part_two(&seats.clone()));
}

#[derive(Debug, PartialEq)]
enum Cell {
    Taken,
    Empty,
    Floor,
}

fn parse(input: &str) -> Vec<Vec<Cell>> {
    input.lines().map(|line| parse_line(line)).collect()
}

fn parse_line(line: &str) -> Vec<Cell> {
    line.chars().map(|c| parse_c(c)).collect()
}

fn parse_c(c: char) -> Cell {
    match c {
        'L' => Cell::Empty,
        '.' => Cell::Floor,
        '#' => Cell::Taken,
        _ => panic!("invalid seat option found"),
    }
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

// fn find_neighbours(seats: &Vec<Vec<Cell>>, seat_idx: (u32, u32)) -> Vec<(u32, u32)> {
//     // take current seat grid and return vec of tuples with index of neighbours
//     // from question: adjacent to a given seat (one of the eight positions immediately up, down, left, right, or diagonal from the seat).

//     // edit: just realized, I way overcomplicated this, simple math would do it too. It works though.
//     let mut neighbours: Vec<(u32, u32)> = Vec::new();
//     for (line_idx, line) in seats.iter().enumerate() {
//         // CHRISTMAS TREE NESTING VIBES ONLY
//         if line_idx as u32 == seat_idx.0
//             || line_idx as u32 == seat_idx.0 + 1
//             || (seat_idx.0 as i32 - 1 > 0 && line_idx as u32 == seat_idx.0 - 1)
//         {
//             for (column_idx, _) in line.iter().enumerate() {
//                 if column_idx as u32 == seat_idx.1
//                     || column_idx as u32 == seat_idx.1 + 1
//                     || (seat_idx.1 as i32 - 1 > 0 && column_idx as u32 == seat_idx.1 - 1)
//                 {
//                     let neighbour = (line_idx as u32, column_idx as u32);
//                     neighbours.push(neighbour);
//                 }
//             }
//         }
//     }
//     neighbours = neighbours
//         .into_iter()
//         .filter(|neighbour| *neighbour != seat_idx)
//         .collect::<Vec<(u32, u32)>>();
//     neighbours
// }

fn part_one(seats: &Vec<Vec<Cell>>) -> usize {
    find_final_seats(seats)
        .iter()
        .flatten()
        .filter(|&seat| *seat == Cell::Taken)
        .count()
}

fn find_final_seats(seats: &Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
    match cycle_once(seats) {
        Err(final_seats) => return final_seats,
        Ok(new_seats) => find_final_seats(&new_seats),
    }
}

fn cycle_once(seats: &Vec<Vec<Cell>>) -> Result<Vec<Vec<Cell>>, Vec<Vec<Cell>>> {
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

            let neighbours = find_neighbours(seats, (line_idx, column_idx));
            let new_seat = match seat {
                Cell::Floor => Cell::Floor,
                Cell::Empty => {
                    if neighbours
                        .iter()
                        .all(|&(row, col)| seats[row][col] == Cell::Empty)
                    {
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
                        >= 4
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