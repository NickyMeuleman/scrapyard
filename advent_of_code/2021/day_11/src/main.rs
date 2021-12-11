use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Data {
    map: HashMap<Point, u8>,
    rows: u8,
    cols: u8,
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        for row_idx in 0..self.rows {
            let mut line = String::new();

            for col_idx in 0..self.cols {
                let num = *self
                    .map
                    .get(&Point {
                        row: row_idx as u8,
                        col: col_idx as u8,
                    })
                    .unwrap();
                line.push(char::from_digit(num as u32, 10).unwrap());
            }

            result.push_str(&line);
            result.push_str("\n");
        }

        write!(f, "{}", result)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    row: u8,
    col: u8,
}

impl Point {
    fn neighbours(&self, data: &Data) -> Vec<Point> {
        let offsets: [(i8, i8); 8] = [
            (0, 1),
            (0, -1),
            (1, 0),
            (1, 1),
            (1, -1),
            (-1, 0),
            (-1, 1),
            (-1, -1),
        ];

        let mut neighbours = Vec::new();

        for (row_offset, col_offset) in offsets {
            // ugly casting because the offsets can be negative, but the final result can not
            // as a result, this can overflow, but it's fiiiiiiiiiiiiiine
            // the better option is saturing_add_negative(), but that's nightly only
            let row = (self.row as i8 + row_offset as i8) as u8;
            let col = (self.col as i8 + col_offset as i8) as u8;

            let point = Point { row, col };
            if point.within_bounds(data.rows - 1, data.cols - 1) {
                neighbours.push(point);
            }
        }

        neighbours
    }

    fn within_bounds(&self, max_row: u8, max_col: u8) -> bool {
        self.row <= max_row && self.col <= max_col
    }
}

impl Data {
    pub fn part_one(&self) -> u16 {
        let mut data = self.clone();
        (0..100).map(|_| data.tick()).sum()
    }

    fn part_two(&self) -> u16 {
        let mut data = self.clone();

        (1..)
            .find_map(|step| {
                // if they all flashed, stop
                if data.tick() == (data.rows as u16 * data.cols as u16) {
                    Some(step)
                } else {
                    None
                }
            })
            .unwrap()
    }

    /// function that performs one tick and returns the amount of resulting flashes
    fn tick(&mut self) -> u16 {
        let mut flashes = 0;
        let mut queue: VecDeque<Point> = VecDeque::new();
        let mut visited: HashSet<Point> = HashSet::new();

        // First, the energy level of each octopus increases by 1.
        for (point, energy) in self.map.iter_mut() {
            // increase energy by one
            *energy += 1;
            // check if energy is now higher than 9
            if *energy > 9 {
                // add it to the queue and mark it as visited (it should flash, but only once)
                queue.push_back(*point);
                visited.insert(*point);
            }
        }

        // while there are items in the queue, continue calculating next states
        while let Some(point) = queue.pop_front() {
            let neighbours: Vec<Point> = point.neighbours(&self);
            for neighbour in neighbours {
                // increase energy
                let energy = self.map.get_mut(&neighbour).unwrap();
                *energy += 1;
                // check if energy is now > 9 AND the neighbour is unvisited
                if *energy > 9 && !visited.contains(&neighbour) {
                    // add neighbour to the queue and mark it as visited
                    queue.push_back(neighbour);
                    visited.insert(neighbour);
                }
            }
        }

        // Finally, any octopus that flashed during this step has its energy level set to 0,
        // as it used all of its energy to flash.
        for (_, energy) in self.map.iter_mut() {
            // check if energy is now higher than 9
            if *energy > 9 {
                // flash now, as we didn't at any previous point
                flashes += 1;
                // reset energy
                *energy = 0;
            }
        }

        flashes
    }
}

impl FromStr for Data {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.trim();
        let rows = input.lines().count() as u8;
        let cols = input.lines().nth(0).unwrap().len() as u8;

        let map = input
            .lines()
            .enumerate()
            .flat_map(|(row_idx, line)| {
                line.chars().enumerate().map(move |(col_idx, c)| {
                    let num = c.to_digit(10).unwrap() as u8;
                    let point = Point {
                        row: row_idx as u8,
                        col: col_idx as u8,
                    };
                    (point, num)
                })
            })
            .collect();

        Ok(Self { map, rows, cols })
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let data: Data = input.parse().unwrap();
    println!("Part one answer: {}", data.clone().part_one());
    println!("Part two answer: {}", data.part_two());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_one_example() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_one(), 1656);
    }

    #[test]

    fn part_two_example() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_two(), 195);
    }
}
