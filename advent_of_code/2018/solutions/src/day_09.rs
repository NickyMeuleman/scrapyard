use std::fmt::Display;

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data<'a>(usize, &'a str, usize);

#[derive(Debug, Clone, Copy)]
struct Marble {
    clockwise: usize,
    counterclockwise: usize,
}

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        let (players, points) = input.split_once(";").unwrap();
        let players = players
            .split_ascii_whitespace()
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let points = points.strip_suffix(" points").unwrap();
        let end_marble = points
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        Ok(Self(players, points, end_marble))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        // using a doubly linked list while not using a linked list, letsgooo
        let (players, end_marble) = (self.0, self.2);
        let mut marbles = vec![
            Marble {
                clockwise: 0,
                counterclockwise: 0
            };
            end_marble + 1
        ];
        let mut scores = vec![0; players];
        // First, the marble numbered 0 is placed in the circle.
        // At this point, while it contains only a single marble, it is still a circle: the marble is both clockwise from itself and counter-clockwise from itself.
        // This marble is designated the current marble.
        let mut current = 0;

        for (value, player) in (1..=end_marble).zip((0..players).cycle()) {
            if value % 23 != 0 {
                // Then, each Elf takes a turn placing
                // the lowest-numbered remaining marble into the circle
                // between the marbles that are 1 and 2 marbles clockwise of the current marble.
                let clockwise_1 = marbles[current].clockwise;
                let clockwise_2 = marbles[clockwise_1].clockwise;
                marbles[clockwise_1].clockwise = value;
                marbles[clockwise_2].counterclockwise = value;
                marbles[value].clockwise = clockwise_2;
                marbles[value].counterclockwise = clockwise_1;
                // (When the circle is large enough, this means that there is one marble between the marble that was just placed and the current marble.)
                // The marble that was just placed then becomes the current marble.
                current = value;
            } else {
                // However, if the marble that is about to be placed has a number which is a multiple of 23,
                // something entirely different happens.
                // First, the current player keeps the marble they would have placed,
                // adding it to their score.
                scores[player] += value;
                // In addition, the marble 7 marbles
                // counter-clockwise from the current marble is removed from the circle
                // and also added to the current player's score.
                let mut tmp = current;
                for _ in 0..7 {
                    tmp = marbles[tmp].counterclockwise;
                }
                scores[player] += tmp;
                let tmp_clockwise = marbles[tmp].clockwise;
                let tmp_counterclockwise = marbles[tmp].counterclockwise;
                marbles[tmp_counterclockwise].clockwise = tmp_clockwise;
                marbles[tmp_clockwise].counterclockwise = tmp_counterclockwise;
                // The marble located immediately clockwise of the marble that was removed becomes the new current marble.
                current = tmp_clockwise;
            }
        }

        scores
            .into_iter()
            .max()
            .ok_or(AoCError::Solving)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let (players, end_marble) = (self.0, self.2);
        let end_marble = end_marble * 100;
        let mut marbles = vec![
            Marble {
                clockwise: 0,
                counterclockwise: 0
            };
            end_marble + 1
        ];
        let mut scores = vec![0; players];
        // First, the marble numbered 0 is placed in the circle.
        // At this point, while it contains only a single marble, it is still a circle: the marble is both clockwise from itself and counter-clockwise from itself.
        // This marble is designated the current marble.
        let mut current = 0;

        for (value, player) in (1..=end_marble).zip((0..players).cycle()) {
            if value % 23 != 0 {
                // Then, each Elf takes a turn placing
                // the lowest-numbered remaining marble into the circle
                // between the marbles that are 1 and 2 marbles clockwise of the current marble.
                let clockwise_1 = marbles[current].clockwise;
                let clockwise_2 = marbles[clockwise_1].clockwise;
                marbles[clockwise_1].clockwise = value;
                marbles[clockwise_2].counterclockwise = value;
                marbles[value].clockwise = clockwise_2;
                marbles[value].counterclockwise = clockwise_1;
                // (When the circle is large enough, this means that there is one marble between the marble that was just placed and the current marble.)
                // The marble that was just placed then becomes the current marble.
                current = value;
            } else {
                // However, if the marble that is about to be placed has a number which is a multiple of 23,
                // something entirely different happens.
                // First, the current player keeps the marble they would have placed,
                // adding it to their score.
                scores[player] += value;
                // In addition, the marble 7 marbles
                // counter-clockwise from the current marble is removed from the circle
                // and also added to the current player's score.
                let mut tmp = current;
                for _ in 0..7 {
                    tmp = marbles[tmp].counterclockwise;
                }
                scores[player] += tmp;
                let tmp_clockwise = marbles[tmp].clockwise;
                let tmp_counterclockwise = marbles[tmp].counterclockwise;
                marbles[tmp_counterclockwise].clockwise = tmp_clockwise;
                marbles[tmp_clockwise].counterclockwise = tmp_counterclockwise;
                // The marble located immediately clockwise of the marble that was removed becomes the new current marble.
                current = tmp_clockwise;
            }
        }

        scores
            .into_iter()
            .max()
            .ok_or(AoCError::Solving)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "30 players; last marble is worth 5807 points";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "37305");
    }
}
