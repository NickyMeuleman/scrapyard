// use std::collections::HashSet;

use itertools::Itertools;

use crate::AoCData;

pub struct Data(String);

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        Some(Self(input))
    }

    fn part_1(&self) -> String {
        self.0
            .lines()
            .map(|line| line.as_bytes())
            .filter_map(|line| {
                let (left, right) = line.split_at(line.len() / 2);

                right
                    .iter()
                    .find(|num| left.contains(*num))
                    .map(|num| match num {
                        b'a'..=b'z' => (num - b'a') as u32 + 1,
                        b'A'..=b'Z' => (num - b'A') as u32 + 1 + 26,
                        _ => panic!("invalid input"),
                    })
            })
            .sum::<u32>()
            .to_string()
    }

    // alternative version with HashSets
    // fn part_1(&self) -> String {
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
    //         .to_string()
    // }

    fn part_2(&self) -> String {
        self.0
            .lines()
            .map(|line| line.as_bytes())
            .tuples()
            .filter_map(|(sack1, sack2, sack3)| {
                sack1
                    .iter()
                    .find(|num| sack2.contains(num) && sack3.contains(num))
                    .map(|common| match common {
                        b'a'..=b'z' => (common - b'a') as u32 + 1,
                        b'A'..=b'Z' => (common - b'A') as u32 + 1 + 26,
                        _ => panic!("invalid input"),
                    })
            })
            .sum::<u32>()
            .to_string()
    }

    // alternative version with HashSets
    // fn part_2(&self) -> String {
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
    //         .to_string()
    // }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(3);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "157");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(3);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "70");
    }
}
