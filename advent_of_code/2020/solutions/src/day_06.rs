use std::{collections::HashSet, fmt::Display};

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<Vec<HashSet<char>>>);

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        fn parse_block(block: &str) -> Vec<HashSet<char>> {
            block
                .split("\n")
                .map(|line| parse_line(line))
                .collect()
        }

        fn parse_line(line: &str) -> HashSet<char> {
            line.chars().collect()
        }

        Ok(Self(
            input
                .split("\n\n")
                .map(|block| parse_block(block))
                .collect(),
        ))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let result: usize = self
            .0
            .iter()
            .map(|block| {
                let start_acc = HashSet::new();
                block
                    .iter()
                    .fold(start_acc, |acc, line| acc.union(line).map(|c| *c).collect())
                    .len()
            })
            .sum();

        Ok(result)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let result: usize = self
            .0
            .iter()
            .map(|block| {
                // Rust has an experimental fold_first method where the first item is the starting value of the acc
                let start_acc = block[0].clone();
                block
                    .iter()
                    .fold(start_acc, |acc, line| {
                        acc.intersection(line)
                            .map(|c| *c)
                            .collect()
                    })
                    .len()
            })
            .sum();

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "11");
    }

    #[test]
    fn part_2() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "6");
    }
}
