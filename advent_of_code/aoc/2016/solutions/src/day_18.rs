use std::{fmt::Display, iter};

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<bool>);

fn helper(mut row: Vec<bool>, total: u32) -> usize {
    let mut count = row.iter().filter(|&trap| !trap).count();

    for _ in 1..total {
        let padded: Vec<bool> = iter::once(false)
            .chain(row.into_iter())
            .chain(iter::once(false))
            .collect();
        let new_row: Vec<bool> = padded
            .windows(3)
            .map(|window| window[0] != window[2])
            .collect();
        count += new_row
            .iter()
            .filter(|&trap| !trap)
            .count();
        row = new_row;
    }

    count
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut row = Vec::new();
        for c in input.trim().chars() {
            let trap = match c {
                '.' => false,
                '^' => true,
                _ => return Err(AoCError::Parsing),
            };
            row.push(trap);
        }
        Ok(Self(row))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        Ok(helper(self.0.clone(), 40))
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        Ok(helper(self.0.clone(), 400_000))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = ".^^.^.^^^^";
        let data = Data::try_new(input).unwrap();
        assert_eq!(helper(data.0, 10), 38);
    }

    #[test]
    fn part_2() {
        let input = "^.....^.^^^^^.^..^^.^.......^^..^^^..^^^^..^.^^.^.^....^^...^^.^^.^...^^.^^^^..^^.....^.^...^.^.^^.^";
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2().unwrap().to_string(), "19991126");
    }
}
