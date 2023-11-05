use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Display,
};

use aoc_core::{AoCError, Solution};
use itertools::Itertools;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data {
    map: HashMap<Coord, Kind>,
    width: usize,
    height: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn neighbours(&self, width: usize, height: usize) -> [Option<Self>; 4] {
        let up = if self.y > 0 {
            Some(Coord {
                x: self.x,
                y: self.y - 1,
            })
        } else {
            None
        };
        let down = if self.y < height - 1 {
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
        let right = if self.x < width - 1 {
            Some(Coord {
                x: self.x + 1,
                y: self.y,
            })
        } else {
            None
        };

        [up, down, left, right]
    }

    fn manhattan(&self, other: Coord) -> usize {
        other.x.abs_diff(self.x) + other.y.abs_diff(self.y)
    }
}

#[derive(Debug, Clone)]
enum Kind {
    Wall,
    Open,
    Number(u32),
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Node {
    cost: usize,
    heuristic: usize,
    pos: Coord,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.cost + other.heuristic).cmp(&(self.cost + self.heuristic))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Data {
    fn min_steps(&self, from: &Coord, to: &Coord) -> usize {
        let mut pq = BinaryHeap::new();
        let mut visited = HashSet::new();

        pq.push(Node {
            cost: 0,
            heuristic: from.manhattan(*to),
            pos: *from,
        });
        visited.insert(*from);

        while let Some(node) = pq.pop() {
            if node.pos == *to {
                return node.cost;
            }

            // trying to store the candidates iterator in a seperate variable
            // causes "creates a temporary which is freed while still in use".
            // so I'm going with the less readable version where the Y in "for X in Y" is a large piece of logic
            for candidate in node
                .pos
                .neighbours(self.width, self.height)
                .iter()
                // only keep neighbours that are within bounds
                .filter_map(|res| *res)
                // only keep neighbours that are not a wall
                .filter(|coord| !matches!(self.map.get(coord), Some(Kind::Wall)))
            {
                // only visit the candidate if it has not been visited before
                if visited.insert(candidate) {
                    pq.push(Node {
                        cost: node.cost + 1,
                        heuristic: candidate.manhattan(*to),
                        pos: candidate,
                    });
                }
            }
        }

        usize::MAX
    }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut map = HashMap::new();
        let height = input.lines().count();
        let width = input
            .lines()
            .next()
            .ok_or(AoCError::Parsing)?
            .chars()
            .count();

        for (row_idx, line) in input.trim().lines().enumerate() {
            for (col_idx, c) in line.chars().enumerate() {
                let pos = Coord {
                    x: col_idx,
                    y: row_idx,
                };
                let spot = match c {
                    '#' => Kind::Wall,
                    '.' => Kind::Open,
                    s => Kind::Number(
                        s.to_digit(10)
                            .ok_or(AoCError::Parsing)?,
                    ),
                };
                map.insert(pos, spot);
            }
        }
        Ok(Self { height, width, map })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let goals: HashMap<Coord, u32> = self
            .map
            .iter()
            .filter_map(|(coord, spot)| match spot {
                Kind::Number(num) => Some((*coord, *num)),
                _ => None,
            })
            .collect();
        let dist_map = {
            let mut map = HashMap::new();
            for ((pos_a, num_a), (pos_b, num_b)) in goals.iter().tuple_combinations() {
                let steps = self.min_steps(pos_a, pos_b);
                map.insert((num_a, num_b), steps);
                map.insert((num_b, num_a), steps);
            }
            map
        };
        // travelling salesmen problem
        // cost to visit all numbers, calc cost of every way to visit em all, take min.
        let result = goals
            .values()
            .permutations(goals.len())
            // only keep the ones that start at 0
            .filter(|perm| *perm[0] == 0)
            .filter_map(|perm| {
                perm.into_iter()
                    .tuple_windows()
                    .try_fold(0, |acc, (num_a, num_b)| {
                        let steps = dist_map.get(&(num_a, num_b))?;
                        Some(acc + steps)
                    })
            })
            .min()
            .ok_or(AoCError::Solving)?;

        Ok(result)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let goals: HashMap<Coord, u32> = self
            .map
            .iter()
            .filter_map(|(coord, spot)| match spot {
                Kind::Number(num) => Some((*coord, *num)),
                _ => None,
            })
            .collect();
        let dist_map = {
            let mut map = HashMap::new();
            for ((pos_a, num_a), (pos_b, num_b)) in goals.iter().tuple_combinations() {
                let steps = self.min_steps(pos_a, pos_b);
                map.insert((num_a, num_b), steps);
                map.insert((num_b, num_a), steps);
            }
            map
        };
        // travelling salesmen problem
        // cost to visit all numbers, calc cost of every way to visit em all, take min.
        let result = goals
            .values()
            .permutations(goals.len())
            // only keep the ones that start at 0
            .filter(|perm| *perm[0] == 0)
            .filter_map(|mut perm| {
                // end in 0 again
                perm.push(&0);
                perm.into_iter()
                    .tuple_windows()
                    .try_fold(0, |acc, (num_a, num_b)| {
                        let steps = dist_map.get(&(num_a, num_b))?;
                        Some(acc + steps)
                    })
            })
            .min()
            .ok_or(AoCError::Solving)?;

        Ok(result)
    }

    fn solve(self) -> AoCResult<Solution>
    where
        Self: Sized,
    {
        let goals: HashMap<Coord, u32> = self
            .map
            .iter()
            .filter_map(|(coord, spot)| match spot {
                Kind::Number(num) => Some((*coord, *num)),
                _ => None,
            })
            .collect();
        let dist_map = {
            let mut map = HashMap::new();
            for ((pos_a, num_a), (pos_b, num_b)) in goals.iter().tuple_combinations() {
                let steps = self.min_steps(pos_a, pos_b);
                map.insert((num_a, num_b), steps);
                map.insert((num_b, num_a), steps);
            }
            map
        };
        let permutations = goals
            .values()
            .permutations(goals.len())
            // only keep the ones that start at 0
            .filter(|perm| *perm[0] == 0);

        let (p1_costs, p2_costs): (Vec<_>, Vec<_>) = permutations
            .filter_map(|perm| {
                let last_visited = perm[perm.len() - 1];
                let cost_with_leave = perm
                    .into_iter()
                    .tuple_windows()
                    .try_fold(0, |acc, (num_a, num_b)| {
                        let steps = dist_map.get(&(num_a, num_b))?;
                        Some(acc + steps)
                    })?;
                let cost_with_go_back = cost_with_leave + dist_map.get(&(last_visited, &0))?;
                Some((cost_with_leave, cost_with_go_back))
            })
            .unzip();
        let p1_result = p1_costs
            .into_iter()
            .min()
            .ok_or(AoCError::Solving)?;
        let p2_result = p2_costs
            .into_iter()
            .min()
            .ok_or(AoCError::Solving)?;

        Ok(Solution {
            part1: Box::new(p1_result),
            part2: Box::new(p2_result),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "###########
#0.1.....2#
#.#######.#
#4.......3#
###########";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "14");
    }

    #[test]
    fn part_2() {
        let input = "###########
#0.1.....2#
#.#######.#
#4.......3#
###########";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "20");
    }

    #[test]
    fn solve() {
        let input = "###########
#0.1.....2#
#.#######.#
#4.......3#
###########";
        let data = Data::try_new(input).unwrap();
        let Solution { part1, part2 } = data.solve().unwrap();
        assert_eq!(part1.to_string(), "14");
        assert_eq!(part2.to_string(), "20");
    }
}
