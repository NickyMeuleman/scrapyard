use std::collections::HashMap;

use itertools::{Itertools, MinMaxResult};

use crate::{utils::Solution, AoCData};

pub struct Data(String);

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        Some(Self(input))
    }

    fn part_1(&self) -> String {
        let word_len = self.0.lines().next().unwrap().len();
        let mut maps = vec![HashMap::new(); word_len];

        for line in self.0.lines() {
            for (i, c) in line.chars().enumerate() {
                *maps[i].entry(c).or_insert(0) += 1
            }
        }

        maps.into_iter()
            .map(|map| map.into_iter().max_by_key(|(_c, count)| *count).unwrap().0)
            .collect()
    }

    fn part_2(&self) -> String {
        let word_len = self.0.lines().next().unwrap().len();
        let mut maps = vec![HashMap::new(); word_len];

        for line in self.0.lines() {
            for (i, c) in line.chars().enumerate() {
                *maps[i].entry(c).or_insert(0) += 1
            }
        }

        maps.into_iter()
            .map(|map| map.into_iter().min_by_key(|(_c, count)| *count).unwrap().0)
            .collect()
    }

    fn solve(self) -> crate::utils::Solution
    where
        Self: Sized,
    {
        let word_len = self.0.lines().next().unwrap().len();
        let mut maps = vec![HashMap::new(); word_len];

        for line in self.0.lines() {
            for (i, c) in line.chars().enumerate() {
                *maps[i].entry(c).or_insert(0) += 1
            }
        }

        let (message1, message2) = maps
            .into_iter()
            .map(|map| map.into_iter().minmax_by_key(|(_c, count)| *count))
            .fold(
                (String::new(), String::new()),
                |(mut message1, mut message2), item| {
                    if let MinMaxResult::MinMax(min, max) = item {
                        message1.push(max.0);
                        message2.push(min.0);
                    }
                    (message1, message2)
                },
            );

        Solution {
            part1: message1,
            part2: message2,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(6);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "easter");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(6);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "advent");
    }

    #[test]
    fn solve() {
        let input = utils::get_sample_input(6);
        let data = Data::try_new(input).unwrap();
        let Solution { part1, part2 } = data.solve();
        assert_eq!(part1, "easter");
        assert_eq!(part2, "advent");
    }
}
