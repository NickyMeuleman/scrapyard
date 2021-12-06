use std::cmp;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
struct Data(Vec<Line>);

impl Data {
    fn new(input: &str) -> Self {
        Self(input.lines().map(Line::new).collect())
    }

    fn part_one(&self) -> usize {
        self.0
            .iter()
            .fold(HashMap::new(), |mut acc: HashMap<Point, usize>, line| {
                for point in line.get_points(false) {
                    let count = acc.entry(point).or_default();
                    *count += 1;
                }
                acc
            })
            .iter()
            .filter(|(_, &count)| count >= 2)
            .count()
    }

    fn part_two(&self) -> usize {
        self.0
            .iter()
            .fold(HashMap::new(), |mut acc: HashMap<Point, usize>, line| {
                for point in line.get_points(true) {
                    let count = acc.entry(point).or_default();
                    *count += 1;
                }
                acc
            })
            .iter()
            .filter(|(_, &count)| count >= 2)
            .count()
    }
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

    fn get_points(&self, diagonals: bool) -> Vec<Point> {
        let (x1, y1, x2, y2) = (self.from.x, self.from.y, self.to.x, self.to.y);

        if !diagonals && !(x1 == x2 || y1 == y2) {
            return Vec::new();
        }

        let (x_min, x_max) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
        let (y_min, y_max) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };
        let magnitude = cmp::max(y_max - y_min, x_max - x_min);
        let sign_x = (x2 as isize - x1 as isize).signum();
        let sign_y = (y2 as isize - y1 as isize).signum();

        (0..=magnitude)
            .map(|step| Point {
                x: (x1 as isize + (sign_x * step as isize)) as usize,
                y: (y1 as isize + (sign_y * step as isize)) as usize,
            })
            .collect()
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Point {
    x: usize,
    y: usize,
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

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let data = Data::new(&input);
    println!("Part one answer: {}", data.part_one());
    println!("Part two answer: {}", data.part_two());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_one_example() {
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

        let data = Data::new(input);
        assert_eq!(data.part_one(), 5);
    }

    #[test]

    fn part_two_example() {
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

        let data = Data::new(input);
        assert_eq!(data.part_two(), 12);
    }
}
