use std::fmt::Display;

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(i64, i64);

struct Generator {
    num: i64,
    factor: i64,
}

impl Iterator for Generator {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        self.num = (self.num * self.factor) % 2_147_483_647;
        Some(self.num)
    }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut lines = input.lines();
        let start_a = lines
            .next()
            .ok_or(AoCError::Parsing)?
            .split_whitespace()
            .nth(4)
            .ok_or(AoCError::Parsing)?
            .parse()?;
        let start_b = lines
            .next()
            .ok_or(AoCError::Parsing)?
            .split_whitespace()
            .nth(4)
            .ok_or(AoCError::Parsing)?
            .parse()?;
        Ok(Self(start_a, start_b))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let gen_a = Generator {
            num: self.0,
            factor: 16807,
        };
        let gen_b = Generator {
            num: self.1,
            factor: 48271,
        };

        let result = gen_a
            .zip(gen_b)
            .take(40_000_000)
            .map(|(a, b)| (a & 0xffff, b & 0xffff))
            .filter(|(a, b)| a == b)
            .count();

        Ok(result)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let gen_a = Generator {
            num: self.0,
            factor: 16807,
        }
        .filter(|n| n % 4 == 0);
        let gen_b = Generator {
            num: self.1,
            factor: 48271,
        }
        .filter(|n| n % 8 == 0);

        let result = gen_a
            .zip(gen_b)
            .take(5_000_000)
            .map(|(a, b)| (a & 0xffff, b & 0xffff))
            .filter(|(a, b)| a == b)
            .count();

        Ok(result)
    }
}
