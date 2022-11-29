use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use crate::AoCData;

pub struct Data(HashMap<Coord, StorageNode>);

#[derive(Debug, PartialEq, Clone)]
struct StorageNode {
    size: u16,
    used: u16,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct State {
    goal: Coord,
    empty: Coord,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Node {
    cost: u32,
    heuristic: u32,
    state: State,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        let other_total = other.cost + other.heuristic;
        let self_total = self.cost + self.heuristic;
        other_total.cmp(&self_total)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl StorageNode {
    fn avail(&self) -> u16 {
        self.size - self.used
    }
}

impl Coord {
    fn neighbours(&self, width: i32, height: i32) -> [Option<Self>; 4] {
        let up = if self.y > 0 {
            Some(Coord {
                x: self.x,
                y: self.y - 1,
            })
        } else {
            None
        };
        let down = if self.y < height {
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
        let right = if self.x < width {
            Some(Coord {
                x: self.x + 1,
                y: self.y,
            })
        } else {
            None
        };

        [up, down, left, right]
    }

    fn manhatten(&self, other: Coord) -> u32 {
        other.x.abs_diff(self.x) + other.y.abs_diff(self.y)
    }
}
impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        let mut nodes = HashMap::new();
        for line in input.trim().lines().skip(2) {
            let mut words = line.split_whitespace();
            let pos = words.next()?.strip_prefix("/dev/grid/node-")?;
            let (x, y) = pos.split_once('-')?;
            let x = x.strip_prefix('x')?.parse().ok()?;
            let y = y.strip_prefix('y')?.parse().ok()?;
            let size = words.next()?.strip_suffix('T')?.parse().ok()?;
            let used = words.next()?.strip_suffix('T')?.parse().ok()?;
            nodes.insert(Coord { x, y }, StorageNode { size, used });
        }
        Some(Self(nodes))
    }

    fn part_1(&self) -> String {
        // Node A is not empty (its Used is not zero).
        // Nodes A and B are not the same node.
        // The data on node A (its Used) would fit on node B (its Avail).
        self.0
            .iter()
            .flat_map(|(pos_a, node_a)| {
                self.0.iter().filter(move |(pos_b, node_b)| {
                    node_a.used != 0 && pos_a != *pos_b && node_a.used <= node_b.avail()
                })
            })
            .count()
            .to_string()
    }

    fn part_2(&self) -> String {
        let storage_nodes = self.0.clone();
        let empty = *storage_nodes
            .iter()
            .find(|(_coord, node)| node.used == 0)
            .expect("empty node must exist")
            .0;
        let (height, width) =
            storage_nodes
                .iter()
                .fold((0, 0), |(height, width), (coord, _node)| {
                    let width = width.max(coord.x);
                    let height = height.max(coord.y);
                    (height, width)
                });
        let wanted = Coord { x: 0, y: 0 };
        let goal = Coord { x: width, y: 0 };
        let state = State { empty, goal };
        let mut seen = HashSet::new();
        let mut pq = BinaryHeap::new();
        seen.insert(state.clone());
        pq.push(Node {
            cost: 0,
            heuristic: goal.manhatten(wanted),
            state,
        });
        while let Some(node) = pq.pop() {
            if node.state.goal == wanted {
                return node.cost.to_string();
            }
            for candidate in node
                .state
                .empty
                .neighbours(width, height)
                .iter()
                .filter_map(|coord| *coord)
                .filter(|coord| {
                    let neighbour = storage_nodes
                        .get(coord)
                        .expect("couldn't find coord in nodes");
                    let empty = storage_nodes.get(&node.state.empty).unwrap();
                    empty.size >= neighbour.used
                })
            {
                let new_state = if candidate == node.state.goal {
                    State {
                        empty: candidate,
                        goal: node.state.empty,
                    }
                } else {
                    State {
                        empty: candidate,
                        goal: node.state.goal,
                    }
                };
                if seen.insert(new_state.clone()) {
                    let new_cost = node.cost + 1;
                    let new_heuristic = new_state.empty.manhatten(new_state.goal)
                        + new_state.goal.manhatten(wanted);
                    pq.push(Node {
                        cost: new_cost,
                        heuristic: new_heuristic,
                        state: new_state,
                    });
                }
            }
        }
        String::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(22);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "7");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(22);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "7");
    }
}
