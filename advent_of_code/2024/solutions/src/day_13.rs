use crate::{AoCData, AoCResult};
use itertools::Itertools;
use std::fmt::Display;

const TEN_TRILLY: i64 = 10_000_000_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

fn solve(a: &Point, b: &Point, prize: &Point) -> Option<i64> {
    let na = ((prize.x * b.y) - (prize.y * b.x)) / ((a.x * b.y) - (a.y * b.x));
    let nb = (prize.x - na * a.x) / b.x;
    // the two linear equations could be parallell or overlap, check if a single solution exists
    let solution = Point::new(na * a.x + nb * b.x, na * a.y + nb * b.y);
    (&solution == prize).then_some(3 * na + nb)
}

#[derive(Debug, Clone)]
pub struct Data(Vec<[Point; 3]>);

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        Ok(Self(
            input
                .split("\n\n")
                .map(|block| {
                    let (adx, ady, bdx, bdy, x, y) = block
                        .split(|c: char| !c.is_ascii_digit())
                        .filter(|s| !s.is_empty())
                        .map(|s| s.parse().unwrap())
                        .collect_tuple()
                        .unwrap();
                    let a = Point::new(adx, ady);
                    let b = Point::new(bdx, bdy);
                    let prize = Point::new(x, y);
                    [a, b, prize]
                })
                .collect(),
        ))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        Ok(self
            .0
            .iter()
            .filter_map(|[a, b, prize]| solve(a, b, prize))
            .sum::<i64>())
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        Ok(self
            .0
            .iter()
            .filter_map(|[a, b, mut prize]| {
                prize.x += TEN_TRILLY;
                prize.y += TEN_TRILLY;
                solve(a, b, &prize)
            })
            .sum::<i64>())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "480");
    }
}
