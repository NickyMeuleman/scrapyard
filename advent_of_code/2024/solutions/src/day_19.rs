// Blog writeup with simpler Rust code (I should handle errors here):
// https://nickymeuleman.netlify.app/blog/aoc2024-day19/

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone)]
pub struct Data<'a> {
    towels: Vec<&'a str>,
    patterns: Vec<&'a str>,
}
fn ways<'a>(pattern: &'a str, towels: &[&str], cache: &mut HashMap<&'a str, u64>) -> u64 {
    if pattern.is_empty() {
        return 1;
    }
    if let Some(&num) = cache.get(&pattern) {
        return num;
    }

    let sum = towels
        .iter()
        .filter(|&towel| pattern.starts_with(towel))
        .map(|towel| ways(&pattern[towel.len()..], towels, cache))
        .sum();
    cache.insert(pattern, sum);

    sum
}
impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        let (towels, patterns) = input
            .split_once("\n\n")
            .ok_or(AoCError::Parsing)?;
        let towels = towels.split(", ").collect();
        let patterns = patterns.lines().collect();

        Ok(Self { towels, patterns })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut cache = HashMap::new();

        Ok(self
            .patterns
            .iter()
            .filter(|pattern| ways(pattern, &self.towels, &mut cache) != 0)
            .count())
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut cache = HashMap::new();

        Ok(self
            .patterns
            .iter()
            .map(|pattern| ways(pattern, &self.towels, &mut cache))
            .sum::<u64>())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "6");
    }

    #[test]
    fn part_2() {
        let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "16");
    }
}
