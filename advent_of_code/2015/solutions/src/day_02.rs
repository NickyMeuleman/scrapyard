use std::fmt::Display;

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<Dimensions>);

#[derive(Debug, Clone, Copy)]
struct Dimensions {
    l: i32,
    w: i32,
    h: i32,
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let dimensions = input
            .lines()
            .map(|line| {
                let nums = line
                    .split('x')
                    .map(|s| s.parse().ok())
                    .collect::<Option<Vec<i32>>>()?;
                Some(Dimensions {
                    l: nums[0],
                    w: nums[1],
                    h: nums[2],
                })
            })
            .collect::<Option<Vec<Dimensions>>>()
            .ok_or(AoCError::Parsing)?;

        Ok(Self(dimensions))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let result: i32 = self
            .0
            .iter()
            .map(|Dimensions { l, w, h }| {
                2 * l * w + 2 * w * h + 2 * l * h + (l * w).min(w * h).min(l * h)
            })
            .sum();

        Ok(result)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let result: i32 = self
            .0
            .iter()
            .map(|Dimensions { w, l, h }| {
                let mut sides = vec![w, l, h];
                sides.sort();
                2 * sides[0] + 2 * sides[1] + l * w * h
            })
            .sum();

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "2x3x4";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "58");
    }

    #[test]
    fn part_2() {
        let input = "2x3x4";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "34");
    }
}
