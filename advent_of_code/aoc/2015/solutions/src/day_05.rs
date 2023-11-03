use std::fmt::Display;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let vowels = &['a', 'e', 'i', 'o', 'u'];
        let naughty_patterns = &["ab", "cd", "pq", "xy"];

        let result = self
            .0
            .lines()
            // contains at least three vowels
            .filter(|line| {
                line.chars()
                    .filter(|c| vowels.contains(c))
                    .count()
                    >= 3
            })
            // contains a letter that appears twice in a row
            .filter(|line| {
                line.as_bytes()
                    .windows(2)
                    .any(|window| window[0] == window[1])
            })
            // does not include a naughty pattern
            .filter(|line| {
                !naughty_patterns
                    .iter()
                    .any(|pat| line.contains(pat))
            })
            .count();

        Ok(result)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let result = self
            .0
            .lines()
            // contains a pair of any two letters that appears at least twice in the string without overlapping
            .filter(|line| {
                line.as_bytes()
                    .windows(2)
                    .enumerate()
                    .any(|(i, pair)| {
                        if let Some(idx) = line.rfind(std::str::from_utf8(pair).unwrap()) {
                            idx > i + 1
                        } else {
                            false
                        }
                    })
            })
            // contains at least one letter which repeats with exactly one letter between the
            .filter(|line| {
                line.as_bytes()
                    .windows(3)
                    .any(|window| window[0] == window[2])
            })
            .count();

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "ugknbfddgicrmopn
aaa
jchzalrnumimnmhp
haegwjzuvuyypxyu
dvszwmarrgswjxmb";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "2");
    }

    #[test]
    fn part_2() {
        let input = "qjhvhtzxzqqjkmpb
xxyxx
uurcxstgmygtbstg
ieodomkazucvgmuy";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "2");
    }
}
