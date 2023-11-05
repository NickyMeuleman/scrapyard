use std::{cmp, collections::HashMap, fmt::Display};

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data {
    lines: Vec<Line>,
}

#[derive(Debug, Clone)]
struct Line {
    from: Point,
    to: Point,
}

impl Line {
    fn try_new(input: &str) -> Option<Self> {
        let (from, to) = input.split_once(" -> ")?;
        let from = Point::try_new(from)?;
        let to = Point::try_new(to)?;
        Some(Self { from, to })
    }

    fn get_points(&self) -> Vec<Point> {
        let (x1, y1, x2, y2) = (self.from.x, self.from.y, self.to.x, self.to.y);

        let (x_min, x_max) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
        let (y_min, y_max) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };
        let magnitude = cmp::max(y_max - y_min, x_max - x_min);
        let sign_x = (x2 as isize - x1 as isize).signum();
        let sign_y = (y2 as isize - y1 as isize).signum();

        (0..=magnitude)
            .map(|step| Point {
                x: (x1 as isize + (sign_x * step as isize)) as u16,
                y: (y1 as isize + (sign_y * step as isize)) as u16,
            })
            .collect()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: u16,
    y: u16,
}

impl Point {
    fn try_new(input: &str) -> Option<Self> {
        let (x, y) = input.split_once(",")?;
        let x = x.parse().ok()?;
        let y = y.parse().ok()?;
        Some(Self { x, y })
    }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let lines = input
            .trim()
            .lines()
            .map(Line::try_new)
            .collect::<Option<Vec<Line>>>()
            .ok_or(AoCError::Parsing)?;
        Ok(Self { lines })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let result = self
            .lines
            .iter()
            .filter(|line| line.from.x == line.to.x || line.from.y == line.to.y)
            .fold(HashMap::new(), |mut acc: HashMap<Point, usize>, line| {
                for point in line.get_points() {
                    *acc.entry(point).or_default() += 1;
                }
                acc
            })
            .values()
            .filter(|&count| *count >= 2)
            .count();

        Ok(result)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let result = self
            .lines
            .iter()
            .fold(HashMap::new(), |mut acc: HashMap<Point, usize>, line| {
                for point in line.get_points() {
                    *acc.entry(point).or_default() += 1;
                }
                acc
            })
            .values()
            .filter(|&count| *count >= 2)
            .count();

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "5");
    }

    #[test]
    fn part_2() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "12");
    }
}
