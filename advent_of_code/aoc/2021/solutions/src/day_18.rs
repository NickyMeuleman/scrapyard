use std::{fmt::Display, ops::Add};

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data {
    nums: Vec<SnailFishNum>,
}

#[derive(Debug, Clone)]
struct SnailFishNum {
    tokens: Vec<Token>,
}

#[derive(Debug, Clone)]
struct Token {
    val: u32,
    depth: u32,
}

impl SnailFishNum {
    fn parse(s: &str) -> Option<Self> {
        let mut num = SnailFishNum { tokens: Vec::new() };
        let mut depth = 0;

        for c in s.chars() {
            match c {
                '[' => {
                    depth += 1;
                }
                ',' => (),
                ']' => {
                    depth -= 1;
                }
                d => {
                    let val = d.to_digit(10)?;
                    let depth = depth - 1;
                    let token = Token { val, depth };
                    num.tokens.push(token);
                }
            }
        }

        Some(num)
    }

    fn explode(&mut self) -> bool {
        // explode if a depth 4 token is found
        if let Some(i) = (0..self.tokens.len()).find(|&i| self.tokens[i].depth == 4) {
            // add left value to left neighbour
            if i != 0 {
                self.tokens[i - 1].val += self.tokens[i].val;
            }

            // add right value to right neighbour
            if i + 2 < self.tokens.len() {
                self.tokens[i + 2].val += self.tokens[i + 1].val;
            }

            // replace left side of the exploded pair by val 0 at depth 3
            self.tokens[i] = Token { val: 0, depth: 3 };
            // remove the right side of the exploded pair
            self.tokens.remove(i + 1);

            // exploded
            return true;
        }
        // did not explode
        false
    }

    fn split(&mut self) -> bool {
        // split the first token with a value of 10 or greater
        if let Some(i) = (0..self.tokens.len()).find(|&i| self.tokens[i].val >= 10) {
            // replace the found value by a pair
            let Token { val, depth } = self.tokens[i];
            // the created left value is the found value divided by 2 and rounded down
            // the created right value is the found value divided by 2 and rounded up
            let (left_val, right_val) = if val % 2 == 0 {
                (val / 2, val / 2)
            } else {
                (val / 2, val / 2 + 1)
            };

            // a pair is inserted, so the depth increases by 1
            let left = Token {
                val: left_val,
                depth: depth + 1,
            };
            let right = Token {
                val: right_val,
                depth: depth + 1,
            };
            // replace the found index with the new left token
            self.tokens[i] = left;
            // insert the new right token and shift every remaining token to the right
            self.tokens.insert(i + 1, right);

            // split
            return true;
        }
        // did not split
        false
    }

    fn reduce(&mut self) {
        // From question:
        // During reduction, at most one action applies, after which the process returns to the top of the list of actions.
        // For example, if split produces a pair that meets the explode criteria, that pair explodes before other splits occur.
        loop {
            if !self.explode() && !self.split() {
                break;
            }
        }
    }

    fn concat(&mut self, other: &mut Self) {
        // from question:
        // To add two snailfish numbers, form a pair from the left and right parameters of the addition operator. For example, [1,2] + [[3,4],5] becomes [[1,2],[[3,4],5]].
        self.tokens.append(&mut other.tokens);
        // after appending the rhs to the lhs, the depth of every token is incremented
        for token in self.tokens.iter_mut() {
            token.depth += 1;
        }
    }

    fn magnitude(&self) -> u32 {
        let mut tokens = self.tokens.clone();

        while tokens.len() > 1 {
            for i in 0..tokens.len() - 1 {
                if tokens[i].depth == tokens[i + 1].depth {
                    tokens[i].val = 3 * tokens[i].val + 2 * tokens[i + 1].val;
                    tokens.remove(i + 1);

                    if tokens[i].depth > 0 {
                        tokens[i].depth -= 1;
                    }

                    break;
                }
            }
        }

        tokens[0].val
    }
}

impl Add for SnailFishNum {
    // From question:
    // There's only one problem: snailfish numbers must always be reduced,
    // and the process of adding two snailfish numbers can result in snailfish numbers that need to be reduced
    type Output = Self;

    fn add(mut self, mut other: Self) -> Self {
        self.concat(&mut other);
        self.reduce();
        self
    }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let nums = input
            .trim()
            .lines()
            .map(SnailFishNum::parse)
            .collect::<Option<Vec<_>>>()
            .ok_or(AoCError::Parsing)?;

        Ok(Self { nums })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let result = self
            .nums
            .clone()
            .into_iter()
            .reduce(|acc, number| acc + number)
            .unwrap()
            .magnitude();

        Ok(result)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let numbers = self.nums.clone();
        let result = numbers
            .iter()
            .enumerate()
            .flat_map(|(lhs_idx, lhs)| {
                numbers
                    .iter()
                    .enumerate()
                    .filter(move |(rhs_idx, _)| lhs_idx != *rhs_idx)
                    .map(|(_, rhs)| (lhs.clone() + rhs.clone()).magnitude())
            })
            // writing a fold because .max() gives a rust analyzer error
            .fold(0, |acc, num| acc.max(num));

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
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
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "4140");
    }

    #[test]
    fn part_2() {
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
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "3993");
    }
}
