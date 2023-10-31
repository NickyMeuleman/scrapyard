use std::{fmt::Display, num::ParseIntError};

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data {
    nums: Vec<u32>,
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let nums = input
            .trim()
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<u32>, ParseIntError>>()?;
        Ok(Self { nums })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let result = self
            .nums
            .windows(2)
            .filter(|window| window[0] < window[1])
            .count();

        Ok(result)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let result =         // turn data into sums of three-measurement windows
        self.nums
            .windows(3)
            .map(|window| window.iter().sum())
            .collect::<Vec<u32>>()
            // count the amount of times a three-measurement sum increases
            .windows(2)
            .filter(|window| window[0] < window[1])
            .count();

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "199
200
208
210
200
207
240
269
260
263";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "7");
    }

    #[test]
    fn part_2() {
        let input = "199
200
208
210
200
207
240
269
260
263";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "5");
    }
}
