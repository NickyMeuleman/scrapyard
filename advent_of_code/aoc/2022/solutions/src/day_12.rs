use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    fmt::Display,
};

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data {
    map: Vec<Vec<u8>>,
    start: Coord,
    end: Coord,
    rows: usize,
    cols: usize,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Node {
    cost: u32,
    coord: Coord,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Coord {
    fn neighbours(&self, rows: usize, cols: usize) -> impl Iterator<Item = Self> {
        let mut result = Vec::new();

        // up
        if self.y > 0 {
            result.push(Self {
                x: self.x,
                y: self.y - 1,
            });
        }
        // down
        if self.y < rows - 1 {
            result.push(Self {
                x: self.x,
                y: self.y + 1,
            });
        }
        // left
        if self.x > 0 {
            result.push(Self {
                x: self.x - 1,
                y: self.y,
            });
        }
        // right
        if self.x < cols - 1 {
            result.push(Self {
                x: self.x + 1,
                y: self.y,
            });
        }

        result.into_iter()
    }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let rows = input.lines().count();
        let cols = input
            .lines()
            .next()
            .ok_or(AoCError::new("Failed Parsing"))?
            .len();
        let mut map = vec![vec![0; cols]; rows];
        let mut start = Coord { x: 0, y: 0 };
        let mut end = Coord { x: 0, y: 0 };

        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                let letter = match c {
                    'S' => {
                        start.x = col;
                        start.y = row;
                        'a'
                    }
                    'E' => {
                        end.x = col;
                        end.y = row;
                        'z'
                    }
                    'a'..='z' => c,
                    _ => return Err(AoCError::new("Failed Parsing")),
                };

                let val = letter as u8 - b'a';
                map[row][col] = val;
            }
        }

        Ok(Self {
            map,
            start,
            end,
            cols,
            rows,
        })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let Data {
            start,
            end,
            map,
            rows,
            cols,
        } = self;
        let mut pq = BinaryHeap::new();
        let mut visited = HashSet::new();

        pq.push(Node {
            cost: 0,
            coord: *start,
        });
        visited.insert(*start);

        while let Some(Node { coord, cost }) = pq.pop() {
            if coord == *end {
                return Ok(cost);
            }

            let curr_height = map[coord.y][coord.x];
            let neighbours = coord.neighbours(*rows, *cols);
            let candidates = neighbours.into_iter().filter(|coord| {
                let height = map[coord.y][coord.x];
                height <= curr_height || height == curr_height + 1
            });

            for candidate in candidates {
                if visited.insert(candidate) {
                    pq.push(Node {
                        cost: cost + 1,
                        coord: candidate,
                    })
                }
            }
        }

        Err(AoCError::new("No path found"))
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let Data {
            end,
            map,
            rows,
            cols,
            ..
        } = self;
        let mut pq = BinaryHeap::new();
        let mut visited = HashSet::new();

        pq.push(Node {
            cost: 0,
            coord: *end,
        });
        visited.insert(*end);

        while let Some(Node { coord, cost }) = pq.pop() {
            let curr_height = map[coord.y][coord.x];

            if curr_height == 0 {
                return Ok(cost);
            }

            let neighbours = coord.neighbours(*rows, *cols);
            let candidates = neighbours.into_iter().filter(|coord| {
                let height = map[coord.y][coord.x];
                height >= curr_height || height == curr_height - 1
            });

            for candidate in candidates {
                if visited.insert(candidate) {
                    pq.push(Node {
                        cost: cost + 1,
                        coord: candidate,
                    })
                }
            }
        }

        Err(AoCError::new("No path found"))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "31");
    }

    #[test]
    fn part_2() {
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "29");
    }
}
