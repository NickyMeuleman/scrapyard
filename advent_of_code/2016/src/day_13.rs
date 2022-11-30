use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

use crate::AoCData;

pub struct Data(i32);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
    fn is_open(&self, constant: i32) -> bool {
        let val = self.x * self.x
            + 3 * self.x
            + 2 * self.x * self.y
            + self.y
            + self.y * self.y
            + constant;

        val.count_ones() % 2 == 0
    }

    fn neighbours(&self) -> Vec<Self> {
        let mut result = Vec::new();

        // up
        if self.y > 0 {
            result.push(Self {
                x: self.x,
                y: self.y - 1,
            });
        }
        // down
        result.push(Self {
            x: self.x,
            y: self.y + 1,
        });
        // left
        if self.x > 0 {
            result.push(Self {
                x: self.x - 1,
                y: self.y,
            });
        }
        // right
        result.push(Self {
            x: self.x + 1,
            y: self.y,
        });

        result
    }

    fn shortest_to_goal(self, goal: &Coord, constant: i32) -> u32 {
        let mut pq = BinaryHeap::new();
        let mut visited = HashSet::new();

        pq.push(Node {
            cost: 0,
            coord: self,
        });
        // not keeping track of cost to reach node in visited:
        // because the first visit is guaranteed to be the lowest-cost visit
        // because every node only ever increases by 1 in cost
        visited.insert(self);

        while let Some(Node { coord, cost }) = pq.pop() {
            if coord == *goal {
                return cost;
            }

            for coord in coord
                .neighbours()
                .iter()
                .filter(|coord| coord.is_open(constant))
            {
                if visited.insert(*coord) {
                    pq.push(Node {
                        cost: cost + 1,
                        coord: *coord,
                    });
                }
            }
        }

        u32::MAX
    }

    fn reachable_in_steps(self, step_count: u32, constant: i32) -> u32 {
        let mut pq = BinaryHeap::new();
        let mut visited = HashSet::new();

        pq.push(Node {
            cost: 0,
            coord: self,
        });
        visited.insert(self);

        while let Some(Node { cost, coord }) = pq.pop() {
            for coord in coord
                .neighbours()
                .iter()
                .filter(|coord| coord.is_open(constant))
            {
                let new_cost = cost + 1;
                if new_cost <= step_count && visited.insert(*coord) {
                    pq.push(Node {
                        cost: cost + 1,
                        coord: *coord,
                    });
                }
            }
        }

        visited.len() as u32
    }
}

fn part_1_helper(num: i32, start: Coord, goal: Coord) -> u32 {
    let constant = num;
    start.shortest_to_goal(&goal, constant)
}
impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        Some(Self(input.parse().ok()?))
    }

    fn part_1(&self) -> String {
        part_1_helper(self.0, Coord { x: 1, y: 1 }, Coord { x: 31, y: 39 }).to_string()
    }

    fn part_2(&self) -> String {
        let start = Coord { x: 1, y: 1 };
        let constant = self.0;
        let max_steps = 50;
        start.reachable_in_steps(max_steps, constant).to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(13);
        let data = Data::try_new(input).unwrap();
        assert_eq!(
            part_1_helper(data.0, Coord { x: 1, y: 1 }, Coord { x: 7, y: 4 }),
            11
        );
    }

    #[test]
    fn part_2() {
        let input = utils::get_input(13);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "127");
    }
}
