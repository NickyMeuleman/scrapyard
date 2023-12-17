use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    fmt::Display,
};

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn forward(&self, dir: &Direction, rows: usize, cols: usize) -> Option<Self> {
        let coord = match dir {
            Direction::Up if self.row > 0 => Coord {
                row: self.row - 1,
                col: self.col,
            },
            Direction::Down if self.row < (rows - 1) => Coord {
                row: self.row + 1,
                col: self.col,
            },
            Direction::Left if self.col > 0 => Coord {
                row: self.row,
                col: self.col - 1,
            },
            Direction::Right if self.col < (cols - 1) => Coord {
                row: self.row,
                col: self.col + 1,
            },
            _ => return None,
        };
        Some(coord)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Crucible {
    cost: u32,
    pos: Coord,
    dir: Direction,
    moves_in_dir: u8,
}

// The priority queue holds Crucibles
// We define an ordering trait so the one with the lowest cost gets popped from the pq first.
// We do this by flipping the ordering on cost (comparing "other to self" instead of "self to other")
// that way, crucibles with a lower cost will compare as Ordering::Greater, and get sent to the front of the pq
impl Ord for Crucible {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

// Ensure partialOrd is consistent with Ord. If you #[derive(PartialOrd)] this it might not be the same as that implementation uses a top-down ordering on the Node struct fields
// in this case, it would order by idx first (as that field occurs first in the source code where Node is defined) and would not be consistent.
// From the docs:
// > If Ord is also implemented for Self and Rhs, it must also be consistent with partial_cmp (see the documentation of that trait for the exact requirements).
// > It’s easy to accidentally make them disagree by deriving some of the traits and manually implementing others.
impl PartialOrd for Crucible {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Crucible {
    fn successors(&self, grid: &Vec<Vec<u8>>) -> Vec<Self> {
        let rows = grid.len();
        let cols = grid[0].len();

        let mut successord = Vec::new();
        for dir in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if self.dir == dir && self.moves_in_dir == 3 {
                // already moved 3 tiles in a straight line, can't move further
                continue;
            }
            if self.dir.opposite() == dir {
                // can't move in opposite direction
                continue;
            }
            // simulate a move inside the bounds
            if let Some(pos) = self.pos.forward(&dir, rows, cols) {
                // calculate the total cost to get to that neighbour
                // it's the total cost to get to the current node + the cost to travel to the neighbour
                let cost = self.cost + grid[pos.row][pos.col] as u32;

                // increment straight_moves if we went straight, else we moved 1 tile in the current direction
                let moves_in_dir = if self.dir == dir {
                    self.moves_in_dir + 1
                } else {
                    1
                };

                successord.push(Crucible {
                    pos,
                    cost,
                    dir,
                    moves_in_dir,
                })
            }
        }

        successord
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct UltraCrucible {
    cost: u32,
    pos: Coord,
    dir: Direction,
    moves_in_dir: u8,
}

impl Ord for UltraCrucible {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for UltraCrucible {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl UltraCrucible {
    fn successors(&self, grid: &Vec<Vec<u8>>) -> Vec<Self> {
        let rows = grid.len();
        let cols = grid[0].len();

        let mut successors = Vec::new();
        for dir in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            // Once an ultra crucible starts moving in a direction, it needs to move a minimum of four blocks in that direction before it can turn
            if self.moves_in_dir < 4 && dir != self.dir {
                continue;
            }
            // an ultra crucible can move a maximum of ten consecutive blocks without turning.
            if self.dir == dir && self.moves_in_dir == 10 {
                // already moved 3 tiles in a straight line, can't move further
                continue;
            }
            if self.dir.opposite() == dir {
                // can't move in opposite direction
                continue;
            }
            // simulate a move inside the bounds
            if let Some(pos) = self.pos.forward(&dir, rows, cols) {
                // calculate the total cost to get to that neighbour
                // it's the total cost to get to the current node + the cost to travel to the neighbour
                let cost = self.cost + grid[pos.row][pos.col] as u32;

                // increment straight_moves if we went straight, else we moved 1 tile in the current direction
                let moves_in_dir = if self.dir == dir {
                    self.moves_in_dir + 1
                } else {
                    1
                };

                successors.push(UltraCrucible {
                    pos,
                    cost,
                    dir,
                    moves_in_dir,
                })
            }
        }

        successors
    }
}

#[derive(Debug, Clone)]
pub struct Data(Vec<Vec<u8>>);

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let grid = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| {
                        c.to_digit(10)
                            .map(|n| n as u8)
                            .ok_or(AoCError::Parsing)
                    })
                    .collect::<AoCResult<Vec<u8>>>()
            })
            .collect::<AoCResult<Vec<Vec<u8>>>>()?;
        Ok(Self(grid))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let goal = Coord {
            row: self.0.len() - 1,
            col: self.0[0].len() - 1,
        };

        let mut pq = BinaryHeap::new();
        let mut seen = HashSet::new();

        let right = Crucible {
            cost: self.0[0][1] as u32,
            dir: Direction::Right,
            pos: Coord { row: 0, col: 1 },
            moves_in_dir: 1,
        };
        pq.push(right);

        let down = Crucible {
            cost: self.0[1][0] as u32,
            dir: Direction::Down,
            pos: Coord { row: 1, col: 0 },
            moves_in_dir: 1,
        };
        pq.push(down);

        while let Some(crucible) = pq.pop() {
            if crucible.pos == goal {
                return Ok(crucible.cost);
            }
            for successor in crucible.successors(&self.0) {
                if seen.insert((successor.pos, successor.dir, successor.moves_in_dir)) {
                    pq.push(successor);
                }
            }
        }

        Err(AoCError::Solving)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let goal = Coord {
            row: self.0.len() - 1,
            col: self.0[0].len() - 1,
        };

        let mut pq = BinaryHeap::new();
        let mut seen = HashSet::new();

        let right = UltraCrucible {
            cost: self.0[0][1] as u32,
            dir: Direction::Right,
            pos: Coord { row: 0, col: 1 },
            moves_in_dir: 1,
        };
        pq.push(right);

        let down = UltraCrucible {
            cost: self.0[1][0] as u32,
            dir: Direction::Down,
            pos: Coord { row: 1, col: 0 },
            moves_in_dir: 1,
        };
        pq.push(down);

        while let Some(ultra_crucible) = pq.pop() {
            // it needs to move a minimum of four blocks in that direction before it can turn (or even before it can stop at the end)
            if ultra_crucible.pos == goal && ultra_crucible.moves_in_dir >= 4 {
                return Ok(ultra_crucible.cost);
            }
            for successor in ultra_crucible.successors(&self.0) {
                if seen.insert((successor.pos, successor.dir, successor.moves_in_dir)) {
                    pq.push(successor);
                }
            }
        }

        Err(AoCError::Solving)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "102");
    }

    #[test]
    fn part_2() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "94");
    }
}
