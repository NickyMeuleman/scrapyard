use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let data: HashMap<Point, Cube> = parse(&input);
    println!("Part one answer: {}", part_one(data));
    // println!("Part two answer: {}", part_two(&data));
}

// NOTE: What's the difference between PartialEq and Eq?
#[derive(Hash, PartialEq, Eq, Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(PartialEq, Eq, Debug)]
enum Cube {
    Active,
    Inactive,
}

fn parse(input: &str) -> HashMap<Point, Cube> {
    // the sample input has a singular z coordinate, we'll pick 0
    // every row has the same y coordinate, to keep it mathemetical let's assume they go from 0 to -amnt of lines
    // every column has the same x coordinate, to keep it mathemetical let's assume they go from 0 to +amnt of characters
    // in other words, the top left of the input is (x=0,y=0,z=0), one spot to the right and to the bottom is (z=1,y=-1,z=0)
    input
        .lines()
        .enumerate()
        .map(|(y, line)| parse_line(0 - y as i32, line))
        .fold(HashMap::new(), |mut acc, map| {
            acc.extend(map);
            acc
        })
}

fn parse_line(y: i32, input: &str) -> HashMap<Point, Cube> {
    input
        .chars()
        .enumerate()
        .map(|(x, c)| parse_c(x as i32, y, c))
        .collect()
}

fn parse_c(x: i32, y: i32, c: char) -> (Point, Cube) {
    let point = Point { x: x, y: y, z: 0 };
    match c {
        '.' => (point, Cube::Inactive),
        '#' => (point, Cube::Active),
        _ => panic!("invalid cube state found"),
    }
}

fn tick(data: HashMap<Point, Cube>) -> HashMap<Point, Cube> {
    let mut result: HashMap<Point, Cube> = HashMap::new();
    for (point, cube) in &data {
        let neighbours = get_neighbours(&point);
        let active_neighbours = get_active_count(&data,neighbours);
        let new_cube = match cube {
            Cube::Active => {
                match active_neighbours {
                    2 | 3 => Cube::Active,
                    _ => Cube::Inactive,
                }
            }
            Cube::Inactive => {
                match active_neighbours {
                    3 => Cube::Active,
                    _ => Cube::Inactive,
                }
            }
        };
        result.insert(point, new_cube);
    }
    result
}

fn part_one(data: HashMap<Point, Cube>) -> usize {
    get_final_state(data, 6)
        .values()
        .filter(|&cube| *cube == Cube::Active)
        .count()
}

fn get_final_state(data: HashMap<Point, Cube>, iterations: usize) -> HashMap<Point, Cube> {
    if iterations == 0 {
        return data;
    }
    let new_state = tick(data);
    get_final_state(new_state, iterations - 1)
}

fn get_neighbours(point: &Point) -> Vec<Point> {
    let mut result: Vec<Point> = Vec::new();
    for x in point.x - 1..point.x + 1 {
        for y in point.y - 1..point.y + 1 {
            for z in point.z - 1..point.z + 1 {
                let point = Point { x: x, y: y, z: z };
                result.push(point);
            }
        }
    }
    // filter out the point that's identical to the function input
    result
        .into_iter()
        .filter(|item| !(point.x == item.x && point.y == item.y && point.z == item.z))
        .collect()
}

fn get_active_count(grid: &HashMap<Point, Cube>, points: Vec<Point>) -> usize {
    points.iter().map(|point| grid.get(point).unwrap_or(&Cube::Inactive)).filter(|&cube| *cube == Cube::Active).count()
}