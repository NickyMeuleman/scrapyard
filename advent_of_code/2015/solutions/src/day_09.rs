use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use aoc_core::{AoCError, Solution};
use itertools::Itertools;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data {
    cities: HashSet<String>,
    dist_map: HashMap<(String, String), u32>,
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut cities = HashSet::new();
        let mut dist_map = HashMap::new();
        for line in input.lines() {
            let (left, right) = line
                .split_once(" = ")
                .ok_or(AoCError::Parsing)?;
            let dist: u32 = right.parse()?;
            let (from, to) = left
                .split_once(" to ")
                .ok_or(AoCError::Parsing)?;
            cities.insert(from.to_string());
            cities.insert(to.to_string());
            dist_map.insert((from.to_string(), to.to_string()), dist);
            dist_map.insert((to.to_string(), from.to_string()), dist);
        }
        Ok(Self { cities, dist_map })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        self.cities
            .iter()
            .permutations(self.cities.len())
            .filter_map(|perm| {
                let cost = perm
                    .into_iter()
                    .tuple_windows()
                    .try_fold(0, |acc, (from, to)| {
                        let dist = self
                            .dist_map
                            .get(&(from.to_string(), to.to_string()))?;
                        Some(acc + dist)
                    })?;
                Some(cost)
            })
            .min()
            .ok_or(AoCError::Solving)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        self.cities
            .iter()
            .permutations(self.cities.len())
            .filter_map(|perm| {
                let cost = perm
                    .into_iter()
                    .tuple_windows()
                    .try_fold(0, |acc, (from, to)| {
                        let dist = self
                            .dist_map
                            .get(&(from.to_string(), to.to_string()))?;
                        Some(acc + dist)
                    })?;
                Some(cost)
            })
            .max()
            .ok_or(AoCError::Solving)
    }

    fn solve(self) -> AoCResult<aoc_core::Solution>
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
                        let dist = self
                            .dist_map
                            .get(&(from.to_string(), to.to_string()))?;
                        Some(acc + dist)
                    })?;
                Some(cost)
            })
            .minmax()
            .into_option()
            .map(|(min, max)| Solution {
                part1: Box::new(min),
                part2: Box::new(max),
            })
            .ok_or(AoCError::Solving)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "605");
    }

    #[test]
    fn part_2() {
        let input = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "982");
    }

    #[test]
    fn solve() {
        let input = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";
        let data = Data::try_new(input).unwrap();
        let Solution { part1, part2 } = data.solve().unwrap();
        assert_eq!(part1.to_string(), "605");
        assert_eq!(part2.to_string(), "982");
    }
}
