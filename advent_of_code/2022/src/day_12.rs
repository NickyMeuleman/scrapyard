use crate::AoCData;

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
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

pub struct Data {
    map: Vec<Vec<u8>>,
    start: Coord,
    end: Coord,
    rows: usize,
    cols: usize,
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> Option<Self> {
        let rows = input.lines().count();
        let cols = input.lines().next()?.len();
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
                    _ => return None,
                };

                let val = letter as u8 - b'a';
                map[row][col] = val;
            }
        }

        Some(Self {
            map,
            start,
            end,
            cols,
            rows,
        })
    }

    fn part_1(&self) -> String {
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
                return cost.to_string();
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

        "No path found".to_string()
    }

    fn part_2(&self) -> String {
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
                return cost.to_string();
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

        "No path found".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(12);
        let data = Data::try_new(&input).unwrap();
        assert_eq!(data.part_1(), "31");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(12);
        let data = Data::try_new(&input).unwrap();
        assert_eq!(data.part_2(), "29");
    }
}
