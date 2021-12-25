use hashbrown::HashMap;
use std::convert::TryInto;
use std::fmt;
use std::{convert::Infallible, str::FromStr};

#[derive(Debug, Clone)]
pub struct Data {
    rows: u8,
    cols: u8,
    map: HashMap<Point, Cucumber>,
}

impl Data {
    fn turn(&mut self, direction: &Cucumber) -> bool {
        let can_move: Vec<_> = self
            .map
            .iter()
            .filter_map(|(point, cucumber)| {
                // filter out every other type of cucumber
                if cucumber != direction {
                    return None;
                }

                // where does this cucumber want to move to?
                let new_point = match direction {
                    Cucumber::Right => Point {
                        row: point.row,
                        col: (point.col + 1) % self.cols,
                    },
                    Cucumber::Down => Point {
                        row: (point.row + 1) % self.rows,
                        col: point.col,
                    },
                };

                // can this cucumber move?
                // if it can, return a tuple (from, to, cucumber)
                if !self.map.contains_key(&new_point) {
                    Some((*point, new_point, *cucumber))
                } else {
                    None
                }
            })
            .collect();

        for (from, to, cucumber) in &can_move {
            self.map.remove(from);
            self.map.insert(*to, *cucumber);
        }

        !can_move.is_empty()
    }

    pub fn part_one(&mut self) -> usize {
        let mut steps = 0;

        loop {
            let moved_right = self.turn(&Cucumber::Right);
            let moved_down = self.turn(&Cucumber::Down);
            steps += 1;
            if !moved_right && !moved_down {
                break;
            }
        }

        steps
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        for row_idx in 0..self.rows {
            let mut line = String::new();

            for col_idx in 0..self.cols {
                let cucumber = self.map.get(&Point {
                    row: row_idx,
                    col: col_idx,
                });
                let c = match cucumber {
                    None => '.',
                    Some(Cucumber::Down) => 'v',
                    Some(Cucumber::Right) => '>',
                };
                line.push(c);
            }

            result.push_str(&line);
            result.push_str("\n");
        }

        write!(f, "{}", result)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cucumber {
    Down,
    Right,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    row: u8,
    col: u8,
}

impl FromStr for Data {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let map = input
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars().enumerate().filter_map(move |(col, c)| {
                    let cucumber = match c {
                        '>' => Some(Cucumber::Right),
                        'v' => Some(Cucumber::Down),
                        _ => None,
                    };

                    if let Some(cucumber) = cucumber {
                        let point = Point {
                            row: row.try_into().unwrap(),
                            col: col.try_into().unwrap(),
                        };
                        Some((point, cucumber))
                    } else {
                        None
                    }
                })
            })
            .collect();

        Ok(Self {
            map,
            rows: input.lines().count().try_into().unwrap(),
            cols: input
                .lines()
                .next()
                .unwrap()
                .chars()
                .count()
                .try_into()
                .unwrap(),
        })
    }
}

// solution without mutation:
// impl Data {
// fn turn(&self, direction: &Cucumber) -> Option<HashMap<Point, Cucumber>> {
//     let mut changed = false;
//     let mut map: HashMap<Point, Cucumber> = HashMap::new();

//     for (point, cucumber) in &self.map {
//         match (point, cucumber) {
//             (point, cucumber) if cucumber == direction => {
//                 let new_point = match direction {
//                     Cucumber::Right => Point {
//                         row: point.row,
//                         col: (point.col + 1) % self.cols,
//                     },
//                     Cucumber::Down => Point {
//                         row: (point.row + 1) % self.rows,
//                         col: point.col,
//                     },
//                 };
//                 if !self.map.contains_key(&new_point) {
//                     // move
//                     changed = true;
//                     map.insert(new_point, *cucumber);
//                 } else {
//                     // stay
//                     map.insert(*point, *cucumber);
//                 }
//             }
//             (point, cucumber) => {
//                 // stay
//                 map.insert(*point, *cucumber);
//             }
//         }
//     }

//     if changed {
//         Some(map)
//     } else {
//         None
//     }
// }

// pub fn part_one(&mut self) -> usize {
//     let mut steps = 0;

//     loop {
//         let mut moved_right = false;
//         let mut moved_down = false;
//         if let Some(map) = self.turn(&Cucumber::Right) {
//             self.map = map;
//             moved_right = true;
//         }
//         if let Some(map) = self.turn(&Cucumber::Down) {
//             self.map = map;
//             moved_down = true;
//         }
//         steps += 1;
//         if !moved_right && !moved_down {
//             break;
//         }
//     }

//     steps
// }
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

        let mut data: Data = input.parse().unwrap();
        assert_eq!(data.part_one(), 58);
    }
}
