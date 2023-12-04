use std::fmt::Display;

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    // fn part_1(&self) -> AoCResult<impl Display> {
    //     let mut sum = 0;
    //     for line in self.0.lines() {
    //         let (_, numbers) = line.split_once(": ").unwrap();
    //         let (winning, holding) = numbers.split_once("|").unwrap();
    //         let winning = winning.split_whitespace();
    //         let holding: Vec<_> = holding.split_whitespace().collect();
    //         let num_winners = winning
    //             .filter(|s| holding.contains(s))
    //             .count() as u32;

    //         let score = match num_winners {
    //             0 => 0,
    //             n => 2u32.pow(n - 1),
    //         };
    //         sum += score;
    //     }
    //     Ok(sum)
    // }

    fn part_1(&self) -> AoCResult<impl Display> {
        self.0
            .lines()
            .map(|line| {
                let (_, numbers) = line
                    .split_once(": ")
                    .ok_or(AoCError::Parsing)?;
                let (winning, holding) = numbers
                    .split_once(" | ")
                    .ok_or(AoCError::Parsing)?;
                let winning = winning.split_whitespace();
                let holding: Vec<_> = holding.split_whitespace().collect();
                let num_winners = winning
                    .filter(|s| holding.contains(s))
                    .count();

                Ok(match num_winners {
                    0 => 0,
                    n => 2u32.pow(n as u32 - 1),
                })
            })
            .sum::<AoCResult<u32>>()
    }

    // fn part_2(&self) -> AoCResult<impl Display> {
    //     let mut counts = vec![1; self.0.lines().count()];

    //     for (idx, line) in self.0.lines().enumerate() {
    //         let (_, numbers) = line.split_once(": ").unwrap();
    //         let (winning, holding) = numbers.split_once("|").unwrap();
    //         let winning = winning.split_whitespace();
    //         let holding: Vec<_> = holding.split_whitespace().collect();
    //         let num_winners = winning
    //             .filter(|s| holding.contains(s))
    //             .count();

    //         let num_cards = counts[idx];
    //         for i in idx + 1..=idx + num_winners {
    //             counts[i] += num_cards;
    //         }
    //     }

    //     Ok(counts.iter().sum::<u32>())
    // }

    fn part_2(&self) -> AoCResult<impl Display> {
        self.0
            .lines()
            .enumerate()
            .scan(vec![1; self.0.lines().count()], |counts, (idx, line)| {
                let Some((_, numbers)) = line.split_once(": ") else {
                    return Some(Err(AoCError::Parsing));
                };
                let Some((winning, holding)) = numbers.split_once("|") else {
                    return Some(Err(AoCError::Parsing));
                };
                let winning = winning.split_whitespace();
                let holding: Vec<_> = holding.split_whitespace().collect();
                let num_winners = winning
                    .filter(|s| holding.contains(s))
                    .count();

                let num_cards = counts[idx];
                for i in idx + 1..=idx + num_winners {
                    counts[i] += num_cards;
                }

                Some(Ok(num_cards))
            })
            .sum::<AoCResult<u32>>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "13");
    }

    #[test]
    fn part_2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "30");
    }
}
