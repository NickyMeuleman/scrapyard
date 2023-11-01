use std::fmt::Display;

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

const TARGET: i32 = 2020;

#[derive(Debug, Clone)]
pub struct Data {
    nums: Vec<i32>,
}

fn find_2(nums: &Vec<i32>, target: i32) -> Option<(i32, i32)> {
    for num in nums {
        if let Some(needed) = nums
            .iter()
            .find(|&&num2| num2 == target - num)
        {
            return Some((*num, *needed));
        }
    }
    None
}

fn find_3(nums: &Vec<i32>, target: i32) -> Option<(i32, i32, i32)> {
    for num in nums {
        if let Some(tuple) = find_2(nums, target - num) {
            let (one, two) = tuple;
            return Some((*num, one, two));
        }
    }
    None
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let nums = input
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { nums })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let (one, two) = find_2(&self.nums, TARGET).ok_or(AoCError::Parsing)?;
        Ok(one * two)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let (one, two, three) = find_3(&self.nums, TARGET).ok_or(AoCError::Parsing)?;
        Ok(one * two * three)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "1721
979
366
299
675
1456
";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "514579");
    }

    #[test]
    fn part_2() {
        let input = "1721
979
366
299
675
1456
";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "241861950");
    }
}
