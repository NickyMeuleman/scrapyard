use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::{convert::Infallible, str::FromStr};

// Thank you @jeffomatic, your code was extremely educational

#[derive(Debug, Clone)]
pub struct Data {
    map: HashMap<Point, usize>,
    num_rows: usize,
    num_cols: usize,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn new(row: usize, col: usize) -> Self {
        Point { row, col }
    }

    fn neighbours(&self, num_rows: usize, num_cols: usize) -> Vec<Point> {
        let mut result = Vec::new();

        // UP
        if self.row > 0 {
            result.push(Point::new(self.row - 1, self.col));
        }
        // DOWN
        if self.row < num_rows - 1 {
            result.push(Point::new(self.row + 1, self.col));
        }
        // LEFT
        if self.col > 0 {
            result.push(Point::new(self.row, self.col - 1));
        }
        // RIGHT
        if self.col < num_cols - 1 {
            result.push(Point::new(self.row, self.col + 1));
        }

        result
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Node {
    pos: Point,
    cost: usize,
}

// used in Dijkstra's algorithm
// flip the standard ordering
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            // compare the position in case of a tie.
            // first compare the row, and in case that also ties, compare the col
            .then_with(|| match self.pos.row.cmp(&other.pos.row) {
                Ordering::Equal => self.pos.col.cmp(&other.pos.col),
                ordering => ordering,
            })
    }
}

// used in Dijkstra's algorithm
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Data {
    pub fn part_one(&self) -> usize {
        self.lowest_cost()
    }

    pub fn part_two(&self) -> usize {
        // calculate necessary data for a new Data struct
        // it has a new rowsize, colsize, and a map with costs for each point
        let num_cols = self.num_cols * 5;
        let num_rows = self.num_rows * 5;
        let mut map: HashMap<Point, usize> = HashMap::new();

        // the new map is 5x the width and 5x the height
        // the original map repeats to fill this new size with these rules:
        // costs increase by 1 for every new step to the right
        // costs increase by 1 for every new step to the bottom
        // if a cost would go above 9, it is wrapped to 0 instead
        for tile_row in 0..5 {
            for tile_col in 0..5 {
                for vertical_step in 0..self.num_rows {
                    for horizontal_step in 0..self.num_cols {
                        let mut cost = self
                            .map
                            .get(&Point::new(vertical_step, horizontal_step))
                            .unwrap()
                            + tile_row
                            + tile_col;
                        if cost > 9 {
                            cost -= 9;
                        }
                        map.insert(
                            Point::new(
                                tile_row as usize * self.num_rows + vertical_step,
                                tile_col as usize * self.num_cols + horizontal_step,
                            ),
                            cost,
                        );
                    }
                }
            }
        }

        let cave = Data {
            map,
            num_cols,
            num_rows,
        };

        cave.lowest_cost()
    }

    /// calculates the lowest cost to get from the top left point to the bottom right point
    /// implementation: Dijkstra's algoritme
    fn lowest_cost(&self) -> usize {
        // new priority queue
        let mut pq: BinaryHeap<Node> = BinaryHeap::new();

        // start by putting 2 nodes in the queue
        // one to the right of the start position
        // one to the bottom of the start position
        // these are the only 2 initial ones because the start is the top left corner
        pq.push(Node {
            pos: Point::new(0, 1),
            cost: *self.map.get(&Point::new(0, 1)).unwrap(),
        });
        pq.push(Node {
            pos: Point::new(1, 0),
            cost: *self.map.get(&Point::new(1, 0)).unwrap(),
        });

        // a hashmap to track a point, and the lowest cost so far to reach that point
        let mut lowest_costs: HashMap<Point, usize> = HashMap::new();
        // add the starting point as 0 cost in order not to count it
        lowest_costs.insert(Point::new(0, 0), 0);

        // pop something off the priority queue, do stuff, repeat until the queue is empty
        while let Some(node) = pq.pop() {
            // if the node we popped off the queue is the bottom right point, we're done
            // return the cost to get to that point
            if node.pos == Point::new(self.num_rows - 1, self.num_cols - 1) {
                return node.cost;
            }

            // if a way to get to the current point with a lower (or equal) cost exists, skip this iteration
            if let Some(lowest_cost) = lowest_costs.get(&node.pos) {
                if *lowest_cost <= node.cost {
                    continue;
                }
            }

            // loop through every neighbour of the current point
            for neighbour in node.pos.neighbours(self.num_rows, self.num_cols) {
                // calculate the total cost to get to this neighbour
                // it's the cost for the current NODE plus the cost of this neighbour POINT in the map
                let cost = node.cost
                    + self
                        .map
                        .get(&Point::new(neighbour.row, neighbour.col))
                        .unwrap();

                // if a way to get to the neighbour point with a lower (or equal) cost exists, skip this iteration
                if let Some(lowest_cost) = lowest_costs.get(&neighbour) {
                    if *lowest_cost <= cost {
                        continue;
                    }
                }

                // push the created node onto the priority queue
                pq.push(Node {
                    pos: neighbour,
                    cost,
                });

                // the node we popped off the queue had the lowest cost to get to that point
                // insert the Point to cost into the HashMap that keeps track of all lowest total costs
                lowest_costs.insert(node.pos.clone(), node.cost);
            }
        }

        // there is a lowest cost path, so this should never be reached
        // we should return from our while loop when we pop our goal position off the priority queue
        unreachable!();
    }
}

impl FromStr for Data {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let map = input
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars().enumerate().map(move |(col, c)| {
                    let num = c.to_digit(10).unwrap() as usize;
                    let point = Point { row, col };
                    (point, num)
                })
            })
            .collect();

        let num_rows = input.lines().count();
        let num_cols = input.lines().next().unwrap().len();

        Ok(Self {
            map,
            num_rows,
            num_cols,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_one(), 40);
    }

    #[test]
    fn part_two_example() {
        let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_two(), 315);
    }
}
