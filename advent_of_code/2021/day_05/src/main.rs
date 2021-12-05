use std::collections::HashMap;
use std::fs;

type Data = HashMap<Point, usize>;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Point {
    x: usize,
    y: usize,
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("Part one answer: {}", part_one(parse(&input, false)));
    println!("Part two answer: {}", part_two(parse(&input, true)));
}

fn parse(input: &str, part2: bool) -> Data {
    input
        .lines()
        .map(|line| parse_line(line, part2))
        .fold(HashMap::new(), |mut acc, line| {
            line.into_iter().for_each(|point| {
                let count = acc.entry(point).or_default();
                *count += 1;
            });
            acc
        })
}

fn parse_line(input: &str, part2: bool) -> Vec<Point> {
    let mut result = Vec::new();

    let ((x1, y1), (x2, y2)) = input
        .split_once(" -> ")
        .map(|(from, to)| {
            let (x1, y1) = from
                .split_once(",")
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                .unwrap();
            let (x2, y2) = to
                .split_once(",")
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                .unwrap();
            ((x1, y1), (x2, y2))
        })
        .unwrap();

    let (x_min, x_max) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
    let (y_min, y_max) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };

    // calculate all points on a line
    // for horizontal or vertical lines
    if x1 == x2 || y1 == y2 {
        for x_pos in x_min..=x_max {
            for y_pos in y_min..=y_max {
                let point = Point { x: x_pos, y: y_pos };
                result.push(point);
            }
        }
    }
    // for diagonal lines
    else if part2 {
        // so, doing math with numbers of a different types is NOT fun. Beware, there be "as"
        // assuming there is no information loss by using "as"
        let sign_x = (x2 as isize - x1 as isize).signum();
        let sign_y = (y2 as isize - y1 as isize).signum();
        // magnitude would be identical if I used x_max and x_min instead of y_max and y_min because of the strict 45deg angle
        let magnitude = y_max - y_min;
        for step in 0..=magnitude {
            let point = Point {
                x: (x1 as isize + (sign_x * step as isize)) as usize,
                y: (y1 as isize + (sign_y * step as isize)) as usize,
            };
            result.push(point);
        }
    }

    result
}

fn part_one(data: Data) -> usize {
    data.iter().filter(|(_, &count)| count >= 2).count()
}

fn part_two(data: Data) -> usize {
    data.iter().filter(|(_, &count)| count >= 2).count()
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

        let data = parse(input, false);
        assert_eq!(part_one(data), 5);
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

        let data = parse(input, true);
        assert_eq!(part_two(data), 12);
    }
}
