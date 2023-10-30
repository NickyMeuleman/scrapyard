use std::fmt::Display;

use aoc_core::AoCError;
use itertools::Itertools;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let result = self
            .0
            .split("\n\n")
            .map(|elf| {
                elf.lines()
                    .filter_map(|s| s.parse::<u32>().ok())
                    .sum::<u32>()
            })
            .max();

        if let Some(max) = result {
            Ok(max)
        } else {
            Err(AoCError::new("No maximum found"))
        }
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let result = self
            .0
            .split("\n\n")
            .map(|elf| {
                elf.lines()
                    .filter_map(|s| s.parse::<u32>().ok())
                    .sum::<u32>()
            })
            .sorted()
            .rev()
            .take(3)
            .sum::<u32>();

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "24000");
    }

    #[test]
    fn part_2() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "45000");
    }
}
