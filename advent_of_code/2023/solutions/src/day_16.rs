use std::fmt::Display;

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

use std::collections::{HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Beam {
    pos: Coord,
    dir: Direction,
}

impl Beam {
    // bounds checking version used in slower version
    // fn forward(mut self, rows: usize, cols: usize) -> Option<Self> {
    //     match self.dir {
    //         Direction::Up if self.pos.y > 0 => self.pos.y -= 1,
    //         Direction::Down if self.pos.y < rows - 1 => self.pos.y += 1,
    //         Direction::Left if self.pos.x > 0 => self.pos.x -= 1,
    //         Direction::Right if self.pos.x < cols - 1 => self.pos.x += 1,
    //         _ => return None,
    //     }
    //     Some(self)
    // }

    fn forward(&mut self) {
        match self.dir {
            Direction::Up => self.pos.y -= 1,
            Direction::Down => self.pos.y += 1,
            Direction::Left => self.pos.x -= 1,
            Direction::Right => self.pos.x += 1,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Tile {
    Empty,
    SplitHoriz,
    SplitVert,
    MirrorForward,
    MirrorBack,
}

#[derive(Debug, Clone)]
pub struct Data(Vec<Vec<Tile>>);

// slower
// fn energized(start: Beam, grid: &[Vec<Tile>]) -> usize {
//     let rows = grid.len();
//     let cols = grid[0].len();

//     let mut q = VecDeque::new();
//     let mut energized = HashSet::new();
//     let mut seen = HashSet::new();
//     q.push_back(start);

//     while let Some(mut beam) = q.pop_front() {
//         if seen.contains(&beam) {
//             continue;
//         }
//         energized.insert(beam.pos);
//         seen.insert(beam);

//         let dirs = match (grid[beam.pos.y][beam.pos.x], beam.dir) {
//             (Tile::Empty, _)
//             | (Tile::SplitHoriz, Direction::Left)
//             | (Tile::SplitHoriz, Direction::Right)
//             | (Tile::SplitVert, Direction::Up)
//             | (Tile::SplitVert, Direction::Down) => vec![beam.dir],
//             (Tile::SplitHoriz, _) => {
//                 vec![Direction::Left, Direction::Right]
//             }
//             (Tile::SplitVert, _) => {
//                 vec![Direction::Up, Direction::Down]
//             }
//             (Tile::MirrorForward, Direction::Up) | (Tile::MirrorBack, Direction::Down) => {
//                 vec![Direction::Right]
//             }
//             (Tile::MirrorForward, Direction::Down) | (Tile::MirrorBack, Direction::Up) => {
//                 vec![Direction::Left]
//             }
//             (Tile::MirrorForward, Direction::Left) | (Tile::MirrorBack, Direction::Right) => {
//                 vec![Direction::Down]
//             }
//             (Tile::MirrorForward, Direction::Right) | (Tile::MirrorBack, Direction::Left) => {
//                 vec![Direction::Up]
//             }
//         };
//         for dir in dirs {
//             beam.dir = dir;
//             if let Some(beam) = beam.forward(rows, cols) {
//                 q.push_back(beam);
//             }
//         }
//     }
//     energized.len()
// }

// faster, checks bounds before changing direction
fn energized(start: Beam, grid: &[Vec<Tile>]) -> usize {
    let mut q = VecDeque::new();
    let mut energized = HashSet::new();
    let mut seen = HashSet::new();
    q.push_back(start);

    while let Some(mut beam) = q.pop_front() {
        if seen.contains(&beam) {
            continue;
        }
        energized.insert(beam.pos);
        seen.insert(beam);

        match (grid[beam.pos.y][beam.pos.x], beam.dir) {
            (Tile::Empty, Direction::Up) => {
                if beam.pos.y == 0 {
                    continue;
                } else {
                    beam.forward();
                    q.push_back(beam);
                }
            }
            (Tile::Empty, Direction::Down) => {
                if beam.pos.y == grid.len() - 1 {
                    continue;
                } else {
                    beam.forward();
                    q.push_back(beam);
                }
            }
            (Tile::Empty, Direction::Left) => {
                if beam.pos.x == 0 {
                    continue;
                } else {
                    beam.forward();
                    q.push_back(beam);
                }
            }
            (Tile::Empty, Direction::Right) => {
                if beam.pos.x == grid[0].len() - 1 {
                    continue;
                } else {
                    beam.forward();
                    q.push_back(beam);
                }
            }
            (Tile::SplitHoriz, Direction::Up) | (Tile::SplitHoriz, Direction::Down) => {
                if beam.pos.x != 0 {
                    beam.dir = Direction::Left;
                    beam.forward();
                    q.push_back(beam);
                }
                if beam.pos.x != grid[0].len() - 1 {
                    beam.dir = Direction::Right;
                    beam.forward();
                    q.push_back(beam);
                }
            }
            (Tile::SplitHoriz, Direction::Left) => {
                if beam.pos.x == 0 {
                    continue;
                } else {
                    beam.forward();
                    q.push_back(beam);
                }
            }
            (Tile::SplitHoriz, Direction::Right) => {
                if beam.pos.x == grid[0].len() - 1 {
                    continue;
                } else {
                    beam.forward();
                    q.push_back(beam);
                }
            }
            (Tile::SplitVert, Direction::Up) => {
                if beam.pos.y == 0 {
                    continue;
                } else {
                    beam.forward();
                    q.push_back(beam);
                }
            }
            (Tile::SplitVert, Direction::Down) => {
                if beam.pos.y == grid.len() - 1 {
                    continue;
                } else {
                    beam.forward();
                    q.push_back(beam);
                }
            }
            (Tile::SplitVert, Direction::Left) | (Tile::SplitVert, Direction::Right) => {
                if beam.pos.y != 0 {
                    beam.dir = Direction::Up;
                    beam.forward();
                    q.push_back(beam);
                }
                if beam.pos.y != grid.len() - 1 {
                    beam.dir = Direction::Down;
                    beam.forward();
                    q.push_back(beam);
                }
            }
            (Tile::MirrorForward, Direction::Up) => {
                if beam.pos.x == grid[0].len() - 1 {
                    continue;
                } else {
                    beam.dir = Direction::Right;
                    beam.forward();
                    q.push_back(beam);
                }
            }
            (Tile::MirrorForward, Direction::Down) => {
                if beam.pos.x == 0 {
                    continue;
                } else {
                    beam.dir = Direction::Left;
                    beam.forward();
                    q.push_back(beam);
                }
            }
            (Tile::MirrorForward, Direction::Left) => {
                if beam.pos.y == grid.len() - 1 {
                    continue;
                } else {
                    beam.dir = Direction::Down;
                    beam.forward();
                    q.push_back(beam);
                }
            }
            (Tile::MirrorForward, Direction::Right) => {
                if beam.pos.y == 0 {
                    continue;
                } else {
                    beam.dir = Direction::Up;
                    beam.forward();
                    q.push_back(beam);
                }
            }
            (Tile::MirrorBack, Direction::Up) => {
                if beam.pos.x == 0 {
                    continue;
                } else {
                    beam.dir = Direction::Left;
                    beam.forward();
                    q.push_back(beam);
                }
            }
            (Tile::MirrorBack, Direction::Down) => {
                if beam.pos.x == grid[0].len() - 1 {
                    continue;
                } else {
                    beam.dir = Direction::Right;
                    beam.forward();
                    q.push_back(beam);
                }
            }
            (Tile::MirrorBack, Direction::Left) => {
                if beam.pos.y == 0 {
                    continue;
                } else {
                    beam.dir = Direction::Up;
                    beam.forward();
                    q.push_back(beam);
                }
            }
            (Tile::MirrorBack, Direction::Right) => {
                if beam.pos.y == grid.len() - 1 {
                    continue;
                } else {
                    beam.dir = Direction::Down;
                    beam.forward();
                    q.push_back(beam);
                }
            }
        }
    }
    energized.len()
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let grid = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '\\' => Ok(Tile::MirrorBack),
                        '/' => Ok(Tile::MirrorForward),
                        '.' => Ok(Tile::Empty),
                        '-' => Ok(Tile::SplitHoriz),
                        '|' => Ok(Tile::SplitVert),
                        _ => Err(AoCError::Parsing),
                    })
                    .collect::<AoCResult<Vec<Tile>>>()
            })
            .collect::<AoCResult<Vec<Vec<Tile>>>>()?;

        Ok(Self(grid))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let start = Beam {
            pos: Coord { x: 0, y: 0 },
            dir: Direction::Right,
        };
        Ok(energized(start, &self.0))
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let from_left = (0..self.0.len()).map(|row| Beam {
            dir: Direction::Right,
            pos: Coord { x: 0, y: row },
        });
        let from_right = (0..self.0.len()).map(|row| Beam {
            dir: Direction::Left,
            pos: Coord {
                x: self.0[0].len() - 1,
                y: row,
            },
        });
        let from_up = (0..self.0[0].len()).map(|col| Beam {
            dir: Direction::Down,
            pos: Coord { x: col, y: 0 },
        });
        let from_down = (0..self.0[0].len()).map(|col| Beam {
            dir: Direction::Up,
            pos: Coord {
                x: col,
                y: self.0.len() - 1,
            },
        });

        from_left
            .chain(from_right)
            .chain(from_up)
            .chain(from_down)
            .map(|start| energized(start, &self.0))
            .max()
            .ok_or(AoCError::Solving)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#;
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "46");
    }

    #[test]
    fn part_2() {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#;
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "51");
    }
}
