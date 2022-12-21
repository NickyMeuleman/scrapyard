use std::collections::HashSet;

use itertools::Itertools;

use crate::AoCData;

enum Dimension {
    X,
    Y,
    Z,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Default, Debug)]
struct Coord {
    x: i16,
    y: i16,
    z: i16,
}

impl Coord {
    fn neighbours(&self) -> impl Iterator<Item = Coord> + '_ {
        [Dimension::X, Dimension::Y, Dimension::Z]
            .iter()
            .cartesian_product([-1, 1])
            .map(|(dimension, offset)| match dimension {
                Dimension::X => Coord {
                    x: self.x + offset,
                    ..*self
                },
                Dimension::Y => Coord {
                    y: self.y + offset,
                    ..*self
                },
                Dimension::Z => Coord {
                    z: self.z + offset,
                    ..*self
                },
            })
    }
    // fn neighbours(&self) -> Vec<Coord> {
    //     let mut neighbours = Vec::new();

    //     // loop over every dimension in a cube
    //     for dimension in [Dimension::X, Dimension::Y, Dimension::Z] {
    //         // add or remove 1 to coordinate in current dimension
    //         for offset in [-1, 1] {
    //             // resulting coordinates are from the coord to a side of a cube
    //             let mut neighbour = self.clone();
    //             match dimension {
    //                 Dimension::X => neighbour.x += offset,
    //                 Dimension::Y => neighbour.y += offset,
    //                 Dimension::Z => neighbour.z += offset,
    //             }
    //             neighbours.push(neighbour);
    //         }
    //     }

    //     neighbours
    // }

    fn in_bounds(&self, bounds: &[Self; 2]) -> bool {
        let [mins, maxs] = bounds;
        self.x >= mins.x - 1
            && self.x <= maxs.x + 1
            && self.y >= mins.y - 1
            && self.y <= maxs.y + 1
            && self.z >= mins.z - 1
            && self.z <= maxs.z + 1
    }
}

fn bounds(cubes: &HashSet<Coord>) -> [Coord; 2] {
    cubes.iter().fold(
        [Coord::default(), Coord::default()],
        |[mut mins, mut maxs], cube| {
            mins.x = mins.x.min(cube.x);
            mins.y = mins.y.min(cube.y);
            mins.z = mins.z.min(cube.z);
            maxs.x = maxs.x.max(cube.x);
            maxs.y = maxs.y.max(cube.y);
            maxs.z = maxs.z.max(cube.z);
            [mins, maxs]
        },
    )
}

fn exposed(cubes: &HashSet<Coord>) -> HashSet<Coord> {
    let bounds = bounds(cubes);
    let mut exposed = HashSet::new();

    let start = Coord::default();
    let mut q = Vec::new();
    let mut seen = HashSet::new();

    q.push(start);
    seen.insert(start);

    while let Some(coord) = q.pop() {
        for neighbour in coord.neighbours() {
            if cubes.contains(&neighbour) || !neighbour.in_bounds(&bounds) {
                continue;
            }
            if seen.insert(neighbour) {
                q.push(neighbour);
                exposed.insert(neighbour);
            }
        }
    }

    exposed
}

pub struct Data(HashSet<Coord>);

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> Option<Self> {
        let cubes = input
            .lines()
            .map(|line| {
                let mut nums = line.split(',').map(|s| s.parse().unwrap());
                Coord {
                    x: nums.next().unwrap(),
                    y: nums.next().unwrap(),
                    z: nums.next().unwrap(),
                }
            })
            .collect();

        Some(Self(cubes))
    }

    fn part_1(&self) -> String {
        self.0
            .iter()
            .flat_map(|coord| coord.neighbours())
            // only keep neighbours that are not a cube
            .filter(|coord| !self.0.contains(coord))
            .count()
            .to_string()
    }

    fn part_2(&self) -> String {
        let exposed = exposed(&self.0);
        self.0
            .iter()
            .flat_map(|coord| coord.neighbours())
            // only keep neighbours that are also exposed
            .filter(|coord| exposed.contains(coord))
            .count()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(18);
        let data = Data::try_new(&input).unwrap();
        assert_eq!(data.part_1(), "64");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(18);
        let data = Data::try_new(&input).unwrap();
        assert_eq!(data.part_2(), "58");
    }
}
