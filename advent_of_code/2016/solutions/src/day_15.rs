use std::{fmt::Display, iter};

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<Disc>);

#[derive(Debug, Clone)]
struct Disc {
    positions: u32,
    start: u32,
}

impl Disc {
    fn is_open(&self, time_passed: u32) -> bool {
        (time_passed + self.start) % self.positions == 0
    }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut discs = Vec::new();
        for line in input.lines() {
            let rest = line
                .strip_prefix("Disc #")
                .ok_or(AoCError::Parsing)?;
            let (_num, rest) = rest
                .split_once(" has ")
                .ok_or(AoCError::Parsing)?;
            let (positions, rest) = rest
                .split_once(" positions; at time=0, it is at position ")
                .ok_or(AoCError::Parsing)?;
            let start = rest
                .strip_suffix('.')
                .ok_or(AoCError::Parsing)?;

            discs.push(Disc {
                positions: positions.parse()?,
                start: start.parse()?,
            });
        }
        Ok(Self(discs))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        for time in 0.. {
            if self
                .0
                .iter()
                .enumerate()
                .all(|(disc_idx, disc)| disc.is_open((time + disc_idx + 1) as u32))
            {
                return Ok(time);
            }
        }
        Err(AoCError::Solving)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        for time in 0.. {
            if self
                .0
                .iter()
                .chain(iter::once(&Disc {
                    positions: 11,
                    start: 0,
                }))
                .enumerate()
                .all(|(disc_idx, disc)| disc.is_open((time + disc_idx + 1) as u32))
            {
                return Ok(time);
            }
        }
        Err(AoCError::Solving)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "Disc #1 has 5 positions; at time=0, it is at position 4.
Disc #2 has 2 positions; at time=0, it is at position 1.";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "5");
    }

    #[test]
    fn part_2() {
        let input = "Disc #1 has 5 positions; at time=0, it is at position 4.
Disc #2 has 2 positions; at time=0, it is at position 1.";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "85");
    }
}
