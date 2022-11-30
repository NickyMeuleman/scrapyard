use md5::{Digest, Md5};
use std::{cmp::Ordering, collections::BinaryHeap, iter};

use crate::{utils::Solution, AoCData};

const HEIGHT: u8 = 4;
const WIDTH: u8 = 4;
const DIRECTIONS: [char; 4] = ['U', 'D', 'L', 'R'];

pub struct Data(String);

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

fn open_doors(passcode: &String, path: &[char]) -> [bool; 4] {
    let mut hasher = Md5::new();
    hasher.update(passcode.as_bytes());
    hasher.update(path.iter().map(|c| *c as u8).collect::<Vec<_>>());
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

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        Some(Self(input))
    }

    fn part_1(&self) -> String {
        let state = State {
            path: Vec::new(),
            pos: Coord { x: 0, y: 0 },
        };
        let mut pq = BinaryHeap::new();
        pq.push(Node { cost: 0, state });

        while let Some(node) = pq.pop() {
            if node.state.pos.x == WIDTH - 1 && node.state.pos.y == HEIGHT - 1 {
                return node.state.path.iter().collect();
            }
            let doors = open_doors(&self.0, &node.state.path);
            let neighbours = node.state.pos.neighbours();
            // tried itertools izip! instead but that didn't work and said DIRECTIONS items were an Options for some reason
            let open_neighbours = doors.iter().zip(neighbours).zip(DIRECTIONS).filter_map(
                |((open, pos), dir)| match (open, pos, dir) {
                    (true, Some(pos), dir) => Some((dir, pos)),
                    _ => None,
                },
            );

            for (dir, pos) in open_neighbours {
                pq.push(Node {
                    cost: node.cost + 1,
                    state: State {
                        path: Vec::from_iter(
                            node.state.path.clone().into_iter().chain(iter::once(dir)),
                        ),
                        pos,
                    },
                });
            }
        }
        String::new()
    }

    fn part_2(&self) -> String {
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
            let open_neighbours = doors.iter().zip(neighbours).zip(DIRECTIONS).filter_map(
                |((open, pos), dir)| match (open, pos, dir) {
                    (true, Some(pos), dir) => Some((dir, pos)),
                    _ => None,
                },
            );

            for (dir, pos) in open_neighbours {
                pq.push(Node {
                    cost: node.cost + 1,
                    state: State {
                        path: Vec::from_iter(
                            node.state.path.clone().into_iter().chain(iter::once(dir)),
                        ),
                        pos,
                    },
                });
            }
        }
        longest.to_string()
    }

    fn solve(self) -> crate::utils::Solution
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
            let open_neighbours = doors.iter().zip(neighbours).zip(DIRECTIONS).filter_map(
                |((open, pos), dir)| match (open, pos, dir) {
                    (true, Some(pos), dir) => Some((dir, pos)),
                    _ => None,
                },
            );

            for (dir, pos) in open_neighbours {
                pq.push(Node {
                    cost: node.cost + 1,
                    state: State {
                        path: Vec::from_iter(
                            node.state.path.clone().into_iter().chain(iter::once(dir)),
                        ),
                        pos,
                    },
                });
            }
        }

        match (shortest, longest) {
            (Some(part1), Some(part2)) => Solution {
                part1,
                part2: part2.to_string(),
            },
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(17);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "DRURDRUDDLLDLUURRDULRLDUUDDDRR");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(17);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "830");
    }

    #[test]
    fn solve() {
        let input = utils::get_sample_input(17);
        let data = Data::try_new(input).unwrap();
        let Solution { part1, part2 } = data.solve();
        assert_eq!(part1, "DRURDRUDDLLDLUURRDULRLDUUDDDRR");
        assert_eq!(part2, "830");
    }
}
