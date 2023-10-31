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
        let result: u32 = self
            .0
            .lines()
            .map(|line| line.as_bytes())
            .filter_map(|line| {
                let (left, right) = line.split_at(line.len() / 2);

                right
                    .iter()
                    .find(|num| left.contains(*num))
                    .map(|num| match num {
                        b'a'..=b'z' => (num - b'a') as u32 + 1,
                        _ => (num - b'A') as u32 + 1 + 26,
                    })
            })
            .sum();

        Ok(result)

        // alternative version with HashSets
        //     self.0
        //         .lines()
        //         .map(|line| {
        //             let (left, right) = line.split_at(line.len() / 2);
        //             let sack1: HashSet<_> = left.chars().collect();
        //             let sack2: HashSet<_> = right.chars().collect();
        //             let mut common = sack1.intersection(&sack2);
        //             let common = *common.next().unwrap() as u8; // problem states there is always one
        //             if common.is_ascii_lowercase() {
        //                 (common - b'a' + 1) as u32
        //             } else {
        //                 (common - b'A' + 27) as u32
        //             }
        //         })
        //         .sum::<u32>()
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let result: u32 = self
            .0
            .lines()
            .map(|line| line.as_bytes())
            .tuples()
            .filter_map(|(sack1, sack2, sack3)| {
                sack1
                    .iter()
                    .find(|num| sack2.contains(num) && sack3.contains(num))
                    .map(|common| match common {
                        b'a'..=b'z' => (common - b'a') as u32 + 1,
                        _ => (common - b'A') as u32 + 1 + 26,
                    })
            })
            .sum();

        Ok(result)

        // alternative version with HashSets
        //     self.0
        //         .lines()
        //         .tuples()
        //         .map(|(sack1, sack2, sack3)| {
        //             let sack1: HashSet<_> = sack1.chars().collect();
        //             let sack2: HashSet<_> = sack2.chars().collect();
        //             let sack3: HashSet<_> = sack3.chars().collect();
        //             let common_one_two: HashSet<_> = sack1.intersection(&sack2).copied().collect();
        //             let mut common = common_one_two.intersection(&sack3);

        //             let common = *common.next().unwrap() as u8; // problem states there is always one
        //             if common.is_ascii_lowercase() {
        //                 (common - b'a' + 1) as u32
        //             } else {
        //                 (common - b'A' + 27) as u32
        //             }
        //         })
        //         .sum::<u32>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "157");
    }

    #[test]
    fn part_2() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "70");
    }
}
