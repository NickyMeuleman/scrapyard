use std::fs;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Data {
    lines: Vec<Vec<Boi>>,
}

#[derive(Debug, Clone)]
enum Boi {
    Opening(BoiType),
    Closing(BoiType),
}

#[derive(Debug, Clone, PartialEq)]
enum BoiType {
    Curly,
    Round,
    Square,
    Pointy,
}

impl Data {
    fn part_one(self) -> usize {
        let mut scores = Vec::new();

        for line in self.lines {
            line.iter().try_fold(Vec::new(), |mut acc, boi| match boi {
                Boi::Opening(boi_type) => {
                    acc.push(boi_type);
                    Some(acc)
                }
                Boi::Closing(boi_type) => {
                    if acc.pop() == Some(boi_type) {
                        Some(acc)
                    } else {
                        // found a syntax error
                        // calculate score and push to scores vector
                        let score = match boi_type {
                            BoiType::Round => 3,
                            BoiType::Square => 57,
                            BoiType::Curly => 1197,
                            BoiType::Pointy => 25137,
                        };
                        scores.push(score);
                        None
                    }
                }
            });
        }
        scores.iter().sum()
    }

    fn part_two(&self) -> usize {
        let mut scores = Vec::new();
        for line in &self.lines {
            let mut score = 0;

            let maybe_stack = line.iter().try_fold(Vec::new(), |mut acc, boi| match boi {
                Boi::Opening(boi_type) => {
                    acc.push(boi_type);
                    Some(acc)
                }
                Boi::Closing(boi_type) => {
                    if acc.pop() == Some(boi_type) {
                        Some(acc)
                    } else {
                        // found a syntax error
                        None
                    }
                }
            });

            // if there is a stack, it contains all the remaining brackets
            if let Some(mut stack) = maybe_stack {
                while let Some(boi_type) = stack.pop() {
                    // calculate and update score
                    let boi_score = match boi_type {
                        BoiType::Round => 1,
                        BoiType::Square => 2,
                        BoiType::Curly => 3,
                        BoiType::Pointy => 4,
                    };
                    score = score * 5 + boi_score;
                }
                scores.push(score);
            }
        }

        scores.sort();
        *scores.iter().nth(scores.len() / 2).unwrap()
    }
}

impl FromStr for Data {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lines = input
            .trim()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '{' => Boi::Opening(BoiType::Curly),
                        '(' => Boi::Opening(BoiType::Round),
                        '[' => Boi::Opening(BoiType::Square),
                        '<' => Boi::Opening(BoiType::Pointy),
                        '}' => Boi::Closing(BoiType::Curly),
                        ')' => Boi::Closing(BoiType::Round),
                        ']' => Boi::Closing(BoiType::Square),
                        '>' => Boi::Closing(BoiType::Pointy),
                        _ => panic!("Invalid input detected"),
                    })
                    .collect()
            })
            .collect();
        Ok(Self { lines })
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let data: Data = input.parse().unwrap();
    println!("Part one answer: {}", data.clone().part_one());
    println!("Part two answer: {}", data.part_two());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_one_example() {
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
        let data: Data = input.parse().unwrap();
        dbg!(&data.lines[0]);
        assert_eq!(data.part_one(), 26397);
    }

    #[test]

    fn part_two_example() {
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
        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_two(), 288957);
    }
}
