use std::fmt::Display;

use aoc_core::{AoCError, Solution};
use itertools::Itertools;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<u32>);

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let containers = input
            .lines()
            .map(|line| line.parse().ok())
            .collect::<Option<Vec<u32>>>()
            .ok_or(AoCError::Parsing)?;
        Ok(Self(containers))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        Ok((1..self.0.len())
            // map amount of buckets used to amount of valid combinations with that amount of buckets
            .map(|num_buckets| {
                self.0
                    .iter()
                    .combinations(num_buckets)
                    .filter(|combo| combo.iter().copied().sum::<u32>() == 150)
                    .count()
            })
            .sum::<usize>())
    }

    fn part_2(&self) -> AoCResult<impl Display> {
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
            .ok_or(AoCError::Solving)
    }

    fn solve(self) -> AoCResult<Solution>
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

        Ok(Solution {
            part1: Box::new(p1),
            part2: Box::new(p2),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "43
3
4
10
21
44
4
6
47
41
34
17
17
44
36
31
46
9
27
38
";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "1638");
    }

    #[test]
    fn part_2() {
        let input = "43
3
4
10
21
44
4
6
47
41
34
17
17
44
36
31
46
9
27
38
";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "17");
    }

    #[test]
    fn solve() {
        let input = "43
3
4
10
21
44
4
6
47
41
34
17
17
44
36
31
46
9
27
38
";
        let data = Data::try_new(input).unwrap();
        let Solution { part1, part2 } = data.solve().unwrap();
        assert_eq!(part1.to_string(), "1638");
        assert_eq!(part2.to_string(), "17");
    }
}
