use std::collections::VecDeque;
use std::iter;
use std::ops::Add;
use std::{convert::Infallible, str::FromStr};

// modified from:
// https://github.com/ephemient/aoc2021/blob/main/rs/src/day18.rs

#[derive(Debug, Clone)]
pub struct Data {
    // numbers is a vector of lines
    // each line is a vecdeque of tokens
    numbers: Vec<Tokens>,
}

#[derive(Debug, Clone)]
struct Tokens {
    tokens: VecDeque<SnailfishToken>,
}

#[derive(Debug, Clone, Copy)]
enum SnailfishToken {
    Open,
    Close,
    Value(u32),
}

impl Tokens {
    fn magnitude(&mut self) -> u32 {
        match self.tokens.pop_front().unwrap() {
            SnailfishToken::Open => {
                // not the end of a snailfish number yet, we need to go deeper -Leonardo DiCaprio squinting-

                // keep recursing until you get the magnitude of a number, the left number
                let lhs = self.magnitude();
                // keep recursing until you get the magnitude of a second number, the right number,
                let rhs = self.magnitude();

                // check if there is still something left in tokens
                match self.tokens.pop_front().unwrap() {
                    // at this point, the next token is a closing token, we are at the end of a snailfish number, return the magnitude
                    SnailfishToken::Close => 3 * lhs + 2 * rhs,
                    _ => unreachable!("Expected snailfish number to end, but it didn't"),
                }
            }
            // the magnitude of a regular number is just that number
            SnailfishToken::Value(num) => num,
            // a closing enum as first value? Nope
            SnailfishToken::Close => unreachable!("Found closing bracket as first value"),
        }
    }
}

impl Add for Tokens {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        // TODO: comment this
        let mut vec: Vec<SnailfishToken> = iter::once(SnailfishToken::Open)
            .chain(self.tokens)
            .chain(other.tokens)
            .chain(iter::once(SnailfishToken::Close))
            .collect();

        'outer: loop {
            let mut depth = 0;
            for (i, window) in vec[..].windows(4).enumerate() {
                // do we need to go boom?
                if depth > 3 {
                    // watch me explooooooooooooooooooooooode
                    if let [SnailfishToken::Open, SnailfishToken::Value(x), SnailfishToken::Value(y), SnailfishToken::Close] =
                        *window
                    {
                        vec.splice(i..i + 4, iter::once(SnailfishToken::Value(0)));
                        if let Some(t) = vec.iter_mut().take(i).rev().find_map(|t| match t {
                            SnailfishToken::Value(t) => Some(t),
                            _ => None,
                        }) {
                            *t += x;
                        }
                        if let Some(t) = vec.iter_mut().skip(i + 1).find_map(|t| match t {
                            SnailfishToken::Value(t) => Some(t),
                            _ => None,
                        }) {
                            *t += y;
                        }
                        // During reduction, at most one action applies, after which the process returns to the top of the list of actions.
                        // this exploded, jump to the next iteration of the outer infinite loop
                        continue 'outer;
                    }
                }
                match window[0] {
                    SnailfishToken::Open => depth += 1,
                    SnailfishToken::Close => depth -= 1,
                    _ => {}
                }
            }
            // do we need to split?
            if let Some((i, t)) = vec.iter_mut().enumerate().find_map(|(i, t)| match t {
                SnailfishToken::Value(t) => Some((i, *t)).filter(|(_, t)| *t > 9),
                _ => None,
            }) {
                vec.splice(
                    i..=i,
                    [
                        SnailfishToken::Open,
                        SnailfishToken::Value(t / 2),
                        SnailfishToken::Value((t + 1) / 2),
                        SnailfishToken::Close,
                    ],
                );
                // During reduction, at most one action applies, after which the process returns to the top of the list of actions.
                // this split, jump to the next iteration of the outer infinite loop
                // because it's the end of this codeblock, an explicit continue statement isn't needed, but it can be present
                continue 'outer;
            } else {
                // done, return the result
                break Tokens {
                    // vec isn't a vecdeque from the start because that doesn't have a windows method for some reason
                    tokens: vec.into_iter().collect(),
                };
            }
        }
    }
}

impl Data {
    pub fn part_one(&self) -> u32 {
        self.numbers
            .clone()
            .into_iter()
            .reduce(|acc, number| acc + number)
            .unwrap()
            .magnitude()
    }

    pub fn part_two(&self) -> u32 {
        let numbers = self.numbers.clone();

        numbers
            .iter()
            .enumerate()
            .flat_map(|(lhs_idx, lhs)| {
                numbers
                    .iter()
                    .enumerate()
                    .filter(move |(rhs_idx, _)| lhs_idx != *rhs_idx)
                    .map(|(_, rhs)| (lhs.clone() + rhs.clone()).magnitude())
            })
            .max()
            .unwrap()
    }
}

impl FromStr for Data {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let numbers = input
            .trim()
            .lines()
            .map(|line| Tokens {
                tokens: line
                    .chars()
                    .filter_map(|c| match c {
                        '[' => Some(SnailfishToken::Open),
                        ']' => Some(SnailfishToken::Close),
                        _ => c.to_digit(10).map(SnailfishToken::Value),
                    })
                    .collect(),
            })
            .collect();

        Ok(Self { numbers })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one_1() {
        let input = "[[1,2],[[3,4],5]]";
        let data: Data = input.parse().unwrap();
        dbg!(&data);
        assert_eq!(data.part_one(), 143);
    }

    #[test]
    fn part_one_2() {
        let input = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";
        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_one(), 1384);
    }

    #[test]
    fn part_one_3() {
        let input = "[[[[1,1],[2,2]],[3,3]],[4,4]]";
        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_one(), 445);
    }

    #[test]
    fn part_one_4() {
        let input = "[[[[3,0],[5,3]],[4,4]],[5,5]]";
        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_one(), 791);
    }

    #[test]
    fn part_one_5() {
        let input = "[[[[5,0],[7,4]],[5,5]],[6,6]]";
        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_one(), 1137);
    }

    #[test]
    fn part_one_6() {
        let input = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]";
        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_one(), 3488);
    }

    #[test]
    fn part_one_example() {
        let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
";
        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_one(), 4140);
    }

    #[test]
    fn part_two_example() {
        let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_two(), 3993);
    }
}
