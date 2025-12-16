// Blog writeup with simpler Rust code (I should handle errors here):
// https://nickymeuleman.netlify.app/blog/aoc2025-day12/

use crate::{AoCData, AoCResult};
use aoc_core::AoCError;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Data {
    shapes: Vec<Vec<Vec<bool>>>,
    regions: Vec<Region>,
}

// assume all presents are 3x3
const BLOCK_SIZE: u32 = 3;

#[derive(Debug, Clone)]
struct Region {
    width: u32,
    length: u32,
    counts: Vec<u32>,
}

impl Region {
    fn definitely_fits(&self) -> bool {
        let blocks_free = (self.width / BLOCK_SIZE) * (self.length / BLOCK_SIZE);
        let blocks_to_place = self.counts.iter().sum();
        blocks_free >= blocks_to_place
    }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let (shapes_str, regions_str) = input
            .rsplit_once("\n\n")
            .ok_or(AoCError::Parsing)?;

        let shapes = shapes_str
            .split("\n\n")
            .map(|block| {
                block
                    .lines()
                    .skip(1)
                    .map(|line| line.chars().map(|c| c == '#').collect())
                    .collect()
            })
            .collect();

        let regions = regions_str
            .lines()
            .map(|line| {
                let (tree, counts) = line
                    .split_once(": ")
                    .ok_or(AoCError::Parsing)?;
                let (width, length) = tree
                    .split_once("x")
                    .ok_or(AoCError::Parsing)?;
                Ok(Region {
                    width: width.parse()?,
                    length: length.parse()?,
                    counts: counts
                        .split(" ")
                        .map(|s| s.parse().unwrap())
                        .collect(),
                })
            })
            .collect::<AoCResult<_>>()?;

        Ok(Self { shapes, regions })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        Ok(self
            .regions
            .iter()
            .filter(|region| region.definitely_fits())
            .count())
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        Ok("🎄")
    }
}
