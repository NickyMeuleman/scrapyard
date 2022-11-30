use std::iter;

use crate::AoCData;

pub struct Data(Vec<Disc>);
#[derive(Debug)]
struct Disc {
    positions: u32,
    start: u32,
}

impl Disc {
    fn is_open(&self, time_passed: u32) -> bool {
        (time_passed + self.start) % self.positions == 0
    }
}

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        let mut discs = Vec::new();
        for line in input.lines() {
            let rest = line.strip_prefix("Disc #")?;
            let (_num, rest) = rest.split_once(" has ")?;
            let (positions, rest) = rest.split_once(" positions; at time=0, it is at position ")?;
            let start = rest.strip_suffix('.')?;

            discs.push(Disc {
                positions: positions.parse().ok()?,
                start: start.parse().ok()?,
            });
        }
        Some(Self(discs))
    }

    fn part_1(&self) -> String {
        for time in 0.. {
            if self
                .0
                .iter()
                .enumerate()
                .all(|(disc_idx, disc)| disc.is_open((time + disc_idx + 1) as u32))
            {
                return time.to_string();
            }
        }
        unreachable!()
    }

    fn part_2(&self) -> String {
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
                return time.to_string();
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(15);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "5");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(15);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "85");
    }
}
