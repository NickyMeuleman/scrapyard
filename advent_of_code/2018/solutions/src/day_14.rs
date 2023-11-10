use std::fmt::Display;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let delay: usize = self.0.trim().parse()?;

        let mut recipes = vec![3, 7];
        let mut elves = [0, 1];

        while recipes.len() < delay + 10 {
            let new: u8 = elves
                .iter()
                .map(|&elf| recipes[elf])
                .sum();
            if new >= 10 {
                recipes.push(1);
                recipes.push(new - 10);
            } else {
                recipes.push(new);
            }
            for i in &mut elves {
                *i = (*i + recipes[*i] as usize + 1) % recipes.len();
            }
        }

        let result: String = recipes
            .into_iter()
            .skip(delay)
            .take(10)
            .filter_map(|i| char::from_digit(i as u32, 10))
            .collect();

        Ok(result)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let target: Vec<u8> = self
            .0
            .trim()
            .chars()
            .filter_map(|chr| chr.to_digit(10).map(|n| n as u8))
            .collect();

        let mut recipes = vec![3, 7];
        let mut elves = [0, 1];

        loop {
            let last: Vec<_> = recipes
                .iter()
                .rev()
                .take(target.len())
                .rev()
                .copied()
                .collect();
            let second: Vec<_> = recipes
                .iter()
                .rev()
                .skip(1)
                .take(target.len())
                .rev()
                .copied()
                .collect();
            if last == target {
                return Ok((recipes.len() - target.len()) as i32);
            }
            if second == target {
                return Ok((recipes.len() - target.len() - 1) as i32);
            }
            let new: u8 = elves
                .iter()
                .map(|&elf| recipes[elf])
                .sum();
            if new >= 10 {
                recipes.push(1);
                recipes.push(new - 10);
            } else {
                recipes.push(new);
            }
            for i in &mut elves {
                *i = (*i + recipes[*i] as usize + 1) % recipes.len();
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "2018";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "5941429882");
    }

    #[test]
    fn part_2() {
        let input = "59414";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "2018");
    }
}
