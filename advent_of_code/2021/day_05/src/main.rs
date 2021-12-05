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

    println!("Part one answer: {}", part_one(parse_part_one(&input)));
    println!("Part two answer: {}", part_two(parse_part_two(&input)));
}

fn parse_part_one(input: &str) -> Data {
    let mut result: HashMap<Point, usize> = HashMap::new();
    let lines: Vec<Vec<Point>> = input.lines().map(|line| parse_line_part1(line)).collect();
    for line in lines {
        for point in line {
            // increment point counter
            let count = result.entry(point).or_default();
            *count += 1;
        }
    }

    result
}

fn parse_part_two(input: &str) -> Data {
    let mut result: HashMap<Point, usize> = HashMap::new();
    let lines: Vec<Vec<Point>> = input.lines().map(|line| parse_line_part2(line)).collect();
    for line in lines {
        for point in line {
            // increment point counter
            let count = result.entry(point).or_default();
            *count += 1;
        }
    }

    result
}

fn parse_line_part1(input: &str) -> Vec<Point> {
    // get all individual points from a line
    // only conside horizontal or vertical lines
    // LOTS of duplication, oh well
    let mut result = Vec::new();

    let (from, to) = input.split_once(" -> ").unwrap();
    let (x, y) = from.split_once(",").unwrap();
    let (xx, yy) = to.split_once(",").unwrap();
    let x: usize = x.parse().unwrap();
    let xx: usize = xx.parse().unwrap();
    let y: usize = y.parse().unwrap();
    let yy: usize = yy.parse().unwrap();
    // check if horizontal
    let horizontal = x == xx;
    // check if vertical
    let vertical = y == yy;
    // calculate all points on that line
    if horizontal || vertical {
        if x <= xx && y <= yy {
            for x_pos in x..=xx {
                for y_pos in y..=yy {
                    result.push(Point { x: x_pos, y: y_pos });
                }
            }
        } else if x <= xx && y >= yy {
            for x_pos in x..=xx {
                for y_pos in yy..=y {
                    result.push(Point { x: x_pos, y: y_pos });
                }
            }
        } else if x >= xx && y <= yy {
            for x_pos in xx..=x {
                for y_pos in y..=yy {
                    result.push(Point { x: x_pos, y: y_pos });
                }
            }
        } else if x >= xx && y >= yy {
            for x_pos in xx..=x {
                for y_pos in yy..=y {
                    result.push(Point { x: x_pos, y: y_pos });
                }
            }
        }
    }
    result
}

fn parse_line_part2(input: &str) -> Vec<Point> {
    // get all individual points from a line
    // also consider diagonals
    // LOTS of duplication, oh well
    let mut result = Vec::new();

    let (from, to) = input.split_once(" -> ").unwrap();
    let (x, y) = from.split_once(",").unwrap();
    let (xx, yy) = to.split_once(",").unwrap();
    let x: usize = x.parse().unwrap();
    let xx: usize = xx.parse().unwrap();
    let y: usize = y.parse().unwrap();
    let yy: usize = yy.parse().unwrap();
    // check if horizontal
    let horizontal = x == xx;
    // check if vertical
    let vertical = y == yy;
    // calculate all points on that line
    if horizontal || vertical {
        if x <= xx && y <= yy {
            for x_pos in x..=xx {
                for y_pos in y..=yy {
                    result.push(Point { x: x_pos, y: y_pos });
                }
            }
        } else if x <= xx && y >= yy {
            for x_pos in x..=xx {
                for y_pos in yy..=y {
                    result.push(Point { x: x_pos, y: y_pos });
                }
            }
        } else if x >= xx && y <= yy {
            for x_pos in xx..=x {
                for y_pos in y..=yy {
                    result.push(Point { x: x_pos, y: y_pos });
                }
            }
        } else if x >= xx && y >= yy {
            for x_pos in xx..=x {
                for y_pos in yy..=y {
                    result.push(Point { x: x_pos, y: y_pos });
                }
            }
        }
    } else {
        //  I'M SO SORRY FOR THIS
        // must be diagonal, at an exact 45 deg angle per the question
        let (y_min, y_max) = if y <= yy { (y, yy) } else { (yy, y) };
        let delta_x = (xx as isize - x as isize).signum();
        let delta_y = (yy as isize - y as isize).signum();
        // magnitude should be identical if I used x_max and x_min instead of y_max and y_min
        let magnitude = y_max - y_min;
        for step in 0..=magnitude {
            result.push(Point {
                x: (x as isize + (delta_x * step as isize)) as usize,
                y: (y as isize + (delta_y * step as isize)) as usize,
            })
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

        let data = parse_part_one(input);
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

        let data = parse_part_two(input);
        assert_eq!(part_two(data), 12);
    }
}
