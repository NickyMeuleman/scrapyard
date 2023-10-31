use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Display,
    iter,
};

use aoc_core::{AoCError, Solution};

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data {
    map: HashMap<Coord, Tile>,
    rows: usize,
    cols: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn neighbours(&self, rows: usize, cols: usize) -> Vec<Self> {
        use Direction::*;
        let mut neighbours = Vec::new();
        if self.row > 0 {
            neighbours.push(self.add_dir(&Up));
        }
        if self.col < cols - 1 {
            neighbours.push(self.add_dir(&Right));
        }
        if self.row < rows - 1 {
            neighbours.push(self.add_dir(&Down));
        }
        if self.col > 0 {
            neighbours.push(self.add_dir(&Left));
        }
        neighbours
    }

    fn add_dir(&self, dir: &Direction) -> Self {
        use Direction::*;
        match dir {
            Up => Coord {
                row: self.row - 1,
                col: self.col,
            },
            Right => Coord {
                row: self.row,
                col: self.col + 1,
            },
            Down => Coord {
                row: self.row + 1,
                col: self.col,
            },
            Left => Coord {
                row: self.row,
                col: self.col - 1,
            },
        }
    }

    fn manhattan(&self, other: Coord) -> usize {
        other.col.abs_diff(self.col) + other.row.abs_diff(self.row)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Tile {
    Wall,
    Blizzard(Direction),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(PartialEq, Eq)]
struct Node {
    cost: usize,
    heuristic: usize,
    pos: Coord,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_total = self.cost + self.heuristic;
        let other_total = other.cost + other.heuristic;
        other_total.cmp(&self_total)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn lcm(lhs: usize, rhs: usize) -> usize {
    lhs * rhs / gcd(lhs, rhs)
}

fn gcd(lhs: usize, rhs: usize) -> usize {
    let mut max = lhs;
    let mut min = rhs;
    if min > max {
        std::mem::swap(&mut max, &mut min);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn bliz_maps(
    map: &HashMap<Coord, Tile>,
    rows: usize,
    cols: usize,
    max_time: usize,
) -> HashMap<usize, HashSet<Coord>> {
    // key: turn, val: set of a bliz locations
    let mut cache = HashMap::new();

    let mut blizzards: Vec<(Coord, Direction)> = map
        .iter()
        .filter_map(|(pos, tile)| match tile {
            Tile::Wall => None,
            Tile::Blizzard(dir) => Some((*pos, *dir)),
        })
        .collect();

    let coords = blizzards
        .iter()
        .map(|(coord, _)| *coord)
        .collect();
    cache.insert(0, coords);

    // precompute every blizzard coord at every time before the coords repeat
    for time in 1..max_time {
        for (coord, dir) in blizzards.iter_mut() {
            *coord = coord.add_dir(dir);
            // if next coord went to an edge, wrap
            match dir {
                Direction::Left => {
                    if coord.col == 0 {
                        coord.col = cols - 2;
                    }
                }
                Direction::Right => {
                    if coord.col == cols - 1 {
                        coord.col = 1;
                    }
                }
                Direction::Up => {
                    if coord.row == 0 {
                        coord.row = rows - 2;
                    }
                }
                Direction::Down => {
                    if coord.row == rows - 1 {
                        coord.row = 1;
                    }
                }
            }
        }
        let coords = blizzards
            .iter()
            .map(|(coord, _)| *coord)
            .collect();
        cache.insert(time, coords);
    }

    cache
}

struct MapInfo {
    rows: usize,
    cols: usize,
    walls: HashSet<Coord>,
    blizzard_maps: HashMap<usize, HashSet<Coord>>,
    repeats_at: usize,
}

fn shortest(from: Coord, to: Coord, start_time: usize, map_info: &MapInfo) -> usize {
    let MapInfo {
        rows,
        cols,
        walls,
        blizzard_maps,
        repeats_at,
    } = map_info;

    let mut pq = BinaryHeap::new();
    // backtracking is allowed, keep track of visited coords at a certain time
    let mut seen = HashSet::new();

    pq.push(Node {
        cost: start_time,
        heuristic: from.manhattan(to),
        pos: from,
    });
    seen.insert((from, start_time));

    // keep stepping through time until the priority queue is empty
    while let Some(Node { cost, pos, .. }) = pq.pop() {
        // did we pop a node that's at the target position? It's guaranteed to be the shortest path
        if pos == to {
            return cost;
        }

        let new_cost = cost + 1;
        let blizzards = &blizzard_maps[&(new_cost % repeats_at)];

        let candidates = pos
            // moving to a neighbour is an option
            .neighbours(*rows, *cols)
            .into_iter()
            // not moving is an option
            .chain(iter::once(pos))
            // can not share a coordinate with a wall
            .filter(|coord| !walls.contains(coord))
            // can not share a coordinate with a blizzard
            .filter(|coord| !blizzards.contains(coord));

        for new_pos in candidates {
            // only push to pq if we didn't already see that coord at the same time
            if seen.insert((new_pos, new_cost)) {
                pq.push(Node {
                    cost: new_cost,
                    heuristic: new_pos.manhattan(to),
                    pos: new_pos,
                });
            }
        }
    }
    usize::MAX
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut map = HashMap::new();

        let rows = input.lines().count();
        let cols = input
            .lines()
            .next()
            .ok_or(AoCError::Parsing)?
            .chars()
            .count();

        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c == '.' {
                    continue;
                }
                let coord = Coord { row, col };
                let tile = match c {
                    '#' => Tile::Wall,
                    '^' => Tile::Blizzard(Direction::Up),
                    'v' => Tile::Blizzard(Direction::Down),
                    '<' => Tile::Blizzard(Direction::Left),
                    '>' => Tile::Blizzard(Direction::Right),
                    _ => return Err(AoCError::Parsing),
                };
                map.insert(coord, tile);
            }
        }
        Ok(Self { map, rows, cols })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let walls: HashSet<Coord> = self
            .map
            .iter()
            .filter(|(_, tile)| **tile == Tile::Wall)
            .map(|(pos, _)| *pos)
            .collect();
        // lcm of inner area without the walls. patterns repeat every lcm steps
        let lcm = lcm(self.rows - 2, self.cols - 2);
        let blizzard_maps = bliz_maps(&self.map, self.rows, self.cols, lcm);
        let start = Coord { row: 0, col: 1 };
        let end = Coord {
            row: self.rows - 1,
            col: self.cols - 2,
        };
        let map_info = MapInfo {
            rows: self.rows,
            cols: self.cols,
            repeats_at: lcm,
            walls,
            blizzard_maps,
        };

        Ok(shortest(start, end, 0, &map_info))
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let walls: HashSet<Coord> = self
            .map
            .iter()
            .filter(|(_, tile)| **tile == Tile::Wall)
            .map(|(pos, _)| *pos)
            .collect();
        // lcm of inner area without the walls. patterns repeat every lcm steps
        let lcm = lcm(self.rows - 2, self.cols - 2);
        let blizzard_maps = bliz_maps(&self.map, self.rows, self.cols, lcm);
        let start = Coord { row: 0, col: 1 };
        let end = Coord {
            row: self.rows - 1,
            col: self.cols - 2,
        };
        let map_info = MapInfo {
            rows: self.rows,
            cols: self.cols,
            repeats_at: lcm,
            walls,
            blizzard_maps,
        };

        let there = shortest(start, end, 0, &map_info);
        let back = shortest(end, start, there, &map_info);

        Ok(shortest(start, end, back, &map_info))
    }

    fn solve(self) -> AoCResult<Solution>
    where
        Self: Sized,
    {
        let walls: HashSet<Coord> = self
            .map
            .iter()
            .filter(|(_, tile)| **tile == Tile::Wall)
            .map(|(pos, _)| *pos)
            .collect();
        // lcm of inner area without the walls. patterns repeat every lcm steps
        let lcm = lcm(self.rows - 2, self.cols - 2);
        let blizzard_maps = bliz_maps(&self.map, self.rows, self.cols, lcm);
        let start = Coord { row: 0, col: 1 };
        let end = Coord {
            row: self.rows - 1,
            col: self.cols - 2,
        };

        let map_info = MapInfo {
            rows: self.rows,
            cols: self.cols,
            repeats_at: lcm,
            walls,
            blizzard_maps,
        };

        let there = shortest(start, end, 0, &map_info);
        let back = shortest(end, start, there, &map_info);
        let there_again = shortest(start, end, back, &map_info);

        Ok(Solution {
            part1: Box::new(there),
            part2: Box::new(there_again),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "18");
    }

    #[test]
    fn part_2() {
        let input = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "54");
    }
}
