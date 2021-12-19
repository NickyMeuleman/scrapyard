// https://github.com/vodik/aoc/blob/main/aoc-2021/src/day18.rs
// yay learning nom!

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res},
    sequence::{delimited, separated_pair},
    IResult,
};
use std::{convert::Infallible, str::FromStr};

#[derive(Debug)]
pub enum Expr {
    Pair(Box<Expr>, Box<Expr>),
    Value(u8),
}

#[derive(Debug, Default, Clone)]
pub struct SnailNumber(Vec<(u8, usize)>);

impl SnailNumber {
    fn parse(n: &Expr) -> Self {
        let mut sn = SnailNumber::default();

        fn push(n: &Expr, depth: usize, sn: &mut SnailNumber) {
            match n {
                Expr::Pair(a, b) => {
                    push(a, depth + 1, sn);
                    push(b, depth + 1, sn);
                }
                Expr::Value(v) => {
                    sn.0.push((*v, depth));
                }
            }
        }

        push(n, 0, &mut sn);
        sn
    }

    fn add(&mut self, other: &SnailNumber) {
        self.0.extend_from_slice(&other.0);
        self.0.iter_mut().for_each(|(_, c)| *c += 1);
    }

    fn explode(&mut self, hint: usize) -> Option<usize> {
        self.0[hint..].iter().position(|&(_, d)| d == 5).map(|pos| {
            let pos = pos + hint;

            let newhint = if pos > 0 {
                self.0[pos - 1].0 += self.0[pos].0;
                pos - 1
            } else {
                0
            };

            if pos + 2 < self.0.len() {
                self.0[pos + 2].0 += self.0[pos + 1].0;
            }

            self.0.remove(pos);
            self.0[pos].0 = 0;
            self.0[pos].1 -= 1;

            newhint
        })
    }

    fn split(&mut self, hint: usize) -> Option<usize> {
        self.0[hint..]
            .iter()
            .position(|&(v, _)| v >= 10)
            .map(|pos| {
                let pos = pos + hint;
                let cell = &mut self.0[pos];

                let left = cell.0 / 2;
                let right = cell.0 - left;
                let depth = cell.1 + 1;

                *cell = (left, depth);
                self.0.insert(pos + 1, (right, depth));

                pos
            })
    }

    fn reduce(&mut self) {
        let mut hint = 0;
        while let Some(newhint) = self.explode(hint) {
            hint = newhint;
        }

        if let Some(mut hint) = self.split(0) {
            loop {
                if let Some(newhint) = self.explode(hint) {
                    hint = newhint;
                }

                if let Some(newhint) = self.split(hint) {
                    hint = newhint
                } else {
                    break;
                }
            }
        }
    }

    fn magnitude(&self) -> u64 {
        let mut numbers: Vec<_> = self.0.iter().map(|&(v, d)| (v.into(), d)).collect();

        for depth in (1..5).rev() {
            let mut left = 0;
            while left < numbers.len() {
                if numbers[left].1 == depth {
                    let mut right = left + 1;
                    while numbers[right].1 == 0 {
                        right += 1;
                    }

                    numbers[left].0 = numbers[left].0 * 3 + numbers[right].0 * 2;
                    numbers[left].1 -= 1;
                    numbers[right].1 = 0;

                    left = right;
                }
                left += 1;
            }
        }

        numbers[0].0
    }
}

#[derive(Debug, Clone)]
pub struct Data {
    numbers: Vec<SnailNumber>,
}

impl Data {
    pub fn part_one(&self) -> u64 {
        self.numbers
            .clone()
            .into_iter()
            .reduce(|mut acc, number| {
                acc.add(&number);
                acc.reduce();
                acc
            })
            .unwrap()
            .magnitude()
    }

    pub fn part_two(&self) -> u64 {
        let numbers = self.numbers.clone();

        numbers
            .iter()
            .enumerate()
            .flat_map(|(lhs_idx, lhs)| {
                numbers
                    .iter()
                    .enumerate()
                    .filter(move |(rhs_idx, _)| lhs_idx != *rhs_idx)
                    .map(|(_, rhs)| {
                        let mut result = lhs.clone();
                        result.add(rhs);
                        result.reduce();
                        result.magnitude()
                    })
            })
            .max()
            .unwrap()
    }
}

impl FromStr for Data {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        fn pair(input: &str) -> IResult<&str, Expr> {
            delimited(
                tag("["),
                map(
                    separated_pair(snail_number, tag(","), snail_number),
                    |(left, right)| Expr::Pair(Box::new(left), Box::new(right)),
                ),
                tag("]"),
            )(input)
        }

        fn number<T: FromStr>(input: &str) -> IResult<&str, T> {
            map_res(digit1, FromStr::from_str)(input)
        }
        fn value(input: &str) -> IResult<&str, Expr> {
            map(number, Expr::Value)(input)
        }
        fn snail_number(input: &str) -> IResult<&str, Expr> {
            alt((pair, value))(input)
        }

        Ok(Self {
            numbers: input
                .trim()
                .lines()
                .filter_map(|line| snail_number(line).ok())
                .map(|(_, expr)| SnailNumber::parse(&expr))
                .collect(),
        })
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
