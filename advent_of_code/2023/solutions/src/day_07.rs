use std::{cmp::Ordering, fmt::Display};

use itertools::Itertools;

use crate::{AoCData, AoCResult};

#[derive(PartialEq, Eq)]
struct Line1<'a> {
    cards: &'a str,
    bid: u32,
}
#[derive(PartialEq, Eq)]
struct Line2<'a> {
    cards: &'a str,
    bid: u32,
}

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        fn card_value(c: char) -> usize {
            "23456789TJQKA"
                .chars()
                .position(|card| card == c)
                .unwrap()
        }

        impl<'a> Line1<'a> {
            fn score(&self) -> [u8; 2] {
                let mut faces = [0; 13];
                for c in self.cards.chars() {
                    faces[card_value(c)] += 1;
                }
                faces.sort_unstable();
                let mut score: [u8; 2] = faces[11..].try_into().unwrap();
                score.reverse();
                score
            }
        }

        impl<'a> PartialOrd for Line1<'a> {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl<'a> Ord for Line1<'a> {
            fn cmp(&self, other: &Self) -> Ordering {
                self.score()
                    .cmp(&other.score())
                    .then_with(|| {
                        let (a, b) = self
                            .cards
                            .chars()
                            .zip(other.cards.chars())
                            .find(|(a, b)| a != b)
                            .expect("hands are completely identical");

                        card_value(a).cmp(&card_value(b))
                    })
            }
        }

        let sum: u32 = self
            .0
            .lines()
            .map(|line| {
                let (cards, bid) = line.split_once(" ").unwrap();
                Line1 {
                    cards,
                    bid: bid.parse().unwrap(),
                }
            })
            .sorted_by(|a, b| a.cmp(&b))
            .enumerate()
            .map(|(idx, turn)| (idx as u32 + 1) * turn.bid)
            .sum();

        Ok(sum)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        fn card_value(c: char) -> usize {
            "J23456789TQKA"
                .chars()
                .position(|card| card == c)
                .unwrap()
        }

        impl<'a> Line2<'a> {
            fn score(&self) -> [u8; 2] {
                let mut faces = [0; 13];
                let mut jokers = 0;
                for c in self.cards.chars() {
                    if c == 'J' {
                        jokers += 1;
                    } else {
                        faces[card_value(c)] += 1;
                    }
                }
                faces.sort_unstable();
                let mut score: [u8; 2] = faces[11..].try_into().unwrap();
                score.reverse();
                // add the amount of jokers to the counts of the card that occurs the most already to increase the hand score
                score[0] += jokers;
                score
            }
        }

        impl<'a> PartialOrd for Line2<'a> {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl<'a> Ord for Line2<'a> {
            fn cmp(&self, other: &Self) -> Ordering {
                self.score()
                    .cmp(&other.score())
                    .then_with(|| {
                        let (a, b) = self
                            .cards
                            .chars()
                            .zip(other.cards.chars())
                            .find(|(a, b)| a != b)
                            .expect("hands are completely identical");

                        card_value(a).cmp(&card_value(b))
                    })
            }
        }

        let sum: u32 = self
            .0
            .lines()
            .map(|line| {
                let (cards, bid) = line.split_once(" ").unwrap();
                Line2 {
                    cards,
                    bid: bid.parse().unwrap(),
                }
            })
            .sorted_by(|a, b| a.cmp(&b))
            .enumerate()
            .map(|(idx, turn)| (idx as u32 + 1) * turn.bid)
            .sum();

        Ok(sum)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "6440");
    }

    #[test]
    fn part_2() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "5905");
    }
}
