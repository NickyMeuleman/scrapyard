use std::{
    cmp::{Ordering, Reverse},
    collections::{BTreeMap, BinaryHeap, HashMap, HashSet},
    fmt::Display,
};

use itertools::Itertools;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<Vec<Track>>, BTreeMap<Coord, (i32, i32, i32)>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Coord {
    row: i32,
    col: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Track {
    Track(TrackDir),
    Curve(Curve),
    Intersection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TrackDir {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Curve {
    BackSlash,
    ForwardSlash,
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut carts = BTreeMap::new();

        let map = input
            .lines()
            .enumerate()
            .map(|(row, line)| {
                let row = row as i32;
                line.chars()
                    .enumerate()
                    .map(|(col, chr)| {
                        let col = col as i32;
                        match chr {
                            '+' => Track::Intersection,
                            '\\' => Track::Curve(Curve::BackSlash),
                            '/' => Track::Curve(Curve::ForwardSlash),
                            'v' => {
                                carts.insert(Coord { row, col }, (1, 0, 0));
                                Track::Track(TrackDir::Vertical)
                            }
                            '^' => {
                                carts.insert(Coord { row, col }, (-1, 0, 0));
                                Track::Track(TrackDir::Vertical)
                            }
                            '<' => {
                                carts.insert(Coord { row, col }, (0, -1, 0));
                                Track::Track(TrackDir::Horizontal)
                            }
                            '>' => {
                                carts.insert(Coord { row, col }, (0, 1, 0));
                                Track::Track(TrackDir::Horizontal)
                            }
                            _ => Track::Track(TrackDir::Horizontal),
                        }
                    })
                    .collect()
            })
            .collect();

        Ok(Self(map, carts))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        Ok("Not implemented yet")
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let map = &self.0;
        let mut carts = self.1.clone();

        while carts.len() > 1 {
            let mut new_carts = BTreeMap::new();

            while let Some(&Coord { row, col }) = carts.keys().next() {
                let (d_row, d_col, mut intersect_dir) = carts
                    .remove(&Coord { row, col })
                    .unwrap();

                let (new_col, new_row) = (col + d_col, row + d_row);
                let new_coord = Coord {
                    row: new_row,
                    col: new_col,
                };

                if carts.contains_key(&new_coord) {
                    carts.remove(&new_coord);
                } else if new_carts.contains_key(&new_coord) {
                    new_carts.remove(&new_coord);
                } else {
                    let (d_col, d_row) = match map[new_row as usize][new_col as usize] {
                        Track::Track(_) => (d_col, d_row),
                        Track::Curve(direction) => {
                            let direction = match direction {
                                Curve::BackSlash => 1,
                                Curve::ForwardSlash => -1,
                            };
                            (d_row * direction, d_col * direction)
                        }
                        Track::Intersection => {
                            let result = match intersect_dir {
                                0 => (d_row, -d_col),
                                1 => (d_col, d_row),
                                2 => (-d_row, d_col),
                                _ => panic!(""),
                            };
                            intersect_dir = (intersect_dir + 1) % 3;
                            result
                        }
                    };

                    new_carts.insert(
                        Coord {
                            row: new_row,
                            col: new_col,
                        },
                        (d_row, d_col, intersect_dir),
                    );
                }
            }

            carts = new_carts;
        }

        let Coord { row, col } = carts.keys().next().unwrap();

        Ok(format!("{},{}", col, row))
    }
}

// What I used to get the star for part 1:
// use std::{
//     cmp::Ordering,
//     collections::{BTreeMap, BinaryHeap, HashMap, HashSet},
//     fmt::Display,
// };

// use itertools::Itertools;

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
// struct Coord {
//     row: i32,
//     col: i32,
// }
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// enum Track {
//     Track(TrackDir),
//     Curve(Curve),
//     Intersection,
// }
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// enum TrackDir {
//     Horizontal,
//     Vertical,
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// enum Curve {
//     BackSlash,
//     ForwardSlash,
// }
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// enum Direction {
//     Up,
//     Right,
//     Down,
//     Left,
// }

// impl Direction {
//     fn turn_left(&self) -> Self {
//         match self {
//             Direction::Up => Direction::Left,
//             Direction::Right => Direction::Up,
//             Direction::Down => Direction::Right,
//             Direction::Left => Direction::Down,
//         }
//     }
//     fn turn_right(&self) -> Self {
//         match self {
//             Direction::Up => Direction::Right,
//             Direction::Right => Direction::Down,
//             Direction::Down => Direction::Left,
//             Direction::Left => Direction::Up,
//         }
//     }
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// struct Cart {
//     id: usize,
//     coord: Coord,
//     intersection_count: usize,
//     facing: Direction,
// }

// impl Cart {
//     fn move_one(&self) -> Coord {
//         match self.facing {
//             Direction::Up => Coord {
//                 row: self.coord.row - 1,
//                 col: self.coord.col,
//             },
//             Direction::Right => Coord {
//                 row: self.coord.row,
//                 col: self.coord.col + 1,
//             },
//             Direction::Down => Coord {
//                 row: self.coord.row + 1,
//                 col: self.coord.col,
//             },
//             Direction::Left => Coord {
//                 row: self.coord.row,
//                 col: self.coord.col - 1,
//             },
//         }
//     }
// }

// impl Ord for Cart {
//     fn cmp(&self, other: &Self) -> Ordering {
//         other
//             .coord
//             .row
//             .cmp(&self.coord.row)
//             .then_with(|| other.coord.col.cmp(&self.coord.col))
//     }
// }

// impl PartialOrd for Cart {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

// struct Cave {
//     crashed: HashSet<usize>,
//     map: HashMap<Coord, Track>,
//     prev: BinaryHeap<Cart>,
//     carts: BinaryHeap<Cart>,
// }

// impl Display for Cave {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let (min_row, max_row) = self
//             .map
//             .keys()
//             .map(|coord| coord.row)
//             .minmax()
//             .into_option()
//             .unwrap();
//         let (min_col, max_col) = self
//             .map
//             .keys()
//             .map(|coord| coord.col)
//             .minmax()
//             .into_option()
//             .unwrap();
//         let mut result = Vec::new();
//         for row in min_row..=max_row {
//             let mut row_result = Vec::new();
//             for col in min_col..=max_col {
//                 let coord = Coord { row, col };
//                 if let Some(cart) = self.carts.iter().find(|cart| cart.coord == coord) {
//                     if !self.crashed.contains(&cart.id) {
//                         let dir = match cart.facing {
//                             Direction::Up => '^',
//                             Direction::Right => '>',
//                             Direction::Down => 'v',
//                             Direction::Left => '<',
//                         };
//                         row_result.push(dir);
//                         continue;
//                     }
//                 }
//                 if let Some(track) = self.map.get(&coord) {
//                     let ch = match track {
//                         Track::Track(TrackDir::Horizontal) => '-',
//                         Track::Track(TrackDir::Vertical) => '|',
//                         Track::Curve(kind) => match kind {
//                             Curve::BackSlash => '\\',
//                             Curve::ForwardSlash => '/',
//                         },
//                         Track::Intersection => '+',
//                     };
//                     row_result.push(ch);
//                 } else {
//                     row_result.push(' ');
//                 };
//             }
//             result.push(row_result);
//         }

//         let result = result
//             .iter()
//             .map(|v| v.iter().collect::<String>())
//             .join("\n");

//         write!(f, "{}", result)
//     }
// }

// fn parse(input: &str) -> (Vec<Vec<Track>>, BTreeMap<Coord, (i32, i32, i32)>) {
//     let mut carts = BTreeMap::new();

//     let map = input
//         .lines()
//         .enumerate()
//         .map(|(row, line)| {
//             let row = row as i32;
//             line.chars()
//                 .enumerate()
//                 .map(|(col, chr)| {
//                     let col = col as i32;
//                     match chr {
//                         '+' => Track::Intersection,
//                         '\\' => Track::Curve(Curve::BackSlash),
//                         '/' => Track::Curve(Curve::ForwardSlash),
//                         'v' => {
//                             carts.insert(Coord { row, col }, (1, 0, 0));
//                             Track::Track(TrackDir::Vertical)
//                         }
//                         '^' => {
//                             carts.insert(Coord { row, col }, (-1, 0, 0));
//                             Track::Track(TrackDir::Vertical)
//                         }
//                         '<' => {
//                             carts.insert(Coord { row, col }, (0, -1, 0));
//                             Track::Track(TrackDir::Horizontal)
//                         }
//                         '>' => {
//                             carts.insert(Coord { row, col }, (0, 1, 0));
//                             Track::Track(TrackDir::Horizontal)
//                         }
//                         _ => Track::Track(TrackDir::Horizontal),
//                     }
//                 })
//                 .collect()
//         })
//         .collect();

//     (map, carts)
// }

// pub fn part_1(input: &str) -> String {
//     let mut cave = parse(input);
//     for i in 0.. {
//         // println!("{}", cave);
//         std::mem::swap(&mut cave.carts, &mut cave.prev);
//         cave.carts.clear();
//         for cart in cave.prev.iter() {
//             let new_coord = cart.move_one();
//             if cave
//                 .carts
//                 .iter()
//                 .map(|cart| cart.coord)
//                 .contains(&new_coord)
//                 || cave.prev.iter().map(|cart| cart.coord).contains(&new_coord)
//             {
//                 return format!("{},{}", new_coord.col, new_coord.row);
//             }
//             let (new_face, new_count) = match cave.map.get(&new_coord).unwrap() {
//                 Track::Track(_) => (cart.facing, cart.intersection_count),
//                 Track::Curve(kind) => {
//                     let face = match (cart.facing, kind) {
//                         (Direction::Up, Curve::BackSlash) => Direction::Left,
//                         (Direction::Up, Curve::ForwardSlash) => Direction::Right,
//                         (Direction::Right, Curve::BackSlash) => Direction::Down,
//                         (Direction::Right, Curve::ForwardSlash) => Direction::Up,
//                         (Direction::Down, Curve::BackSlash) => Direction::Right,
//                         (Direction::Down, Curve::ForwardSlash) => Direction::Left,
//                         (Direction::Left, Curve::BackSlash) => Direction::Up,
//                         (Direction::Left, Curve::ForwardSlash) => Direction::Down,
//                     };
//                     (face, cart.intersection_count)
//                 }
//                 // Each time a cart has the option to turn,
//                 //  it turns left the first time,
//                 // goes straight the second time,
//                 // turns right the third time,
//                 // and then repeats those directions starting again with left the fourth time, straight the fifth time, and so on.
//                 Track::Intersection => {
//                     let new_face = match cart.intersection_count % 3 {
//                         0 => cart.facing.turn_left(),
//                         1 => cart.facing,
//                         2 => cart.facing.turn_right(),
//                         _ => panic!("what"),
//                     };
//                     (new_face, cart.intersection_count + 1)
//                 }
//             };
//             cave.carts.push(Cart {
//                 id: cart.id,
//                 coord: new_coord,
//                 facing: new_face,
//                 intersection_count: new_count,
//             });
//         }
//     }
//     todo!()
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "");
    }

    #[test]
    fn part_2() {
        let input = "";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "");
    }
}
