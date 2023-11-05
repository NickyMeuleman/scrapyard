use std::{collections::HashMap, fmt::Display};

use aoc_core::Solution;
use itertools::{Itertools, MinMaxResult};

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let word_len = self.0.lines().next().unwrap().len();
        let mut maps = vec![HashMap::new(); word_len];

        for line in self.0.lines() {
            for (i, c) in line.chars().enumerate() {
                *maps[i].entry(c).or_insert(0) += 1
            }
        }

        let result: String = maps
            .into_iter()
            .map(|map| {
                map.into_iter()
                    .max_by_key(|(_c, count)| *count)
                    .unwrap()
                    .0
            })
            .collect();

        Ok(result)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let word_len = self.0.lines().next().unwrap().len();
        let mut maps = vec![HashMap::new(); word_len];

        for line in self.0.lines() {
            for (i, c) in line.chars().enumerate() {
                *maps[i].entry(c).or_insert(0) += 1
            }
        }

        let result: String = maps
            .into_iter()
            .map(|map| {
                map.into_iter()
                    .min_by_key(|(_c, count)| *count)
                    .unwrap()
                    .0
            })
            .collect();

        Ok(result)
    }

    fn solve(self) -> AoCResult<Solution>
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
            .map(|map| {
                map.into_iter()
                    .minmax_by_key(|(_c, count)| *count)
            })
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

        Ok(Solution {
            part1: Box::new(message1),
            part2: Box::new(message2),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "easter");
    }

    #[test]
    fn part_2() {
        let input = "eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "advent");
    }
}
