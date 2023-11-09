use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let counts: Vec<HashMap<char, u32>> = self
            .0
            .lines()
            .map(|line| {
                let hash_map = HashMap::new();
                line.chars()
                    .fold(hash_map, |mut acc, c| {
                        *acc.entry(c).or_default() += 1;
                        acc
                    })
            })
            .collect();
        let has_double = counts
            .iter()
            .filter(|map| map.values().contains(&2))
            .count();
        let has_triple = counts
            .iter()
            .filter(|map| map.values().contains(&3))
            .count();

        Ok(has_double * has_triple)
    }

    // cooler but slower (because of extra allocation?)
    // pub fn part_2(input: &str) -> String {
    //     input
    //         .lines()
    //         .tuple_combinations()
    //         .find_map(|(first, second)| {
    //             let (same, different): (Vec<_>, Vec<_>) = first
    //                 .chars()
    //                 .zip(second.chars())
    //                 .partition(|(c1, c2)| c1 == c2);
    //             if different.len() == 1 {
    //                 Some(same)
    //             } else {
    //                 None
    //             }
    //         })
    //         .map(|same| same.into_iter().map(|(c1, _)| c1).collect())
    //         .unwrap()
    // }

    fn part_2(&self) -> AoCResult<impl Display> {
        let (first, second) = self
            .0
            .lines()
            .tuple_combinations()
            .find(|(first, second)| {
                let dist = first
                    .chars()
                    .zip(second.chars())
                    .filter(|(c1, c2)| c1 != c2)
                    .count();
                dist == 1
            })
            .unwrap();

        let result: String = first
            .chars()
            .zip(second.chars())
            .filter(|(c1, c2)| c1 == c2)
            .map(|(c1, _)| c1)
            .collect();

        Ok(result)
    }
}
