use std::fmt::Display;

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

struct Race {
    time: u32,
    dist: u32,
}

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let (time, dist) = self
            .0
            .split_once("\n")
            .ok_or(AoCError::Parsing)?;
        let time = time
            .strip_prefix("Time: ")
            .ok_or(AoCError::Parsing)?
            .split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok());
        let dist = dist
            .strip_prefix("Distance: ")
            .ok_or(AoCError::Parsing)?
            .split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok());
        let races = time
            .zip(dist)
            .map(|(time, dist)| Race { time, dist });

        let result: usize = races
            .map(|race| {
                (0..=race.time)
                    .map(|elapsed| {
                        let speed = elapsed;
                        speed * (race.time - elapsed)
                    })
                    .filter(|&dist| dist > race.dist)
                    .count()
            })
            .product();

        Ok(result)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let (time, dist) = self
            .0
            .split_once("\n")
            .ok_or(AoCError::Parsing)?;
        let race_time = time
            .strip_prefix("Time: ")
            .ok_or(AoCError::Parsing)?
            .chars()
            .filter_map(|c| c.to_digit(10))
            .fold(0u64, |curr, digit| curr * 10 + digit as u64);
        let race_dist = dist
            .strip_prefix("Distance: ")
            .ok_or(AoCError::Parsing)?
            .chars()
            .filter_map(|c| c.to_digit(10))
            .fold(0u64, |curr, digit| curr * 10 + digit as u64);

        let a = 1.0;
        let b = 0.0 - race_time as f64;
        let c = race_dist as f64;

        let x1 = ((0.0 - b) - (b.powf(2.0) - (4.0 * a * c)).sqrt()) / (2.0 * a);
        let x2 = ((0.0 - b) + (b.powf(2.0) - (4.0 * a * c)).sqrt()) / (2.0 * a);

        let lower_bound = x1.ceil() as u32;
        let upper_bound = x2.floor() as u32 + 1;

        Ok(upper_bound - lower_bound)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "288");
    }

    #[test]
    fn part_2() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "71503");
    }
}
