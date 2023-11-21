use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

use aoc_core::AoCError;

use crate::{intcode::Computer, AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<i64>);

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Default)]
struct Droid {
    computer: Computer,
    position: Coord,
}

enum Dir {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Tile {
    Wall,
    Empty(bool),
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        Ok(Self(
            input
                .split(',')
                .filter_map(|s| s.parse().ok())
                .collect(),
        ))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut q: VecDeque<(i64, Droid)> = VecDeque::new();
        let mut seen: HashSet<Coord> = HashSet::new();
        let mut droid: Droid = Default::default();
        droid
            .computer
            .set_memory(self.0.clone());
        seen.insert(droid.position.clone());
        q.push_back((0, droid));

        while let Some((dist, droid)) = q.pop_front() {
            // The remote control program executes the following steps in a loop forever:
            // Accept a movement command via an input instruction.
            for dir in [Dir::North, Dir::South, Dir::East, Dir::West] {
                let new_pos = match dir {
                    Dir::North => Coord {
                        x: droid.position.x,
                        y: droid.position.y - 1,
                    },
                    Dir::South => Coord {
                        x: droid.position.x,
                        y: droid.position.y + 1,
                    },
                    Dir::West => Coord {
                        x: droid.position.x - 1,
                        y: droid.position.y,
                    },
                    Dir::East => Coord {
                        x: droid.position.x + 1,
                        y: droid.position.y,
                    },
                };
                if !seen.insert(new_pos.clone()) {
                    continue;
                }
                let mut new_droid = droid.clone();
                // Send the movement command to the repair droid.
                new_droid.computer.input(dir as i64);
                // Wait for the repair droid to finish the movement operation.
                new_droid.computer.run()?;
                // Report on the status of the repair droid via an output instruction.
                let status = new_droid
                    .computer
                    .consume_output()
                    .ok_or(AoCError::Solving)?;
                // The repair droid can reply with any of the following status codes:
                match status {
                    // 0: The repair droid hit a wall. Its position has not changed.
                    0 => {}
                    // 1: The repair droid has moved one step in the requested direction.
                    1 => {
                        new_droid.position = new_pos;
                        q.push_back((dist + 1, new_droid));
                    }
                    // 2: The repair droid has moved one step in the requested direction; its new position is the location of the oxygen system.
                    2 => return Ok(dist + 1),
                    _ => return Err(AoCError::Solving),
                }
            }
        }

        Err(AoCError::Solving)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        // I could do this in two parts, one to find to oxygen tank, and one to floodfill, but I conbined them
        let mut q: VecDeque<(i64, Droid)> = VecDeque::new();
        let mut map: HashMap<Coord, Tile> = HashMap::new();
        let mut droid: Droid = Default::default();
        droid
            .computer
            .set_memory(self.0.clone());
        map.insert(droid.position.clone(), Tile::Empty(false));
        q.push_back((0, droid));

        let mut filling = false;
        let mut res = 0;

        while let Some((time, droid)) = q.pop_front() {
            res = time;
            // The remote control program executes the following steps in a loop forever:
            // Accept a movement command via an input instruction.
            for dir in [Dir::North, Dir::South, Dir::East, Dir::West] {
                let new_pos = match dir {
                    Dir::North => Coord {
                        x: droid.position.x,
                        y: droid.position.y - 1,
                    },
                    Dir::South => Coord {
                        x: droid.position.x,
                        y: droid.position.y + 1,
                    },
                    Dir::West => Coord {
                        x: droid.position.x - 1,
                        y: droid.position.y,
                    },
                    Dir::East => Coord {
                        x: droid.position.x + 1,
                        y: droid.position.y,
                    },
                };

                if filling {
                    if let Some(tile) = map.get(&new_pos) {
                        if *tile == Tile::Empty(true) || *tile == Tile::Wall {
                            continue;
                        }
                    }
                } else {
                    if let Some(tile) = map.get(&new_pos) {
                        if *tile == Tile::Empty(false) || *tile == Tile::Wall {
                            continue;
                        }
                    }
                }
                let mut new_droid = droid.clone();
                // Send the movement command to the repair droid.
                new_droid.computer.input(dir as i64);
                // Wait for the repair droid to finish the movement operation.
                new_droid.computer.run()?;
                // Report on the status of the repair droid via an output instruction.
                let status = new_droid
                    .computer
                    .consume_output()
                    .ok_or(AoCError::Solving)?;
                // The repair droid can reply with any of the following status codes:
                match status {
                    // 0: The repair droid hit a wall. Its position has not changed.
                    0 => {
                        map.insert(new_pos.clone(), Tile::Wall);
                    }
                    // 1: The repair droid has moved one step in the requested direction.
                    1 => {
                        new_droid.position = new_pos.clone();
                        let new_time = if filling { time + 1 } else { 0 };
                        let new_tile = if filling {
                            Tile::Empty(true)
                        } else {
                            Tile::Empty(false)
                        };
                        map.insert(new_pos, new_tile);
                        q.push_back((new_time, new_droid));
                    }
                    // 2: The repair droid has moved one step in the requested direction; its new position is the location of the oxygen system.
                    2 => {
                        filling = true;
                        map.insert(new_pos.clone(), Tile::Empty(true));
                        q.clear();
                        new_droid.position = new_pos.clone();
                        q.push_back((0, new_droid));
                    }
                    _ => return Err(AoCError::Solving),
                }
            }
        }

        Ok(res)
    }
}
