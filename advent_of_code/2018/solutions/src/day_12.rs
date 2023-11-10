use std::fmt::Display;

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

struct Tunnel {
    current_gen: std::vec::Vec<bool>,
    next_gen: std::vec::Vec<bool>,
    offset: usize,
    evolutions: [bool; 32],
}

impl Tunnel {
    fn parse(input_string: &str, space_for_generations: usize) -> Result<Self, AoCError> {
        let mut evolutions = [false; 32];

        let mut lines = input_string.lines();
        let next_line = lines.next().ok_or(AoCError::Parsing)?;
        let prefix_length = "initial state: ".len();

        let initial_line: &str = next_line
            .get(prefix_length..)
            .ok_or(AoCError::Parsing)?;

        let max_growth = space_for_generations * 2;
        let state_length = initial_line.len() + 2 * max_growth;
        let mut current_gen = vec![false; state_length];
        let next_gen = vec![false; state_length];
        for (i, byte) in initial_line.bytes().enumerate() {
            current_gen[max_growth + i] = byte == b'#';
        }

        lines.next(); // Skip empty line
        for line in lines {
            let (part1, part2) = line
                .split_once(" => ")
                .ok_or_else(|| AoCError::Parsing)?;
            if part2 == "#" {
                let from_bytes: Vec<u8> = part1.bytes().collect();
                if from_bytes.len() != 5 {
                    return Err(AoCError::Parsing);
                }
                let from = (usize::from(from_bytes[0] == b'#'))
                    + ((usize::from(from_bytes[1] == b'#')) << 1)
                    + ((usize::from(from_bytes[2] == b'#')) << 2)
                    + ((usize::from(from_bytes[3] == b'#')) << 3)
                    + ((usize::from(from_bytes[4] == b'#')) << 4);
                evolutions[from] = true;
            }
        }

        Ok(Self {
            current_gen,
            next_gen,
            offset: max_growth,
            evolutions,
        })
    }

    fn evolve(&mut self) {
        for i in 2..self.current_gen.len() - 2 {
            let current = usize::from(self.current_gen[i - 2])
                + (usize::from(self.current_gen[i - 1]) << 1)
                + (usize::from(self.current_gen[i]) << 2)
                + (usize::from(self.current_gen[i + 1]) << 3)
                + (usize::from(self.current_gen[i + 2]) << 4);
            self.next_gen[i] = self.evolutions[current];
        }

        std::mem::swap(&mut self.next_gen, &mut self.current_gen);
    }

    fn is_repeating(&self) -> bool {
        self.current_gen
            .iter()
            .skip_while(|&&populated| !populated)
            .zip(
                self.next_gen
                    .iter()
                    .skip_while(|&&populated| !populated),
            )
            .all(|(a, b)| a == b)
    }

    fn score(&self) -> i64 {
        let mut sum = 0;
        for (index, &value) in self.current_gen.iter().enumerate() {
            if value {
                let index = index as i32 - self.offset as i32;
                sum += i64::from(index);
            }
        }
        sum
    }
}

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let max_steps = 20;

        let mut tunnel = Tunnel::parse(self.0, max_steps)?;

        for _ in 0..20 {
            tunnel.evolve();
        }

        Ok(tunnel.score())
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let max_steps = 50;

        let mut tunnel = Tunnel::parse(self.0, max_steps)?;

        let mut previous_score = -1;
        for generation in 1.. {
            tunnel.evolve();

            let score_diff = tunnel.score() - previous_score;
            previous_score = tunnel.score();

            if tunnel.is_repeating() {
                let remaining_generations = 50_000_000_000_i64 - generation as i64;
                let final_score = tunnel.score() + remaining_generations * score_diff;
                return Ok(final_score);
            }
        }
        Err(AoCError::Solving)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "325");
    }
}
