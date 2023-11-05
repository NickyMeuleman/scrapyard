use std::fmt::Display;

use crate::{AoCData, AoCResult};

use aoc_core::Solution;
use md5::{Digest, Md5};

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut hasher = Md5::new();
        let mut index: i32 = 0;
        let key = self.0.as_bytes();
        let mut password = String::new();

        while password.len() < 8 {
            hasher.update(key);
            hasher.update(index.to_string().as_bytes());
            let result = hasher.finalize_reset();
            // an item in the result array is a byte represented by 2 hex characters: 00 to FF
            // example hex pair: FF
            // that byte as bits: 11111111
            // left half (first four bits): 1111
            // right half (last four bits): 1111
            // 0xFF & 0xF0 gives 11110000
            // 0xFF & 0x0F gives 0000FFFF
            // check if both hex characters at idx 0 are 0
            // check if both hex characters at idx 1 are 0
            // check if first hex character at idx 2 is 0
            let one_two = result[0];
            let three_four = result[1];
            let five = result[2] & 0xf0; // flips last four bits of this pair of hex characters to 0, only keeping the ones of the first hex character
            if (one_two == 0) && (three_four == 0) && (five == 0) {
                let six = result[2] & 0x0f; // flips first four bits of this pair of hex characters to 0, only keeping the ones of the last hex character
                if let Some(c) = char::from_digit(six as u32, 16) {
                    password.push(c);
                }
            }
            index += 1;
        }

        Ok(password)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut hasher = Md5::new();
        let mut index: i32 = 0;
        let key = self.0.as_bytes();
        let mut password = [None; 8];

        while password
            .iter()
            .any(|item| item.is_none())
        {
            hasher.update(key);
            hasher.update(index.to_string().as_bytes());
            let result = hasher.finalize_reset();
            let one_two = result[0];
            let three_four = result[1];
            let five = result[2] & 0xf0;
            let six = result[2] & 0x0f;
            if (one_two == 0)
                && (three_four == 0)
                && (five == 0)
                && six <= 7
                && password[six as usize].is_none()
            {
                // cannot only do result[3] & 0xf0 here because that would zero out the last 4 bits bit not shift the intact ones over
                // result[3] >> 4 is functionally the same as (result[3] & 0xf0) >> 4
                let seven = result[3] >> 4;
                if let Some(c) = char::from_digit(seven as u32, 16) {
                    password[six as usize] = Some(c);
                }
            }
            index += 1;
        }

        Ok(password
            .iter()
            .map(|item| item.unwrap())
            .collect::<String>())
    }

    fn solve(self) -> AoCResult<Solution>
    where
        Self: Sized,
    {
        let mut hasher = Md5::new();
        let mut index: i32 = 0;
        let key = self.0.as_bytes();
        let mut password1 = String::new();
        let mut password2 = [None; 8];

        while password1.len() < 8
            || password2
                .iter()
                .any(|item| item.is_none())
        {
            hasher.update(key);
            hasher.update(index.to_string().as_bytes());
            let result = hasher.finalize_reset();
            let one_two = result[0];
            let three_four = result[1];
            let five = result[2] & 0xf0;
            if (one_two == 0) && (three_four == 0) && (five == 0) {
                let six = result[2] & 0x0f;
                if password1.len() < 8 {
                    if let Some(c) = char::from_digit(six as u32, 16) {
                        password1.push(c);
                    }
                }
                if six <= 7 && password2[six as usize].is_none() {
                    let seven = result[3] >> 4;
                    if let Some(c) = char::from_digit(seven as u32, 16) {
                        password2[six as usize] = Some(c);
                    }
                }
            }
            index += 1;
        }

        let password2: String = password2
            .iter()
            .map(|item| item.unwrap())
            .collect();

        Ok(Solution {
            part1: Box::new(password1),
            part2: Box::new(password2),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "abc";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "18f47a30");
    }

    #[test]
    fn part_2() {
        let input = "abc";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "05ace8e3");
    }

    #[test]
    fn solve() {
        let input = "abc";
        let data = Data::try_new(input).unwrap();
        let Solution { part1, part2 } = data.solve().unwrap();
        assert_eq!(part1.to_string(), "18f47a30");
        assert_eq!(part2.to_string(), "05ace8e3");
    }
}
