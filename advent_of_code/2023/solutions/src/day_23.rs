use std::fmt::Display;

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn neighbours1(&self, map: &Vec<Vec<Tile>>) -> Vec<Coord> {
        let rows = map.len();
        let cols = map[0].len();
        let mut res = Vec::new();

        // up
        if self.row > 0 {
            let pos = Coord {
                row: self.row - 1,
                col: self.col,
            };
            let tile = map[pos.row][pos.col];
            let possible = match tile {
                Tile::Open => true,
                Tile::Slope(Dir::Up) => true,
                _ => false,
            };
            if possible {
                res.push(pos);
            }
        }

        // down
        if self.row < rows - 1 {
            let pos = Coord {
                row: self.row + 1,
                col: self.col,
            };
            let tile = map[pos.row][pos.col];
            let possible = match tile {
                Tile::Open => true,
                Tile::Slope(Dir::Down) => true,
                _ => false,
            };
            if possible {
                res.push(pos);
            }
        }

        // left
        if self.col > 0 {
            let pos = Coord {
                row: self.row,
                col: self.col - 1,
            };
            let tile = map[pos.row][pos.col];
            let possible = match tile {
                Tile::Open => true,
                Tile::Slope(Dir::Left) => true,
                _ => false,
            };
            if possible {
                res.push(pos);
            }
        }

        // right
        if self.col < cols - 1 {
            let pos = Coord {
                row: self.row,
                col: self.col + 1,
            };
            let tile = map[pos.row][pos.col];
            let possible = match tile {
                Tile::Open => true,
                Tile::Slope(Dir::Right) => true,
                _ => false,
            };
            if possible {
                res.push(pos);
            }
        }

        res
    }

    fn neighbours2(self, map: &Vec<Vec<Tile>>) -> impl Iterator<Item = Self> + '_ {
        let rows = map.len();
        let cols = map[0].len();

        let up = if self.row > 0 {
            Some(Self {
                row: self.row - 1,
                col: self.col,
            })
        } else {
            None
        };

        let down = if self.row < rows - 1 {
            Some(Self {
                row: self.row + 1,
                col: self.col,
            })
        } else {
            None
        };

        let left = if self.col > 0 {
            Some(Self {
                row: self.row,
                col: self.col - 1,
            })
        } else {
            None
        };

        let right = if self.col < cols - 1 {
            Some(Self {
                row: self.row,
                col: self.col + 1,
            })
        } else {
            None
        };

        [up, down, left, right]
            .into_iter()
            .filter_map(|pos| pos)
            .filter(|pos| map[pos.row][pos.col] != Tile::Rock)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Rock,
    Open,
    Slope(Dir),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Data {
    start: Coord,
    end: Coord,
    map: Vec<Vec<Tile>>,
}

fn longest(
    from: Coord,
    to: Coord,
    map: &HashMap<Coord, HashMap<Coord, usize>>,
) -> AoCResult<usize> {
    let mut q = VecDeque::new();
    let mut max = 0;

    q.push_back((from, 0, HashSet::from([from])));

    while let Some((pos, cost, seen)) = q.pop_front() {
        if pos == to {
            max = cost.max(max);
            continue;
        }

        for (n, add) in map
            .get(&pos)
            .ok_or(AoCError::Solving)?
            .iter()
            .filter(|(pos, _)| !seen.contains(pos))
        {
            let mut new_seen = seen.clone();
            new_seen.insert(*n);
            q.push_back((*n, cost + add, new_seen))
        }
    }

    Ok(max)
}

fn all_forks(map: &Vec<Vec<Tile>>) -> HashSet<Coord> {
    let mut res = HashSet::new();

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            let pos = Coord { row, col };
            let tile = map[pos.row][pos.col];
            if tile != Tile::Rock && pos.neighbours2(map).count() > 2 {
                res.insert(pos);
            }
        }
    }

    res
}

fn costmap(points: &HashSet<Coord>, map: &Vec<Vec<Tile>>) -> HashMap<Coord, HashMap<Coord, usize>> {
    let initial = HashMap::from_iter(
        points
            .iter()
            .map(|node| (*node, HashMap::new())),
    );

    points
        .iter()
        .fold(initial, |mut acc, point| {
            // add the cost of every reachable point.
            // when you reach a point, keep going and remember where you've been so you don't try to visit impossible points
            let mut q: VecDeque<(Coord, usize)> = VecDeque::new();
            let mut seen: HashSet<Coord> = HashSet::new();
            q.push_back((*point, 0));

            while let Some((pos, cost)) = q.pop_front() {
                // record costs for positions in the points set (the condensed map)
                if points.contains(&pos) && cost != 0 {
                    *acc.entry(*point)
                        .or_default()
                        .entry(pos)
                        .or_default() = cost;
                    continue;
                }

                // go to an adjacent tile if it's not already seen during this path
                for n in pos.neighbours2(map) {
                    if seen.insert(n) {
                        q.push_back((n, cost + 1));
                    }
                }

                seen.insert(pos);
            }

            acc
        })
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let rows = input.lines().count();
        let cols = input
            .lines()
            .next()
            .ok_or(AoCError::Parsing)?
            .chars()
            .count();

        let start = Coord { row: 0, col: 1 };
        let end = Coord {
            row: rows - 1,
            col: cols - 2,
        };

        let map = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Ok(Tile::Open),
                        '#' => Ok(Tile::Rock),
                        '^' => Ok(Tile::Slope(Dir::Up)),
                        'v' => Ok(Tile::Slope(Dir::Down)),
                        '<' => Ok(Tile::Slope(Dir::Left)),
                        '>' => Ok(Tile::Slope(Dir::Right)),
                        _ => Err(AoCError::Parsing),
                    })
                    .collect::<AoCResult<Vec<Tile>>>()
            })
            .collect::<AoCResult<Vec<Vec<Tile>>>>()?;

        Ok(Self { start, end, map })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut q: VecDeque<(Coord, usize, HashSet<Coord>)> = VecDeque::new();
        let mut max = 0;

        q.push_back((self.start, 0, HashSet::from([self.start])));

        while let Some((pos, cost, mut seen)) = q.pop_front() {
            if pos == self.end {
                max = cost.max(max);
                continue;
            }

            for n in pos.neighbours1(&self.map) {
                if seen.insert(n) {
                    q.push_back((n, cost + 1, seen.clone()))
                }
            }
        }

        Ok(max)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        // only care about the interesting points, every fork in the road, the start, and the end
        // (this collapses all single path chains into a single point)
        let mut points = all_forks(&self.map);
        points.insert(self.start);
        points.insert(self.end);

        let costmap = costmap(&points, &self.map);

        longest(self.start, self.end, &costmap)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "94");
    }

    #[test]
    fn part_2() {
        let input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "154");
    }
}
