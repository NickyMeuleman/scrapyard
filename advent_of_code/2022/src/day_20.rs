use crate::AoCData;

pub struct Data(Vec<i64>);

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        let nums = input
            .lines()
            .map(|line| line.parse().ok())
            .collect::<Option<Vec<i64>>>()?;
        Some(Self(nums))
    }

    fn part_1(&self) -> String {
        // indexes into nums
        let mut mixed: Vec<_> = (0..self.0.len()).collect();
        for (idx, &num) in self.0.iter().enumerate() {
            // find mixed that corresponds to the number in nums
            let mixed_idx = mixed.iter().position(|&mix_num| mix_num == idx).unwrap();
            // remove that index from mixed
            mixed.remove(mixed_idx);
            // add num offset to that number and add it back
            let new_mixed_idx = (mixed_idx as i64 + num).rem_euclid(mixed.len() as i64) as usize;
            mixed.insert(new_mixed_idx, idx);
        }

        let zero_idx = self.0.iter().position(|&num| num == 0).unwrap();
        let zero_mixed_idx = mixed
            .iter()
            .position(|&mix_num| mix_num == zero_idx)
            .unwrap();

        [1000, 2000, 3000]
            .iter()
            .map(|offset| {
                let mixed_idx = (zero_mixed_idx + offset) % mixed.len();
                let nums_idx = mixed[mixed_idx];
                self.0[nums_idx]
            })
            .sum::<i64>()
            .to_string()
    }

    fn part_2(&self) -> String {
        let decryption_key = 811_589_153;
        let nums: Vec<_> = self.0.iter().map(|num| num * decryption_key).collect();
        // indexes into nums
        let mut mixed: Vec<_> = (0..nums.len()).collect();
        for _ in 0..10 {
            for (idx, &num) in nums.iter().enumerate() {
                // find mixed that corresponds to the number in nums
                let mixed_idx = mixed.iter().position(|&mix_num| mix_num == idx).unwrap();
                // remove that index from mixed
                mixed.remove(mixed_idx);
                // add num offset to that number and add it back
                let new_mixed_idx =
                    (mixed_idx as i64 + num).rem_euclid(mixed.len() as i64) as usize;
                mixed.insert(new_mixed_idx, idx);
            }
        }

        let zero_idx = nums.iter().position(|&num| num == 0).unwrap();
        let zero_mixed_idx = mixed
            .iter()
            .position(|&mix_num| mix_num == zero_idx)
            .unwrap();

        [1000, 2000, 3000]
            .iter()
            .map(|offset| {
                let mixed_idx = (zero_mixed_idx + offset) % mixed.len();
                let nums_idx = mixed[mixed_idx];
                nums[nums_idx]
            })
            .sum::<i64>()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(20);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "3");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(20);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "1623178306");
    }
}
