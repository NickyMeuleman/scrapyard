use std::{fmt::Display, num::ParseIntError};

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<i64>);

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let nums = input
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<i64>, ParseIntError>>()?;
        Ok(Self(nums))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        // indexes into nums
        let mut mixed: Vec<_> = (0..self.0.len()).collect();
        for (idx, &num) in self.0.iter().enumerate() {
            // find mixed that corresponds to the number in nums
            let mixed_idx = mixed
                .iter()
                .position(|&mix_num| mix_num == idx)
                .unwrap();
            // remove that index from mixed
            mixed.remove(mixed_idx);
            // add num offset to that number and add it back
            let new_mixed_idx = (mixed_idx as i64 + num).rem_euclid(mixed.len() as i64) as usize;
            mixed.insert(new_mixed_idx, idx);
        }

        let zero_idx = self
            .0
            .iter()
            .position(|&num| num == 0)
            .unwrap();
        let zero_mixed_idx = mixed
            .iter()
            .position(|&mix_num| mix_num == zero_idx)
            .unwrap();

        let result: i64 = [1000, 2000, 3000]
            .iter()
            .map(|offset| {
                let mixed_idx = (zero_mixed_idx + offset) % mixed.len();
                let nums_idx = mixed[mixed_idx];
                self.0[nums_idx]
            })
            .sum();

        Ok(result)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let decryption_key = 811_589_153;
        let nums: Vec<_> = self
            .0
            .iter()
            .map(|num| num * decryption_key)
            .collect();
        // indexes into nums
        let mut mixed: Vec<_> = (0..nums.len()).collect();
        for _ in 0..10 {
            for (idx, &num) in nums.iter().enumerate() {
                // find mixed that corresponds to the number in nums
                let mixed_idx = mixed
                    .iter()
                    .position(|&mix_num| mix_num == idx)
                    .unwrap();
                // remove that index from mixed
                mixed.remove(mixed_idx);
                // add num offset to that number and add it back
                let new_mixed_idx =
                    (mixed_idx as i64 + num).rem_euclid(mixed.len() as i64) as usize;
                mixed.insert(new_mixed_idx, idx);
            }
        }

        let zero_idx = nums
            .iter()
            .position(|&num| num == 0)
            .unwrap();
        let zero_mixed_idx = mixed
            .iter()
            .position(|&mix_num| mix_num == zero_idx)
            .unwrap();

        let result: i64 = [1000, 2000, 3000]
            .iter()
            .map(|offset| {
                let mixed_idx = (zero_mixed_idx + offset) % mixed.len();
                let nums_idx = mixed[mixed_idx];
                nums[nums_idx]
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
        let input = "1
2
-3
3
-2
0
4";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "3");
    }

    #[test]
    fn part_2() {
        let input = "1
2
-3
3
-2
0
4";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "1623178306");
    }
}
