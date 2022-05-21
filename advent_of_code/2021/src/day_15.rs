use crate::AoCData;
use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(PartialEq, Eq)]
struct Node {
    idx: usize,
    cost: u32,
}

// The priority queue holds Nodes
// We define an ordering trait so the one with the lowest cost gets popped from the pq first.
// We do this by flipping the ordering on cost (comparing "other to self" instead of "self to other")
// that way, nodes with a lower cost will compare as Ordering::Greater, and get sent to the front of the pq
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            // if the costs are equal,we compare on index as tie-breaker
            .then_with(|| other.idx.cmp(&self.idx))
    }
}

// Ensure partialOrd is consistent with Ord. If you #[derive(PartialOrd)] this it might not be the same as that implementation uses a top-down ordering on the Node struct fields
// in this case, it would order by idx first (as that field occurs first in the source code where Node is defined) and would not be consistent.
// From the docs:
// > If Ord is also implemented for Self and Rhs, it must also be consistent with partial_cmp (see the documentation of that trait for the exact requirements).
// > It’s easy to accidentally make them disagree by deriving some of the traits and manually implementing others.
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn neighbours(idx: usize, rows: usize, cols: usize) -> Vec<usize> {
    let (row, col) = (idx / cols, idx % rows);
    let mut neighbours = Vec::new();

    // up
    if row > 0 {
        neighbours.push((row - 1) * cols + col)
    }

    // down
    if row < rows - 1 {
        neighbours.push((row + 1) * cols + col)
    }

    // left
    if col > 0 {
        neighbours.push(row * cols + (col - 1))
    }

    // right
    if col < cols - 1 {
        neighbours.push(row * cols + (col + 1))
    }

    neighbours
}

pub struct Data {
    rows: usize,
    cols: usize,
    map: Vec<u8>,
}

impl Data {
    fn shortest_path(&self) -> u32 {
        // index to total cost to reach that index
        let mut shortest = vec![u32::MAX; self.map.len()];
        // priority queue with ordering so the Node with the lowest cost gets popped first
        // a Node has a position index, and a total cost to travel to that position from the starting point
        let mut pq = BinaryHeap::new();

        // start by adding the starting point to our shortest-map
        // the total cost to travel there is 0, not whatever is in the input-map
        shortest[0] = 0;

        // insert the initial options to travel into the pq
        // right neighbour
        pq.push(Node {
            idx: 1,
            cost: self.map[1] as u32,
        });
        // down neighbour
        pq.push(Node {
            idx: self.cols,
            cost: self.map[self.cols] as u32,
        });

        // keep popping from the pq until it is empty (no path found), or we break out of the loop (shortest path found)
        while let Some(node) = pq.pop() {
            // did we pop our goal position off the pq?
            // if so, it is guaranteed to have the lowest cost, and we're done
            if node.idx == self.map.len() - 1 {
                return node.cost;
            }

            // did we pop something off the pq that got a lower-cost route to it in between the time it was added to the pq and popped off the pq?
            // (this can happen when there are multiple ways to reach a node, and a lower cost one is discovered while this node was in the pq)
            // if yes, disregard (don't update the shortest-list OR look at the neighbours of this node.
            // Looking at the neighbours would calculate costs for paths through this node's index, but it's not the lowest cost way to reach this index.)
            if shortest[node.idx] <= node.cost {
                continue;
            }

            // the node we popped from the pq was the shortest path to that index, update the shortest-map
            shortest[node.idx] = node.cost;

            // look at all neighbours of the current node
            for n_idx in neighbours(node.idx, self.rows, self.cols) {
                // calculate the total cost to get to that neighbour
                // it's the total cost to get to the current node + the cost to travel to the neighbour
                let cost = node.cost + self.map[n_idx] as u32;

                // if the new cost to get to the neighbour isn't lower than what is already in the shortest-list, move on to the next neighbour
                if shortest[n_idx] <= cost {
                    continue;
                }

                // add that neighbour node onto the pq
                pq.push(Node { idx: n_idx, cost });

                // important to not exit the loop here if the neighbour index is equal to our goal
                // we might not have looked at the lowest-cost way to reach the goal yet
            }
        }

        unreachable!("No path found")
    }
}

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        let rows = input.lines().count();
        let cols = input.lines().next()?.len();
        let map = input
            .trim()
            .lines()
            .flat_map(|line| line.chars().map(|c| c.to_digit(10).map(|digit| digit as u8)))
            .collect::<Option<Vec<u8>>>()?;

        Some(Self { rows, cols, map })
    }

    fn part_1(&self) -> String {
        self.shortest_path().to_string()
    }

    fn part_2(&self) -> String {
        let cols = 5 * self.cols;
        let rows = 5 * self.rows;
        let mut big_map = vec![0; rows * cols];

        for tile_row in 0..5 {
            for tile_col in 0..5 {
                for vertical_step in 0..self.rows {
                    for horizontal_step in 0..self.cols {
                        let mut cost = self
                            .map
                            .get(vertical_step * self.cols + horizontal_step)
                            .unwrap()
                            + tile_row
                            + tile_col;
                        if cost > 9 {
                            cost -= 9;
                        }

                        let new_row = tile_row as usize * self.rows + vertical_step;
                        let new_col = tile_col as usize * self.cols + horizontal_step;
                        big_map[new_row * cols + new_col] = cost;
                    }
                }
            }
        }

        let bigger_cave = Data {
            map: big_map,
            rows,
            cols,
        };

        bigger_cave.shortest_path().to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(15);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "40");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(15);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "315");
    }
}
