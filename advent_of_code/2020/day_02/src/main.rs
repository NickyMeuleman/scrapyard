use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::error;
use std::fs;

fn main() -> Result<(), Box<dyn error::Error>> {
    let input = fs::read_to_string("./input.txt")?;
    let password_lines = parse_input(input).unwrap();
    println!("part one answer: {}", part_one(&password_lines));
    println!("part two answer: {}", part_two(&password_lines));
    Ok(())
}

fn part_one(password_lines: &Vec<PasswordLine>) -> i32 {
    let mut valid_count = 0;
    for password in password_lines {
        if password.is_valid(PolicyType::Old) {
            valid_count += 1;
        }
    }
    valid_count
}

fn part_two(password_lines: &Vec<PasswordLine>) -> i32 {
    let mut valid_count = 0;
    for password in password_lines {
        if password.is_valid(PolicyType::New) {
            valid_count += 1;
        }
    }
    valid_count
}

#[derive(Debug)]
struct Policy {
    letter: char,
    lower: i32,
    upper: i32,
}

#[derive(Debug)]
struct PasswordLine {
    policy: Policy,
    password: String,
}

enum PolicyType {
    Old,
    New
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
            },
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

fn parse_input(input: String) -> Result<Vec<PasswordLine>, ()> {
    let mut password_lines = Vec::new();
    for line in input.lines() {
        // TODO: check if only 2 parts after split, or only split on first :
        // so, regex is a thing, might be useful here
        let parts: Vec<&str> = line.split(": ").collect();
        let password = parts[1];
        if let [bounds, letter] = &parts[0].split(" ").collect::<Vec<&str>>()[..] {
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
    Ok(password_lines)
}
