use crate::AoCData;
use hashbrown::HashMap;
use std::cmp;

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
    fn new(input: &str) -> Self {
        input
            .split_once(" -> ")
            .map(|(from, to)| Line {
                from: Point::new(from),
                to: Point::new(to),
            })
            .unwrap()
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
    fn new(input: &str) -> Self {
        input
            .split_once(",")
            .map(|(x, y)| Point {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            })
            .unwrap()
    }
}

impl AoCData for Data {
    fn new(input: String) -> Self {
        Self {
            lines: input.trim().lines().map(Line::new).collect(),
        }
    }

    fn part_1(&self) -> String {
        self.lines
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
            .count()
            .to_string()
    }

    fn part_2(&self) -> String {
        self.lines
            .iter()
            .fold(HashMap::new(), |mut acc: HashMap<Point, usize>, line| {
                for point in line.get_points() {
                    *acc.entry(point).or_default() += 1;
                }
                acc
            })
            .values()
            .filter(|&count| *count >= 2)
            .count()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(5);
        let data = Data::new(input);
        assert_eq!(data.part_1(), "5");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(5);
        let data = Data::new(input);
        assert_eq!(data.part_2(), "12");
    }
}
