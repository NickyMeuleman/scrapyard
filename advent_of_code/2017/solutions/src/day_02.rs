use std::fmt::Display;

use itertools::Itertools;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let result: i32 = self
            .0
            .lines()
            .map(|line| {
                let (min, max) = line
                    .split_ascii_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .minmax()
                    .into_option()
                    .unwrap();
                max - min
            })
            .sum();

        Ok(result)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let result: i32 = self
            .0
            .lines()
            .filter_map(|line| {
                line.split_ascii_whitespace()
                    .filter_map(|s| s.parse::<i32>().ok())
                    .tuple_combinations()
                    .map(|(n1, n2)| if n1 > n2 { (n1, n2) } else { (n2, n1) })
                    .find(|(n1, n2)| n1 % *n2 == 0)
                    .map(|(n1, n2)| n1 / n2)
            })
            .sum();

        Ok(result)
    }

    // slower
    // pub fn part_2(input: &str) -> i32 {
    //     input
    //         .lines()
    //         .map(|line| {
    //             line.split_ascii_whitespace()
    //                 .map(|s| s.parse::<i32>().unwrap())
    //                 .permutations(2)
    //                 .find(|v| v[0] % v[1] == 0)
    //                 .map(|v| v[0] / v[1])
    //                 .unwrap()
    //         })
    //         .sum()
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "5 1 9 5
7 5 3
2 4 6 8";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "18");
    }

    #[test]
    fn part_2() {
        let input = "5 9 2 8
9 4 7 3
3 8 6 5";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "9");
    }
}
