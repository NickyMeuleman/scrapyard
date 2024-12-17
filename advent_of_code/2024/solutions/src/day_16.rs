use crate::{AoCData, AoCResult};
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Display, rc::Rc,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    row: i32,
    col: i32,
}

impl Point {
    fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }

    fn add(&self, other: &Self) -> Self {
        Self::new(self.row + other.row, self.col + other.col)
    }

    fn clockwise(&self) -> Self {
        Point::new(self.col, -self.row)
    }

    fn counter_clockwise(&self) -> Self {
        Point::new(-self.col, self.row)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
    pos: Point,
    dir: Point,
    cost: u32,
}

// The priority queue holds Nodes
// We define an ordering trait so the one with the lowest cost gets popped from the pq first.
// We do this by flipping the ordering on cost (comparing "other to self" instead of "self to other")
// that way, nodes with a lower cost will compare as Ordering::Greater, and get sent to the front of the pq
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
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

fn moves(map: &HashMap<Point, Tile>, pos: Point, dir: Point) -> Vec<(Point, Point, u32)> {
    let mut moves = Vec::new();

    // move forward one
    let new_loc = pos.add(&dir);
    if let Some(&tile) = map.get(&new_loc) {
        if tile != Tile::Wall {
            moves.push((new_loc, dir, 1));
        }
    }
    // turn clockwise
    moves.push((pos, dir.clockwise(), 1000));
    // turn counterclockwise
    moves.push((pos, dir.counter_clockwise(), 1000));

    moves
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Start,
    End,
}

#[derive(Debug, Clone)]
pub struct Data {
    map: HashMap<Point, Tile>,
    start: Point,
    end: Point,
}

fn shortest_path(map: &HashMap<Point, Tile>, start: Point, end: Point) -> u32 {
    // total cost to reach a point facing a direction
    let mut shortest: HashMap<(Point, Point), u32> = HashMap::new();
    // priority queue with ordering so the Node with the lowest cost gets popped first
    let mut pq = BinaryHeap::new();

    // insert the initial position, facing east into the pq
    pq.push(Node {
        pos: start,
        dir: Point::new(0, 1),
        cost: 0,
    });

    // keep popping from the pq until it is empty (no path found), or we break out of the loop (shortest path found)
    while let Some(node) = pq.pop() {
        // did we pop our goal position off the pq?
        // if so, it is guaranteed to have the lowest cost, and we're done
        if node.pos == end {
            return node.cost;
        }

        // did we pop something off the pq that got a lower-cost route to it in between the time it was added to the pq and popped off the pq?
        // (this can happen when there are multiple ways to reach a node, and a lower cost one is discovered while this node was in the pq)
        // if yes, disregard (don't update the shortest-list OR look at the possible next steps for this node.
        // Looking at the next steps would calculate costs for paths through this node, but it's not the lowest cost way to reach it.)
        if let Some(&lowest) = shortest.get(&(node.pos, node.dir)) {
            if lowest < node.cost {
                continue;
            }
        }

        // the node we popped from the pq was the shortest path to that location, facint that direction. Update the shortest-map
        shortest.insert((node.pos, node.dir), node.cost);

        // look at all possible moves
        for (new_pos, new_dir, move_cost) in moves(map, node.pos, node.dir) {
            // calculate the total cost if you execute that move
            let new_cost = node.cost + move_cost;

            // if the new cost to get to that (new_pos, new_dir) pair isn't lower than what is already in the shortest-list, move on to the next move
            if let Some(&lowest) = shortest.get(&(new_pos, new_dir)) {
                if lowest <= new_cost {
                    continue;
                }
            }

            // add that move onto the pq
            pq.push(Node {
                pos: new_pos,
                dir: new_dir,
                cost: new_cost,
            });

            // important to not exit the loop here if the new position is equal to our goal
            // we might not have looked at the lowest-cost way to reach the goal yet
        }
    }

    unreachable!("No path found")
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Node2 {
    pos: Point,
    dir: Point,
    from: Option<Rc<Node2>>,
    cost: u32,
}

impl Ord for Node2 {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_paths_positions(map: &HashMap<Point, Tile>, start: Point, end: Point) -> usize {
    // keep track of all positions along a shortest path
    let mut best_positions = HashSet::new();
    best_positions.insert(start);
    best_positions.insert(end);
    // keep track of shortest cost from start to end
    let mut lowest_cost = u32::MAX;
    // total cost to reach a point facing a direction
    let mut shortest: HashMap<(Point, Point), u32> = HashMap::new();
    // priority queue with ordering so the Node with the lowest cost gets popped first
    let mut pq = BinaryHeap::new();

    pq.push(Node2 {
        pos: start,
        dir: Point::new(0, 1),
        from: None,
        cost: 0,
    });

    while let Some(node) = pq.pop() {
        // did we pop our goal position off the pq?
        // if so, it is guaranteed to have the lowest cost, and this completes a shortest path
        if node.pos == end {
            lowest_cost = lowest_cost.min(node.cost);
            if node.cost > lowest_cost {
                // we went further than the minimum cost, stop looping
                break;
            }
            // reconstruct the path
            let mut curr = Rc::new(node);
            while curr.pos != start {
                best_positions.insert(curr.pos);
                if let Some(prev) = &curr.from {
                    curr = prev.clone();
                }
            }
            continue;
        }

        if let Some(&lowest) = shortest.get(&(node.pos, node.dir)) {
            if lowest < node.cost {
                continue;
            }
        }

        shortest.insert((node.pos, node.dir), node.cost);

        for (new_pos, new_dir, move_cost) in moves(map, node.pos, node.dir) {
            let new_cost = node.cost + move_cost;

            if let Some(&lowest) = shortest.get(&(new_pos, new_dir)) {
                if lowest <= new_cost {
                    continue;
                }
            }

            pq.push(Node2 {
                pos: new_pos,
                dir: new_dir,
                cost: new_cost,
                // remember where this node came from
                from: Some(Rc::new(node.clone())),
            });
        }
    }

    best_positions.len()
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut map = HashMap::new();
        let mut start = Point::new(0, 0);
        let mut end = Point::new(0, 0);
        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                let point = Point::new(row as i32, col as i32);
                let tile = match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Wall,
                    'E' => Tile::End,
                    'S' => Tile::Start,
                    _ => panic!("at the disco"),
                };
                if tile == Tile::Start {
                    start = point;
                }
                if tile == Tile::End {
                    end = point;
                }
                map.insert(point, tile);
            }
        }
        Ok(Self { map, start, end })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        // classic Dijkstra
        Ok(shortest_path(&self.map, self.start, self.end))
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        // Dijkstra, keeping track of all best paths
        Ok(shortest_paths_positions(&self.map, self.start, self.end))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "7036");
    }


    #[test]
    fn part_1_2() {
        let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "11048");
    }


    #[test]
    fn part_2() {
        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "45");
    }

    #[test]
    fn part_2_2() {
        let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "64");
    }
}
