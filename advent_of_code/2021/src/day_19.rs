use crate::utils::{AoCData, Solution};
use hashbrown::HashSet;

pub struct Data {
    scans: Vec<Scan>,
}

#[derive(Debug, Clone, Default)]
struct Scan {
    grid: HashSet<Point>,
}

impl Scan {
    fn rotations(&self) -> impl Iterator<Item = Self> + '_ {
        // create an iterator of rotated scans
        (0..24).map(|idx| {
            // create a scan where every point is rotated the same way
            Scan {
                grid: self.grid.iter().map(|point| point.rotate(idx)).collect(),
            }
        })
    }

    fn try_merge(&mut self, other: &Scan) -> Option<Point> {
        // try to merge the other scan into self by trying all possible rotations of other
        for rotation in other.rotations() {
            // iterate over every possible combination of a point in self and a point in a rotation of other.
            // subtract the points in the rotation from the points in self (the full scan)
            // distances is an iterator of the math-vectors to go from p2 to p1 (distance = p1 - p2, so p1 = p2 + distance)
            // in a later step we add that distance to p2 for every point in that rotated scan, getting p1. At least, if this was the correct rotation.
            // we verify this by checking for sufficient overlapping points
            let distances = self
                .grid
                .iter()
                .flat_map(|p1| rotation.grid.iter().map(move |p2| (p1, p2)))
                .map(|(p1, p2)| Point::new(p1.x - p2.x, p1.y - p2.y, p1.z - p2.z));

            for distance in distances {
                // destructure values
                let Point {
                    x: dx,
                    y: dy,
                    z: dz,
                } = distance;

                // for every point in the rotation, add the distance vector to it
                // this moves all points according to that vector
                // if this results in >= 12 beacon overlaps with the full map, merge this rotation into the full map.
                // this also means the distance vector was the location of the scanner for this scan,
                // relative to the full map (which starts at 0,0,0)
                let translated = rotation
                    .grid
                    .iter()
                    .map(|Point { x, y, z }| Point::new(x + dx, y + dy, z + dz));

                let overlapping = translated
                    .clone()
                    .filter(|point| self.grid.contains(point))
                    .count();

                if overlapping >= 12 {
                    // this translation has significant overlap with self
                    // merge it into self, and return the scanner point
                    self.grid.extend(translated);
                    // the scanner point is the current distance because it is relative to the origin of the full scan (0,0,0)
                    return Some(distance);
                }
            }
        }

        // all possible rotations of other could not be merged with self
        None
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i16,
    y: i16,
    z: i16,
}

impl Point {
    fn new(x: i16, y: i16, z: i16) -> Self {
        Self { x, y, z }
    }

    fn rotate(&self, idx: u8) -> Self {
        // destructure values
        let Point { x, y, z } = *self;
        // all 24 possible rotations
        match idx {
            0 => Point::new(x, y, z),
            1 => Point::new(x, z, -y),
            2 => Point::new(x, -y, -z),
            3 => Point::new(x, -z, y),
            4 => Point::new(y, x, -z),
            5 => Point::new(y, z, x),
            6 => Point::new(y, -x, z),
            7 => Point::new(y, -z, -x),
            8 => Point::new(z, x, y),
            9 => Point::new(z, y, -x),
            10 => Point::new(z, -x, -y),
            11 => Point::new(z, -y, x),
            12 => Point::new(-x, y, -z),
            13 => Point::new(-x, z, y),
            14 => Point::new(-x, -y, z),
            15 => Point::new(-x, -z, -y),
            16 => Point::new(-y, x, z),
            17 => Point::new(-y, z, -x),
            18 => Point::new(-y, -x, -z),
            19 => Point::new(-y, -z, x),
            20 => Point::new(-z, x, -y),
            21 => Point::new(-z, y, x),
            22 => Point::new(-z, -x, y),
            23 => Point::new(-z, -y, -x),
            _ => unreachable!("Tried to create an invalid rotation"),
        }
    }

    fn manhattan(&self, other: &Self) -> i16 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

impl Data {
    fn build_full_scan(&self) -> (Scan, Vec<Point>) {
        let mut todo = self.scans.clone();
        // start the full scan off as the first scan
        let mut full_scan = todo.remove(0);
        // start with the distance to the starting scanner
        let mut distances = vec![Point::new(0, 0, 0)];
        // iterate over the indexes of the todo vecor
        // in reverse order, because we call swap_remove which replaces a todo-scan with the last one in the todo vector
        // because we're looping reversed, that doesn't cause issues, the removed one is always replaced by a todo-scan that has already been checked this loop
        while !todo.is_empty() {
            for idx in (0..todo.len()).rev() {
                if let Some(distance) = full_scan.try_merge(&todo[idx]) {
                    distances.push(distance);
                    todo.swap_remove(idx);
                }
            }
        }

        (full_scan, distances)
    }
}

impl AoCData for Data {
    fn new(input: String) -> Self {
        Self {
            scans: input
                .trim()
                .split("\n\n")
                .map(|block| {
                    Scan {
                        grid: block
                            .lines()
                            // skip the first line telling you what scanner this is
                            // that information is encoded in the index of the vector this block collects into
                            .skip(1)
                            .filter_map(|line| {
                                let (x, rest) = line.split_once(',')?;
                                let (y, z) = rest.split_once(',')?;
                                Some(Point {
                                    x: x.parse().ok()?,
                                    y: y.parse().ok()?,
                                    z: z.parse().ok()?,
                                })
                            })
                            .collect(),
                    }
                })
                .collect(),
        }
    }

    fn part_1(&self) -> String {
        let (full_scan, _) = self.build_full_scan();
        full_scan.grid.len().to_string()
    }

    fn part_2(&self) -> String {
        let (_, distances) = self.build_full_scan();

        // iterate over all possible combinations of 2 scanners
        distances
            .iter()
            .enumerate()
            .flat_map(|(i, scanner1)| {
                distances
                    .iter()
                    .enumerate()
                    // exclude combinations where the 2 scanners are the same one
                    .filter(move |(j, _)| i != *j)
                    .map(move |(_, scanner2)| (scanner1, scanner2))
            })
            .map(|(scanner1, scanner2)| scanner1.manhattan(scanner2))
            // writing a fold because .max() gives a rust analyzer error
            .fold(0, |acc, num| acc.max(num))
            .to_string()
    }

    fn solve(self) -> Solution {
        // code that combines p1 and p2 answers to avoid duplicate work
        let (full_scan, distances) = self.build_full_scan();

        let num_beacons = full_scan.grid.len();
        // iterate over all possible combinations
        let greatest_manhattan = distances
            .iter()
            .enumerate()
            .flat_map(|(i, scanner1)| {
                distances
                    .iter()
                    .enumerate()
                    // exclude combinations where the 2 scanners are the same one
                    .filter(move |(j, _)| i != *j)
                    .map(move |(_, scanner2)| (scanner1, scanner2))
            })
            .map(|(scanner1, scanner2)| scanner1.manhattan(scanner2))
            // writing a fold because .max() gives a rust analyzer error
            .fold(0, |acc, num| acc.max(num));

        Solution {
            part1: num_beacons.to_string(),
            part2: greatest_manhattan.to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(19);
        let data = Data::new(input);
        assert_eq!(data.part_1(), "79");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(19);
        let data = Data::new(input);
        assert_eq!(data.part_2(), "3621");
    }
}
