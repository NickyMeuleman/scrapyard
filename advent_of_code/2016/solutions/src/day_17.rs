use std::{cmp::Ordering, collections::BinaryHeap, fmt::Display, iter};

use aoc_core::{AoCError, Solution};
use md5::{Digest, Md5};

use crate::{AoCData, AoCResult};

const HEIGHT: u8 = 4;
const WIDTH: u8 = 4;
const DIRECTIONS: [char; 4] = ['U', 'D', 'L', 'R'];

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

#[derive(Debug, PartialEq, Eq, Clone)]
struct State {
    path: Vec<char>,
    pos: Coord,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Node {
    cost: u32,
    state: State,
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

fn open_doors(passcode: &str, path: &[char]) -> [bool; 4] {
    let mut hasher = Md5::new();
    hasher.update(passcode.as_bytes());
    hasher.update(
        path.iter()
            .map(|c| *c as u8)
            .collect::<Vec<_>>(),
    );
    let digest = hasher.finalize();
    let mut first_four = Vec::new();
    for &j in digest.iter().take(2) {
        for &k in &[j >> 4 & 0xF, j & 0xF] {
            first_four.push(k);
        }
    }

    [
        (11..=16).contains(&first_four[0]),
        (11..=16).contains(&first_four[1]),
        (11..=16).contains(&first_four[2]),
        (11..=16).contains(&first_four[3]),
    ]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord {
    x: u8,
    y: u8,
}

impl Coord {
    fn neighbours(&self) -> [Option<Coord>; 4] {
        let up = if self.y > 0 {
            Some(Coord {
                x: self.x,
                y: self.y - 1,
            })
        } else {
            None
        };
        let down = if self.y < HEIGHT - 1 {
            Some(Coord {
                x: self.x,
                y: self.y + 1,
            })
        } else {
            None
        };
        let left = if self.x > 0 {
            Some(Coord {
                x: self.x - 1,
                y: self.y,
            })
        } else {
            None
        };
        let right = if self.x < WIDTH - 1 {
            Some(Coord {
                x: self.x + 1,
                y: self.y,
            })
        } else {
            None
        };

        [up, down, left, right]
    }
}

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let state = State {
            path: Vec::new(),
            pos: Coord { x: 0, y: 0 },
        };
        let mut pq = BinaryHeap::new();
        pq.push(Node { cost: 0, state });

        while let Some(node) = pq.pop() {
            if node.state.pos.x == WIDTH - 1 && node.state.pos.y == HEIGHT - 1 {
                return Ok(node
                    .state
                    .path
                    .iter()
                    .collect::<String>());
            }
            let doors = open_doors(self.0, &node.state.path);
            let neighbours = node.state.pos.neighbours();
            // tried itertools izip! instead but that didn't work and said DIRECTIONS items were an Options for some reason
            let open_neighbours = doors
                .iter()
                .zip(neighbours)
                .zip(DIRECTIONS)
                .filter_map(|((open, pos), dir)| match (open, pos, dir) {
                    (true, Some(pos), dir) => Some((dir, pos)),
                    _ => None,
                });

            for (dir, pos) in open_neighbours {
                pq.push(Node {
                    cost: node.cost + 1,
                    state: State {
                        path: Vec::from_iter(
                            node.state
                                .path
                                .clone()
                                .into_iter()
                                .chain(iter::once(dir)),
                        ),
                        pos,
                    },
                });
            }
        }
        Err(AoCError::Solving)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let state = State {
            path: Vec::new(),
            pos: Coord { x: 0, y: 0 },
        };
        let mut pq = BinaryHeap::new();
        pq.push(Node { cost: 0, state });
        let mut longest = 0;
        while let Some(node) = pq.pop() {
            if node.state.pos.x == WIDTH - 1 && node.state.pos.y == HEIGHT - 1 {
                longest = node.cost;
                continue;
            }
            let doors = open_doors(&self.0, &node.state.path);
            let neighbours = node.state.pos.neighbours();
            let open_neighbours = doors
                .iter()
                .zip(neighbours)
                .zip(DIRECTIONS)
                .filter_map(|((open, pos), dir)| match (open, pos, dir) {
                    (true, Some(pos), dir) => Some((dir, pos)),
                    _ => None,
                });

            for (dir, pos) in open_neighbours {
                pq.push(Node {
                    cost: node.cost + 1,
                    state: State {
                        path: Vec::from_iter(
                            node.state
                                .path
                                .clone()
                                .into_iter()
                                .chain(iter::once(dir)),
                        ),
                        pos,
                    },
                });
            }
        }
        Ok(longest)
    }

    fn solve(self) -> AoCResult<Solution>
    where
        Self: Sized,
    {
        let state = State {
            path: Vec::new(),
            pos: Coord { x: 0, y: 0 },
        };
        let mut pq = BinaryHeap::new();
        pq.push(Node { cost: 0, state });
        let mut shortest: Option<String> = None;
        let mut longest = None;
        while let Some(node) = pq.pop() {
            if node.state.pos.x == WIDTH - 1 && node.state.pos.y == HEIGHT - 1 {
                if shortest.is_none() {
                    shortest = Some(node.state.path.iter().collect());
                }
                longest = Some(node.cost);
                continue;
            }
            let doors = open_doors(&self.0, &node.state.path);
            let neighbours = node.state.pos.neighbours();
            let open_neighbours = doors
                .iter()
                .zip(neighbours)
                .zip(DIRECTIONS)
                .filter_map(|((open, pos), dir)| match (open, pos, dir) {
                    (true, Some(pos), dir) => Some((dir, pos)),
                    _ => None,
                });

            for (dir, pos) in open_neighbours {
                pq.push(Node {
                    cost: node.cost + 1,
                    state: State {
                        path: Vec::from_iter(
                            node.state
                                .path
                                .clone()
                                .into_iter()
                                .chain(iter::once(dir)),
                        ),
                        pos,
                    },
                });
            }
        }

        match (shortest, longest) {
            (Some(part1), Some(part2)) => Ok(Solution {
                part1: Box::new(part1),
                part2: Box::new(part2),
            }),
            _ => Err(AoCError::Solving),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "ulqzkmiv";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "DRURDRUDDLLDLUURRDULRLDUUDDDRR");
    }

    #[test]
    fn part_2() {
        let input = "ulqzkmiv";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "830");
    }

    #[test]
    fn solve() {
        let input = "ulqzkmiv";
        let data = Data::try_new(input).unwrap();
        let Solution { part1, part2 } = data.solve().unwrap();
        assert_eq!(part1.to_string(), "DRURDRUDDLLDLUURRDULRLDUUDDDRR");
        assert_eq!(part2.to_string(), "830");
    }
}
