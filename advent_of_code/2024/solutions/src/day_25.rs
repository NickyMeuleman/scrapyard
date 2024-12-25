// Blog writeup with simpler Rust code (I should handle errors here):
// https://nickymeuleman.netlify.app/blog/aoc2024-day25/

use crate::{AoCData, AoCResult};
use std::{collections::HashSet, fmt::Display};

#[derive(Debug, Clone)]
pub struct Data {
    locks: HashSet<[u8; 5]>,
    keys: HashSet<[u8; 5]>,
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut locks = HashSet::new();
        let mut keys = HashSet::new();
        for block in input.split("\n\n") {
            let set = if block.starts_with('.') {
                &mut keys
            } else {
                &mut locks
            };
            let mut heights = [0; 5];
            for line in block.lines() {
                for (idx, c) in line.chars().enumerate() {
                    if c == '#' {
                        heights[idx] += 1;
                    }
                }
            }
            set.insert(heights);
        }
        Ok(Self { locks, keys })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut sum = 0;
        for lock in &self.locks {
            for key in &self.keys {
                if lock
                    .iter()
                    .zip(key)
                    .all(|(l, k)| l + k <= 7)
                {
                    sum += 1;
                }
            }
        }
        Ok(sum)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        Ok("Happy Holidays!")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "3");
    }
}
