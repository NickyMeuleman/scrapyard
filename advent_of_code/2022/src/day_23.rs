use std::collections::{HashSet, HashMap};

use itertools::Itertools;

use crate::AoCData;

pub struct Data(HashSet<Coord>);

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    row: i32,
    col: i32,
}

enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    fn check(&self, neighbours: &[bool; 8]) -> bool {
        // neighbours in form [n, ne, e, se, s, sw, w, nw]
        let [n, ne, e, se, s, sw, w, nw] = neighbours;
        // in these 4 conditions the question says OR, but it means AND, right?
        match &self {
            // If there is no Elf in the N, NE, or NW adjacent positions, the Elf proposes moving north one step.
            Direction::North => !n && !ne && !nw,
            // If there is no Elf in the S, SE, or SW adjacent positions, the Elf proposes moving south one step.
            Direction::South => !s && !se && !sw,
            // If there is no Elf in the W, NW, or SW adjacent positions, the Elf proposes moving west one step.
            Direction::West => !w && !nw && !sw,
            // If there is no Elf in the E, NE, or SE adjacent positions, the Elf proposes moving east one step.
            Direction::East => !e && !ne && !se,
            // question doesn't specify, but I assume the elf doesn't propose anything
            _ => false,
        }
    }
}

impl Coord {
    fn neighbours(&self) -> [Self; 8] {
        use Direction::*;
        let n = self.add_dir(&North);
        let ne = self.add_dir(&NorthEast);
        let e = self.add_dir(&East);
        let se = self.add_dir(&SouthEast);
        let s = self.add_dir(&South);
        let sw = self.add_dir(&SouthWest);
        let w = self.add_dir(&West);
        let nw = self.add_dir(&NorthWest);
        [n, ne, e, se, s, sw, w, nw]
    }

    fn add_dir(&self, dir: &Direction) -> Self {
        use Direction::*;
        match dir {
            North => Coord {
                row: self.row - 1,
                col: self.col,
            },
            NorthEast => Coord {
                row: self.row - 1,
                col: self.col + 1,
            },
            East => Coord {
                row: self.row,
                col: self.col + 1,
            },
            SouthEast => Coord {
                row: self.row + 1,
                col: self.col + 1,
            },
            South => Coord {
                row: self.row + 1,
                col: self.col,
            },
            SouthWest => Coord {
                row: self.row + 1,
                col: self.col - 1,
            },
            West => Coord {
                row: self.row,
                col: self.col - 1,
            },
            NorthWest => Coord {
                row: self.row - 1,
                col: self.col - 1,
            },
        }
    }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> Option<Self> {
        let mut elves = HashSet::new();
        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c == '#' {
                    elves.insert(Coord {
                        row: row as i32,
                        col: col as i32,
                    });
                }
            }
        }
        Some(Self(elves))
    }

    fn part_1(&self) -> String {
        let mut elves = self.0.clone();
        let mut checks = [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ];

        for round in 1.. {
            // key: proposed coordinate, val: list of old coordinates that proposed going there
            let mut proposals: HashMap<Coord, Vec<Coord>> = HashMap::new();

            // consideration phase
            for elf in &elves {
                let neighbours = elf.neighbours();
                // If no other Elves are in one of those eight positions, the Elf does not do anything during this round.
                if neighbours.iter().all(|coord| !elves.contains(coord)) {
                    continue;
                }
                let neighbours = neighbours
                    .iter()
                    .map(|neighbour| elves.contains(neighbour))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();

                let proposed_dir = checks.iter().find(|dir| dir.check(&neighbours));
                if let Some(dir) = proposed_dir {
                    let proposal = elf.add_dir(dir);
                    proposals.entry(proposal).or_default().push(*elf);
                }
            }

            // movement phase
            for (new_coord, old_coords) in proposals {
                if old_coords.len() == 1 {
                    elves.remove(&old_coords[0]);
                    elves.insert(new_coord);
                }
            }

            // after round
            // println!("== End of Round {} ==", round);
            // print(&elves);
            // println!();

            checks.rotate_left(1);
            if round == 10 {
                let (minmax_row, minmax_col) = elves.iter().fold(
                    ((i32::MAX, i32::MIN), (i32::MAX, i32::MIN)),
                    |(minmax_row, minmax_col), Coord { row, col }| {
                        (
                            (minmax_row.0.min(*row), minmax_row.1.max(*row)),
                            (minmax_col.0.min(*col), minmax_col.1.max(*col)),
                        )
                    },
                );
                return (minmax_row.0..=minmax_row.1)
                    .cartesian_product(minmax_col.0..=minmax_col.1)
                    .filter(|(row, col)| {
                        !elves.contains(&Coord {
                            row: *row,
                            col: *col,
                        })
                    })
                    .count()
                    .to_string();
            }
        }

        String::from("")
    }

    fn part_2(&self) -> String {
        let mut elves = self.0.clone();
        let mut checks = [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ];
    
        for round in 1.. {
            let mut moved = false;
            // key: proposed coordinate, val: list of old coordinates that proposed going there
            let mut proposals: HashMap<Coord, Vec<Coord>> = HashMap::new();
    
            // consideration phase
            for elf in &elves {
                let neighbours = elf.neighbours();
                // If no other Elves are in one of those eight positions, the Elf does not do anything during this round.
                if neighbours.iter().all(|coord| !elves.contains(coord)) {
                    continue;
                }
                let neighbours = neighbours
                    .iter()
                    .map(|neighbour| elves.contains(neighbour))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
    
                let proposed_dir = checks.iter().find(|dir| dir.check(&neighbours));
                if let Some(dir) = proposed_dir {
                    let proposal = elf.add_dir(dir);
                    proposals.entry(proposal).or_default().push(*elf);
                }
            }
    
            // movement phase
            for (new_coord, old_coords) in proposals {
                if old_coords.len() == 1 {
                    moved = true;
                    elves.remove(&old_coords[0]);
                    elves.insert(new_coord);
                }
            }
    
            // after round
            if !moved {
                return round.to_string();
            }
            checks.rotate_left(1);
        }
    
        String::from("")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(23);
        let data = Data::try_new(&input).unwrap();
        assert_eq!(data.part_1(), "110");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(23);
        let data = Data::try_new(&input).unwrap();
        assert_eq!(data.part_2(), "20");
    }
}
