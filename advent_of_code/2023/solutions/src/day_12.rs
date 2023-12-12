use std::fmt::Display;

use aoc_core::AoCError;
use itertools::Itertools;

use crate::{AoCData, AoCResult};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Spring {
    Unknown,
    Damaged,
    Operational,
}

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

// https://github.com/Crazytieguy/advent-of-code/blob/c75e0008119a1409fc4e99c302da38804ea96bf8/2023/src/bin/day12/main.rs
fn count_possible_arangements(mut springs: Vec<Spring>, counts: Vec<usize>) -> u64 {
    // to make the Damaged recursion case simpler
    springs.push(Spring::Operational);
    let mut cache = vec![vec![None; springs.len()]; counts.len()];
    count_possible_arangements_inner(&springs, &counts, &mut cache)
}

fn count_possible_arangements_inner(
    springs: &[Spring],
    counts: &[usize],
    cache: &mut [Vec<Option<u64>>],
) -> u64 {
    if counts.is_empty() {
        return if springs.contains(&Spring::Damaged) {
            // Too many previous unknowns were counted as damaged
            0
        } else {
            // All remaining unknowns are operational
            1
        };
    }
    if springs.len() < counts.iter().sum::<usize>() + counts.len() {
        // Not enough space for remaining numbers
        return 0;
    }
    if let Some(cached) = cache[counts.len() - 1][springs.len() - 1] {
        return cached;
    }
    let mut arangements = 0;
    if springs[0] != Spring::Damaged {
        // Assume operational
        arangements += count_possible_arangements_inner(&springs[1..], counts, cache);
    }
    let next_group_size = counts[0];
    if !springs[..next_group_size].contains(&Spring::Operational)
        && springs[next_group_size] != Spring::Damaged
    {
        // Assume damaged
        arangements +=
            count_possible_arangements_inner(&springs[next_group_size + 1..], &counts[1..], cache);
    }
    cache[counts.len() - 1][springs.len() - 1] = Some(arangements);
    arangements
}

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        self.0
            .lines()
            .map(|line| {
                let (springs, counts) = line
                    .split_once(' ')
                    .ok_or(AoCError::Parsing)?;
                let springs = springs
                    .chars()
                    .map(|c| match c {
                        '.' => Ok(Spring::Operational),
                        '#' => Ok(Spring::Damaged),
                        '?' => Ok(Spring::Unknown),
                        _ => return Err(AoCError::Parsing),
                    })
                    .collect::<AoCResult<Vec<Spring>>>()?;
                let counts: Vec<usize> = counts
                    .split(',')
                    .filter_map(|s| s.parse().ok())
                    .collect();

                Ok((springs, counts))
            })
            .map_ok(|(springs, counts)| count_possible_arangements(springs, counts))
            .sum::<AoCResult<u64>>()
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        self.0
            .lines()
            .map(|line| {
                let (springs, counts) = line
                    .split_once(' ')
                    .ok_or(AoCError::Parsing)?;
                let springs = springs
                    .chars()
                    .map(|c| match c {
                        '.' => Ok(Spring::Operational),
                        '#' => Ok(Spring::Damaged),
                        '?' => Ok(Spring::Unknown),
                        _ => return Err(AoCError::Parsing),
                    })
                    .collect::<AoCResult<Vec<Spring>>>()?;
                let counts: Vec<usize> = counts
                    .split(',')
                    .filter_map(|s| s.parse().ok())
                    .collect();

                Ok((springs, counts))
            })
            .map_ok(|(mut springs, mut counts)| {
                springs = springs
                    .iter()
                    .copied()
                    .chain([Spring::Unknown])
                    .cycle()
                    .take(springs.len() * 5 + 4)
                    .collect();
                counts = counts
                    .iter()
                    .copied()
                    .cycle()
                    .take(counts.len() * 5)
                    .collect();

                count_possible_arangements(springs, counts)
            })
            .sum::<AoCResult<u64>>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "21");
    }

    #[test]
    fn part_2() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "525152");
    }
}
