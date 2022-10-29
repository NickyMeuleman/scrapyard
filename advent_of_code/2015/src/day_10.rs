use itertools::Itertools;

use crate::{utils::Solution, AoCData};

pub struct Data(String);

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        Some(Self(input))
    }

    fn part_1(&self) -> String {
        (0..40)
            .fold(self.0.to_string(), |acc, _i| look_and_say(&acc))
            .len()
            .to_string()
    }

    fn part_2(&self) -> String {
        (0..50)
            .fold(self.0.to_string(), |acc, _i| look_and_say(&acc))
            .len()
            .to_string()
    }

    fn solve(self) -> crate::utils::Solution
    where
        Self: Sized,
    {
        let mut part1 = 0;

        let part2 = (0..50)
            .fold(self.0, |acc, i| {
                if i == 40 {
                    part1 = acc.len();
                }
                look_and_say(&acc)
            })
            .len();

        Solution {
            part1: part1.to_string(),
            part2: part2.to_string(),
        }
    }
}

fn look_and_say(s: &str) -> String {
    let mut result = String::new();
    let mut iter = s.chars();
    while let Some(curr) = iter.next() {
        let amount = iter.take_while_ref(|&c| c == curr).count();
        result.push_str(&(1 + amount).to_string());
        result.push(curr);
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn look_and_say_once() {
        assert_eq!(look_and_say("1"), "11");
        assert_eq!(look_and_say("11"), "21");
        assert_eq!(look_and_say("21"), "1211");
        assert_eq!(look_and_say("1211"), "111221");
        assert_eq!(look_and_say("111221"), "312211");
    }

    #[test]
    fn look_and_say_five() {
        let input = "1".to_string();
        let result = (0..5).fold(input, |acc, _i| look_and_say(&acc));
        assert_eq!(result, "312211".to_string());
    }
}
