use crate::AoCData;

fn closing(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!("invalid input"),
    }
}

pub struct Data {
    lines: Vec<Vec<char>>,
}

impl AoCData for Data {
    fn new(input: String) -> Self {
        // TODO: figure out how to pass a &'a str to new() so I can use Vec<&'a str> instead of Vec<Vec<char>> in Data
        Self {
            lines: input
                .trim()
                .lines()
                .map(|line| line.chars().collect())
                .collect(),
        }
    }

    fn part_1(&self) -> String {
        let result: u32 = self
            .lines
            .iter()
            .filter_map(|line| {
                let mut stack = Vec::new();
                line.iter().find_map(|&c| {
                    match c {
                        '(' | '[' | '{' | '<' => stack.push(c),
                        ')' | ']' | '}' | '>' => {
                            // if the stack was empty or if the current brace isn't the correct one, found error
                            if stack.is_empty() || c != closing(stack.pop().unwrap()) {
                                let score = match c {
                                    ')' => 3,
                                    ']' => 57,
                                    '}' => 1197,
                                    '>' => 25137,
                                    _ => unreachable!("invalid input"),
                                };
                                return Some(score);
                            }
                        }
                        _ => unreachable!("invalid input"),
                    };
                    None
                })
            })
            .sum();

        result.to_string()
    }

    fn part_2(&self) -> String {
        let mut scores: Vec<u64> = self
            .lines
            .iter()
            // The lines without an error, and with a stack that has items are unfinished, keep those
            .filter_map(|line| {
                // try to build up a stack for a line, fail if the line has an error
                line.iter().try_fold(Vec::new(), |mut stack, &c| {
                    match c {
                        '(' | '[' | '{' | '<' => stack.push(c),
                        ')' | ']' | '}' | '>' => {
                            // if the stack was empty or if the current brace isn't the correct one, found error
                            if stack.is_empty() || c != closing(stack.pop().unwrap()) {
                                return None;
                            }
                        }
                        _ => unreachable!("invalid input"),
                    };
                    Some(stack)
                })
            })
            // turn the stacks into scores by iterating over it in reverse, completing the line and adding the score
            .map(|mut stack| {
                stack.reverse();
                stack.into_iter().fold(0, |acc, c| {
                    let score = match closing(c) {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => unreachable!("invalid input"),
                    };

                    (acc * 5) + score
                })
            })
            .collect();

        // the amount of scores is guaranteed to be odd, the middle element is the one we want
        scores.sort_unstable();
        let result = scores[scores.len() / 2];

        result.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(10);
        let data = Data::new(input);
        assert_eq!(data.part_1(), "26397");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(10);
        let data = Data::new(input);
        assert_eq!(data.part_2(), "288957");
    }
}
