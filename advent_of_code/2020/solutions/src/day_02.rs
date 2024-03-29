use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::Display,
};

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data {
    password_lines: Vec<PasswordLine>,
}

#[derive(Debug, Clone)]
struct Policy {
    letter: char,
    lower: i32,
    upper: i32,
}

#[derive(Debug, Clone)]
struct PasswordLine {
    policy: Policy,
    password: String,
}

enum PolicyType {
    Old,
    New,
}

impl PasswordLine {
    fn is_valid(&self, policy_age: PolicyType) -> bool {
        match policy_age {
            PolicyType::Old => {
                let mut letter_map: HashMap<char, i32> = HashMap::new();
                for character in self.password.chars() {
                    let count = letter_map.entry(character).or_default();
                    *count += 1
                }
                match letter_map.entry(self.policy.letter) {
                    Entry::Occupied(entry) => {
                        let number = entry.get();
                        if number >= &self.policy.lower && number <= &self.policy.upper {
                            return true;
                        } else {
                            return false;
                        }
                    }
                    Entry::Vacant(_) => return false,
                }
            }
            PolicyType::New => {
                let char_at_lower = self
                    .password
                    .chars()
                    .nth((self.policy.lower - 1) as usize)
                    .unwrap();
                let char_at_upper = self
                    .password
                    .chars()
                    .nth((self.policy.upper - 1) as usize)
                    .unwrap();
                let mut upper_match = false;
                let mut lower_match = false;
                if char_at_upper == self.policy.letter {
                    upper_match = true
                }
                if char_at_lower == self.policy.letter {
                    lower_match = true
                }
                // ^ is the XOR operator
                return upper_match ^ lower_match;
            }
        }
    }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut password_lines = Vec::new();
        for line in input.lines() {
            // TODO: check if only 2 parts after split, or only split on first :
            // so, regex is a thing, might be useful here
            let parts: Vec<&str> = line.split(": ").collect();
            let password = parts[1];
            if let [bounds, letter] = &parts[0]
                .split(" ")
                .collect::<Vec<&str>>()[..]
            {
                let letter: char = letter.parse().unwrap();
                if let [lower, upper] = bounds.split("-").collect::<Vec<&str>>()[..] {
                    let lower: i32 = lower.parse().unwrap();
                    let upper: i32 = upper.parse().unwrap();
                    let password_line = PasswordLine {
                        password: String::from(password),
                        policy: Policy {
                            letter,
                            lower,
                            upper,
                        },
                    };
                    password_lines.push(password_line);
                }
            }
        }
        Ok(Self { password_lines })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut valid_count = 0;
        for password in &self.password_lines {
            if password.is_valid(PolicyType::Old) {
                valid_count += 1;
            }
        }
        Ok(valid_count)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut valid_count = 0;
        for password in &self.password_lines {
            if password.is_valid(PolicyType::New) {
                valid_count += 1;
            }
        }
        Ok(valid_count)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "2");
    }

    #[test]
    fn part_2() {
        let input = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "1");
    }
}
