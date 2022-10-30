use crate::{utils::Solution, AoCData};

use itertools::Itertools;
use std::collections::HashSet;

pub struct Data(String);

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        Some(Self(input))
    }

    fn part_1(&self) -> String {
        let mut pass = self.0.clone();
        pass = increment(&pass);
        next_valid(pass)
    }

    fn part_2(&self) -> String {
        let mut pass = self.0.clone();
        pass = increment(&pass);
        pass = next_valid(pass);
        pass = increment(&pass);
        next_valid(pass)
    }

    fn solve(self) -> crate::utils::Solution
    where
        Self: Sized,
    {
        let mut pass1 = self.0;
        pass1 = increment(&pass1);
        pass1 = next_valid(pass1);
        let mut pass2 = increment(&pass1);
        pass2 = next_valid(pass2);

        Solution { part1: pass1, part2: pass2 }
    }
}

fn is_valid(s: &str) -> bool {
    if !s
        .chars()
        .tuple_windows()
        .any(|(c1, c2, c3)| c1 as u8 + 1 == c2 as u8 && c2 as u8 + 1 == c3 as u8)
    {
        return false;
    }
    if s.chars().any(|c| c == 'i' || c == 'o' || c == 'l') {
        return false;
    }
    let pairs = s
        .chars()
        .tuple_windows()
        .fold(HashSet::new(), |mut acc, (c1, c2)| {
            if c1 == c2 {
                acc.insert(c1);
            }
            acc
        });
    if pairs.len() < 2 {
        return false;
    }
    true
}

fn increment(s: &str) -> String {
    let chars: Vec<char> = s.chars().rev().collect();
    let mut new_password = String::new();

    let mut needs_change = true;
    for c in chars {
        let mut val = c as u8;

        if needs_change {
            val += 1;
            needs_change = false;
        }

        if val > b'z' {
            val = b'a';
            needs_change = true;
        }

        new_password.push(val as char);
    }

    new_password.chars().rev().collect()
}

fn next_valid(mut pass: String) -> String {
    pass = increment(&pass);
    while !is_valid(&pass) {
        pass = increment(&pass);
    }
    pass
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn valid() {
        assert!(!is_valid("hijklmmn"));
        assert!(!is_valid("abbceffg"));
        assert!(!is_valid("abbcegjk"));
        assert!(is_valid("cqjxxyzz"));
    }
    #[test]
    fn wrap() {
        assert_eq!(next_valid("abcdefgh".to_string()), "abcdffaa");
        assert_eq!(next_valid("ghijklmn".to_string()), "ghjaabcc");
    }
}
