use std::fmt::Display;

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const SIZE: usize = WIDTH * HEIGHT;

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        self.0
            .chars()
            .filter_map(|c| c.to_digit(10))
            .array_chunks::<SIZE>()
            .min_by(|chunk_a, chunk_b| {
                let zeroes_a = chunk_a
                    .iter()
                    .filter(|n| **n == 0)
                    .count();
                let zeroes_b = chunk_b
                    .iter()
                    .filter(|n| **n == 0)
                    .count();
                zeroes_a.cmp(&zeroes_b)
            })
            .map(|chunk| {
                let ones = chunk
                    .iter()
                    .filter(|n| **n == 1)
                    .count();
                let twos = chunk
                    .iter()
                    .filter(|n| **n == 2)
                    .count();
                ones * twos
            })
            .ok_or(AoCError::Solving)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut image = [2u32; SIZE];
        for chunk in self
            .0
            .chars()
            .filter_map(|c| c.to_digit(10))
            .array_chunks::<SIZE>()
        {
            for (idx, color) in chunk.iter().enumerate() {
                if image[idx] != 2 {
                    continue;
                }
                image[idx] = *color;
            }
        }

        let mut result = String::new();
        for height in 0..HEIGHT {
            for width in 0..WIDTH {
                let color = image[height * WIDTH + width];
                match color {
                    0 => result.push(' '),
                    1 => result.push('■'),
                    _ => return Err(AoCError::Solving),
                }
            }
            result.push('\n');
        }

        Ok(result)
    }
}
