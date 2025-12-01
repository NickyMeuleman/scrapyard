// Blog writeup with simpler Rust code (I should handle errors here):
// https://nickymeuleman.netlify.app/blog/aoc2025-day01/

use crate::{AoCData, AoCResult};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut pos: i32 = 50;
        let mut sum = 0;
        for line in self.0.lines() {
            let dir = if line.starts_with('R') { 1 } else { -1 };
            let num: i32 = line[1..].parse()?;
            pos = (pos + dir * num).rem_euclid(100);
            if pos == 0 {
                sum += 1
            }
        }
        Ok(sum)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut pos: i32 = 50;
        let mut sum = 0;
        for line in self.0.lines() {
            let dir = if line.starts_with('R') { 1 } else { -1 };
            let num: i32 = line[1..].parse()?;
            for _ in 0..num {
                pos = (pos + dir).rem_euclid(100);
                if pos == 0 {
                    sum += 1
                }
            }
        }
        Ok(sum)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "3");
    }

    #[test]
    fn part_2() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "6");
    }
}
