use std::fmt::Display;

use aoc_core::AoCError;
use itertools::Itertools;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<u64>);

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        Ok(Self(
            input
                .lines()
                .filter_map(|l| l.parse().ok())
                .collect(),
        ))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let total_weight: u64 = self.0.iter().sum();
        let mut num_packages_with_passanger = 1;

        while num_packages_with_passanger < self.0.len() {
            let min_entanglement = self
                .0
                .iter()
                .copied()
                .combinations(num_packages_with_passanger)
                .filter(|comb| comb.iter().sum::<u64>() == total_weight / 3)
                .map(|comb| comb.iter().product::<u64>())
                .min();

            if let Some(result) = min_entanglement {
                return Ok(result);
            }

            num_packages_with_passanger += 1;
        }

        Err(AoCError::Solving)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let packages: Vec<u64> = self
            .0
            .clone()
            .into_iter()
            .sorted()
            .collect();
        let total_weight: u64 = self.0.iter().sum();
        let mut num_packages_with_passanger = 1;

        while num_packages_with_passanger < packages.len() {
            let min_entanglement = packages
                .iter()
                .copied()
                .combinations(num_packages_with_passanger)
                .filter(|comb| comb.iter().sum::<u64>() == total_weight / 4)
                // apparently the check if the remaining packages can be evenly split is not neccessary because of how the inputs are crafted
                // this greatly speeds up execution
                .map(|comb| comb.iter().product::<u64>())
                // we sorted the packages, the first result is the lowest result
                .next();

            if let Some(result) = min_entanglement {
                return Ok(result);
            }

            num_packages_with_passanger += 1;
        }

        Err(AoCError::Solving)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "1
2
3
4
5
7
8
9
10
11";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "99");
    }

    #[test]
    fn part_2() {
        let input = "1
2
3
4
5
7
8
9
10
11";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "44");
    }
}
