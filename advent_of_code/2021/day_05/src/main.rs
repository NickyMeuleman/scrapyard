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

    let (from, to) = input.split_once(" -> ").unwrap();
    let (x, y) = from
        .split_once(",")
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .unwrap();
    let (xx, yy) = to
        .split_once(",")
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .unwrap();

    let (x_min, x_max) = if x <= xx { (x, xx) } else { (xx, x) };
    let (y_min, y_max) = if y <= yy { (y, yy) } else { (yy, y) };

    // calculate all points on a line
    // for horizontal or vertical lines
    if x == xx || y == yy {
        for x_pos in x_min..=x_max {
            for y_pos in y_min..=y_max {
                let point = Point { x: x_pos, y: y_pos };
                result.push(point);
            }
        }
    }
    // for diagonal lines
    else if part2 {
        let delta_x = (xx as isize - x as isize).signum();
        let delta_y = (yy as isize - y as isize).signum();
        // magnitude should be identical if I used x_max and x_min instead of y_max and y_min because of the strict 45deg angle
        let magnitude = y_max - y_min;
        for step in 0..=magnitude {
            let point = Point {
                x: (x as isize + (delta_x * step as isize)) as usize,
                y: (y as isize + (delta_y * step as isize)) as usize,
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
