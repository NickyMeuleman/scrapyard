use std::fmt::Display;

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data {
    lines: Vec<Vec<char>>,
}

fn closing(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!("invalid input"),
    }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let lines = input
            .trim()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>' => Some(c),
                        _ => None,
                    })
                    .collect::<Option<Vec<char>>>()
            })
            .collect::<Option<Vec<Vec<char>>>>()
            .ok_or(AoCError::Parsing)?;

        Ok(Self { lines })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
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

        Ok(result)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut scores: Vec<u64> = self
            .lines
            .iter()
            // The lines without an error, and with a stack that has items are unfinished, keep those
            .filter_map(|line| {
                // try to build up a stack for a line, fail if the line has an error
                line.iter()
                    .try_fold(Vec::new(), |mut stack, &c| {
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

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "26397");
    }

    #[test]
    fn part_2() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "288957");
    }
}
