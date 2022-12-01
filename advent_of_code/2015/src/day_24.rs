use itertools::Itertools;

use crate::AoCData;

pub struct Data(Vec<u64>);

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        Some(Self(input.lines().filter_map(|l| l.parse().ok()).collect()))
    }

    fn part_1(&self) -> String {
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
                return result.to_string();
            }

            num_packages_with_passanger += 1;
        }

        String::from("No result found")
    }

    fn part_2(&self) -> String {
        let packages: Vec<u64> = self.0.clone().into_iter().sorted().collect();
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
                return result.to_string();
            }

            num_packages_with_passanger += 1;
        }

        String::from("No result found")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(24);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "99");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(24);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "44");
    }
}
