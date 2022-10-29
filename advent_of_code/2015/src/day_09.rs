use std::collections::{HashMap, HashSet};

use crate::{utils::Solution, AoCData};
use itertools::{Itertools, MinMaxResult};

pub struct Data {
    cities: HashSet<String>,
    dist_map: HashMap<(String, String), u32>,
}

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        let mut cities = HashSet::new();
        let mut dist_map = HashMap::new();
        for line in input.lines() {
            let (left, right) = line.split_once(" = ")?;
            let dist: u32 = right.parse().ok()?;
            let (from, to) = left.split_once(" to ")?;
            cities.insert(from.to_string());
            cities.insert(to.to_string());
            dist_map.insert((from.to_string(), to.to_string()), dist);
            dist_map.insert((to.to_string(), from.to_string()), dist);
        }
        Some(Self { cities, dist_map })
    }

    fn part_1(&self) -> String {
        self.cities
            .iter()
            .permutations(self.cities.len())
            .filter_map(|perm| {
                let cost = perm
                    .into_iter()
                    .tuple_windows()
                    .try_fold(0, |acc, (from, to)| {
                        let dist = self.dist_map.get(&(from.to_string(), to.to_string()))?;
                        Some(acc + dist)
                    })?;
                Some(cost)
            })
            .min()
            .unwrap_or(u32::MAX)
            .to_string()
    }

    fn part_2(&self) -> String {
        self.cities
            .iter()
            .permutations(self.cities.len())
            .filter_map(|perm| {
                let cost = perm
                    .into_iter()
                    .tuple_windows()
                    .try_fold(0, |acc, (from, to)| {
                        let dist = self.dist_map.get(&(from.to_string(), to.to_string()))?;
                        Some(acc + dist)
                    })?;
                Some(cost)
            })
            .max()
            .unwrap_or(u32::MIN)
            .to_string()
    }

    fn solve(self) -> crate::utils::Solution
    where
        Self: Sized,
    {
        self.cities
            .iter()
            .permutations(self.cities.len())
            .filter_map(|perm| {
                let cost = perm
                    .into_iter()
                    .tuple_windows()
                    .try_fold(0, |acc, (from, to)| {
                        let dist = self.dist_map.get(&(from.to_string(), to.to_string()))?;
                        Some(acc + dist)
                    })?;
                Some(cost)
            })
            .minmax()
            .into_option()
            .map(|(min, max)| Solution {
                part1: min.to_string(),
                part2: max.to_string(),
            })
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(9);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "605");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(9);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "982");
    }

    #[test]
    fn solve() {
        let input = utils::get_sample_input(9);
        let data = Data::try_new(input).unwrap();
        let solution = data.solve();
        assert_eq!(solution.part1, "605");
        assert_eq!(solution.part2, "982");
    }
}
