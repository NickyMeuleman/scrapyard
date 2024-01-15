use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    ops::RangeInclusive,
    str::FromStr,
};

use aoc_core::{AoCError, Solution};
use itertools::Itertools;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<Brick>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: u16,
    y: u16,
    z: u16,
}

impl FromStr for Coord {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y, z) = s
            .split(",")
            .collect_tuple()
            .ok_or(AoCError::Parsing)?;
        Ok(Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
            z: z.parse().unwrap(),
        })
    }
}

#[derive(Debug, Clone)]
struct Brick {
    x: RangeInclusive<u16>,
    y: RangeInclusive<u16>,
    z: RangeInclusive<u16>,
}

impl FromStr for Brick {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (lhs, rhs) = s
            .split_once("~")
            .ok_or(AoCError::Parsing)?;
        let lhs: Coord = lhs.parse()?;
        let rhs: Coord = rhs.parse()?;
        Ok(Self {
            x: lhs.x..=rhs.x,
            y: lhs.y..=rhs.y,
            z: lhs.z..=rhs.z,
        })
    }
}

impl Brick {
    fn can_fall(&self, occupied: &HashMap<Coord, u16>) -> bool {
        // ground is at z == 0
        let above_ground = *self.z.start() > 1;
        let blocked = self
            .x
            .clone()
            .cartesian_product(self.y.clone())
            .any(|(x, y)| {
                let pos = Coord {
                    x,
                    y,
                    z: *self.z.start() - 1,
                };
                occupied.contains_key(&pos)
            });

        above_ground && !blocked
    }

    fn fall(&mut self, occupied: &HashMap<Coord, u16>) {
        while self.can_fall(occupied) {
            self.z = *self.z.start() - 1..=*self.z.end() - 1;
        }
    }
}

fn disintegrate(
    id: u16,
    above_map: &HashMap<u16, HashSet<u16>>,
    below_map: &HashMap<u16, HashSet<u16>>,
    disintegrated: &mut HashSet<u16>,
) {
    disintegrated.insert(id);

    if let Some(above) = above_map.get(&id) {
        for above_idx in above {
            if let Some(below) = below_map.get(above_idx) {
                if below
                    .iter()
                    .all(|idx| disintegrated.contains(idx))
                {
                    disintegrate(*above_idx, above_map, below_map, disintegrated)
                }
            }
        }
    }
}

fn disintegrates_safely(
    id: u16,
    above_map: &HashMap<u16, HashSet<u16>>,
    below_map: &HashMap<u16, HashSet<u16>>,
) -> bool {
    if let Some(above) = above_map.get(&id) {
        // check if the removed block was the only one supporting this upper block
        above
            .iter()
            .all(|idx| below_map.get(idx).unwrap().len() > 1)
    } else {
        // no blocks above, this block is safe to remove
        true
    }
}

fn prepare(bricks: &mut Vec<Brick>) -> (HashMap<u16, HashSet<u16>>, HashMap<u16, HashSet<u16>>) {
    // sort bricks by lowest height
    bricks.sort_by_key(|brick| *brick.z.start());

    // coord - block_idx
    let mut occupied = HashMap::new();
    let mut above_map: HashMap<u16, HashSet<u16>> = HashMap::new();
    let mut below_map: HashMap<u16, HashSet<u16>> = HashMap::new();

    for (idx, brick) in bricks.iter_mut().enumerate() {
        brick.fall(&occupied);

        for (x, y) in (brick.x.clone()).cartesian_product(brick.y.clone()) {
            for z in brick.z.clone() {
                let coord = Coord { x, y, z };
                occupied.insert(coord, idx as u16);
            }
            // check if the coord of a brick below this one (that's why we use the min z, to handle vertical blocks) is occupied
            let below = Coord {
                x,
                y,
                z: brick.z.start() - 1,
            };
            if let Some(&below_idx) = occupied.get(&below) {
                above_map
                    .entry(below_idx)
                    .or_default()
                    .insert(idx as u16);
                below_map
                    .entry(idx as u16)
                    .or_default()
                    .insert(below_idx);
            }
        }
    }

    (above_map, below_map)
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let bricks = input
            .lines()
            .map(FromStr::from_str)
            .collect::<AoCResult<Vec<Brick>>>()?;
        Ok(Self(bricks))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut bricks = self.0.clone();
        let (above_map, below_map) = prepare(&mut bricks);

        let res = (0..bricks.len())
            .filter(|&id| disintegrates_safely(id as u16, &above_map, &below_map))
            .count();

        Ok(res)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut bricks = self.0.clone();
        let (above_map, below_map) = prepare(&mut bricks);

        let res: usize = (0..bricks.len())
            .map(|id| {
                let mut disintegrated = HashSet::new();
                disintegrate(id as u16, &above_map, &below_map, &mut disintegrated);
                disintegrated.len() - 1
            })
            .sum();

        Ok(res)
    }

    fn solve(self) -> AoCResult<aoc_core::Solution>
    where
        Self: Sized,
    {
        let mut bricks = self.0;
        let (above_map, below_map) = prepare(&mut bricks);

        let p1 = (0..bricks.len())
            .filter(|&id| disintegrates_safely(id as u16, &above_map, &below_map))
            .count();

        let p2: usize = (0..bricks.len())
            .map(|id| {
                let mut disintegrated = HashSet::new();
                disintegrate(id as u16, &above_map, &below_map, &mut disintegrated);
                disintegrated.len() - 1
            })
            .sum();

        Ok(Solution {
            part1: Box::new(p1),
            part2: Box::new(p2),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "5");
    }

    #[test]
    fn part_2() {
        let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "7");
    }
}
