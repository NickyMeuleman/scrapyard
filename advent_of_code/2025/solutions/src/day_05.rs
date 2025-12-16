// Blog writeup with simpler Rust code (I should handle errors here):
// https://nickymeuleman.netlify.app/blog/aoc2025-day05/

use crate::{AoCData, AoCError, AoCResult};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Data {
    ranges: Vec<(u64, u64)>,
    foods: Vec<u64>,
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let (top, bottom) = input
            .split_once("\n\n")
            .ok_or(AoCError::Parsing)?;

        let ranges = top
            .lines()
            .map(|line| {
                let (low, high) = line
                    .split_once('-')
                    .ok_or(AoCError::Parsing)?;
                Ok((low.parse()?, high.parse()?))
            })
            .collect::<AoCResult<Vec<_>>>()?;

        let foods = bottom
            .lines()
            .map(|line| line.parse().map_err(Into::into))
            .collect::<AoCResult<Vec<u64>>>()?;

        Ok(Self { ranges, foods })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        Ok(self
            .foods
            .iter()
            .filter(|food| {
                self.ranges
                    .iter()
                    .any(|(low, high)| (low..=high).contains(food))
            })
            .count())
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut ranges = self.ranges.clone();
        ranges.sort_unstable_by_key(|range| range.0);

        let mut sum = 0;
        let &(mut low, mut high) = ranges
            .first()
            .ok_or(AoCError::Solving)?;

        for &(next_low, next_high) in ranges.iter().skip(1) {
            if high < next_low {
                sum += high - low + 1;
                (low, high) = (next_low, next_high);
            } else {
                // overlap, merge ranges
                high = high.max(next_high);
            }
        }
        // count last range
        sum += high - low + 1;

        Ok(sum)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "3");
    }

    #[test]
    fn part_2() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "14");
    }
}
