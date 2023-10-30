use std::fmt::Display;

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<[[u32; 2]; 2]>);

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        fn to_range(s: &str) -> AoCResult<[u32; 2]> {
            let (min, max) = s
                .split_once('-')
                .ok_or(AoCError::new("Parsing Failed"))?;
            Ok([min.parse()?, max.parse()?])
        }

        let mut pairs = Vec::new();
        for line in input.lines() {
            let (elf1, elf2) = line
                .split_once(',')
                .ok_or(AoCError::new("Parsing Failed"))?;
            pairs.push([to_range(elf1)?, to_range(elf2)?]);
        }

        Ok(Self(pairs))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let result: u32 = self
            .0
            .iter()
            .filter(|[[min1, max1], [min2, max2]]| {
                (min1 >= min2 && max1 <= max2) || (min2 >= min1 && max2 <= max1)
                // equivalent:
                // (min1 <= min2 && max1 >= max2) || (min2 <= min1 && max2 >= max1)
            })
            .count()
            .try_into()?;

        Ok(result)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let result: u32 = self
            .0
            .iter()
            .filter(|[[min1, max1], [min2, max2]]| {
                min1.max(min2) <= max1.min(max2)
                // equivalent:
                // min1 <= max2 && max1 >= min2
            })
            .count()
            .try_into()?;

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "2");
    }

    #[test]
    fn part_2() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "4");
    }
}
