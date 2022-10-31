use itertools::Itertools;

use crate::{utils::Solution, AoCData};

pub struct Data(Vec<u32>);

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        let containers = input
            .lines()
            .map(|line| line.parse().ok())
            .collect::<Option<Vec<u32>>>()?;
        Some(Self(containers))
    }

    fn part_1(&self) -> String {
        (1..self.0.len())
            // map amount of buckets used to amount of valid combinations with that amount of buckets
            .map(|num_buckets| {
                self.0
                    .iter()
                    .combinations(num_buckets)
                    .filter(|combo| combo.iter().copied().sum::<u32>() == 150)
                    .count()
            })
            .sum::<usize>()
            .to_string()
    }

    fn part_2(&self) -> String {
        (1..self.0.len())
            // map amount of buckets used to amount of valid combinations with that amount of buckets
            // return the first result (ie. the result with the minimum amount of buckets used)
            .find_map(|num_buckets| {
                let valid = self
                    .0
                    .iter()
                    .combinations(num_buckets)
                    .filter(|combo| combo.iter().copied().sum::<u32>() == 150)
                    .count();
                if valid > 0 {
                    Some(valid)
                } else {
                    None
                }
            })
            .unwrap_or(0)
            .to_string()
    }

    fn solve(self) -> crate::utils::Solution
    where
        Self: Sized,
    {
        let (p1, p2) = (1..self.0.len()).fold((0, 0), |(p1, p2), num_buckets| {
            let valid = self
                .0
                .iter()
                .combinations(num_buckets)
                .filter(|combo| combo.iter().copied().sum::<u32>() == 150)
                .count();
            if p2 > 0 {
                (p1 + valid, p2)
            } else {
                (p1 + valid, valid)
            }
        });

        Solution {
            part1: p1.to_string(),
            part2: p2.to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_input(17);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "1638");
    }

    #[test]
    fn part_2() {
        let input = utils::get_input(17);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "17");
    }

    #[test]
    fn solve() {
        let input = utils::get_input(17);
        let data = Data::try_new(input).unwrap();
        let Solution { part1, part2 } = data.solve();
        assert_eq!(part1, "1638");
        assert_eq!(part2, "17");
    }
}
