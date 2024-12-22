// Blog writeup with simpler Rust code (I should handle errors here):
// https://nickymeuleman.netlify.app/blog/aoc2024-day22/

use crate::{AoCData, AoCError, AoCResult};
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

fn mix(a: i64, b: i64) -> i64 {
    a ^ b
}

fn prune(a: i64) -> i64 {
    a.rem_euclid(16777216)
}

fn next(mut n: i64) -> i64 {
    n = prune(mix(n, n * 64));
    n = prune(mix(n, n / 32));
    n = prune(mix(n, n * 2048));
    n
}

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        self.0
            .lines()
            .map(|line| {
                let mut num = line
                    .parse()
                    .map_err(|_| AoCError::Parsing)?;
                for _ in 0..2000 {
                    num = next(num);
                }
                Ok(num)
            })
            .sum::<AoCResult<i64>>()
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut profitmap: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();

        for line in self.0.lines() {
            let mut num = line
                .parse()
                .map_err(|_| AoCError::Parsing)?;
            let mut prices = [0; 2000];
            for price in prices.iter_mut() {
                num = next(num);
                *price = num.rem_euclid(10);
            }

            let mut seen = HashSet::new();
            for (a, b, c, d, e) in prices.iter().tuple_windows() {
                let diffs = ((b - a), (c - b), (d - c), (e - d));
                // only buy once
                if seen.insert(diffs) {
                    *profitmap.entry(diffs).or_default() += *e;
                }
            }
        }

        profitmap
            .into_values()
            .max()
            .ok_or(AoCError::Solving)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "1
10
100
2024";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "37327623");
    }

    #[test]
    fn part_2() {
        let input = "1
2
3
2024";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "23");
    }
}
