use std::collections::HashMap;
use std::fs;

// TODO: get rid of all that 3d and 4d duplication
// Rust made me copy all functions and change the input types to every one, even if that was the only necessary change

type Grid = HashMap<Point, Cube>;
type Grid4d = HashMap<Point4d, Cube>;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let data: Grid = parse(&input);
    let data4d: Grid4d = parse4d(&input);
    println!("Part one answer: {}", part_one(data));
    println!("Part two answer: {}", part_two(data4d));
}

// NOTE: What's the difference between PartialEq and Eq?
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
struct Point4d {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Cube {
    Active,
    Inactive,
}

fn parse(input: &str) -> Grid {
    // the sample input has a singular z coordinate, we'll pick 0
    // every row has the same y coordinate, to keep it mathemetical let's assume they go from 0 to -amnt of lines
    // every column has the same x coordinate, to keep it mathemetical let's assume they go from 0 to +amnt of characters
    // in other words, the top left of the input is (x=0,y=0,z=0), one spot to the right and to the bottom is (z=1,y=-1,z=0)

    // edit: not a normal grid apparently, top left is 0,0 for xy and the y goes up as a point goes down, computer scientists, amirite
    input
        .lines()
        .enumerate()
        .map(|(y, line)| parse_line(y as i32, line))
        .fold(HashMap::new(), |mut acc, map| {
            acc.extend(map);
            acc
        })
}

fn parse4d(input: &str) -> Grid4d {
    input
        .lines()
        .enumerate()
        .map(|(y, line)| parse_line4d(y as i32, line))
        .fold(HashMap::new(), |mut acc, map| {
            acc.extend(map);
            acc
        })
}

fn parse_line(y: i32, input: &str) -> Grid {
    input
        .chars()
        .enumerate()
        .map(|(x, c)| parse_c(x as i32, y, c))
        .collect()
}

fn parse_line4d(y: i32, input: &str) -> Grid4d {
    input
        .chars()
        .enumerate()
        .map(|(x, c)| parse_c4d(x as i32, y, c))
        .collect()
}

fn parse_c(x: i32, y: i32, c: char) -> (Point, Cube) {
    let point = Point { x, y, z: 0 };
    match c {
        '.' => (point, Cube::Inactive),
        '#' => (point, Cube::Active),
        _ => panic!("invalid cube state found"),
    }
}

fn parse_c4d(x: i32, y: i32, c: char) -> (Point4d, Cube) {
    let point = Point4d { x, y, z: 0, w:0 };
    match c {
        '.' => (point, Cube::Inactive),
        '#' => (point, Cube::Active),
        _ => panic!("invalid cube state found"),
    }
}

fn tick(data: Grid) -> Grid {
    let mut result: Grid = HashMap::new();
    // expand data by one in every direction and set those points to be Inactive, these points can have the right amount of active neighbours too
    let expanded_grid = expand_grid(data.clone());
    for (point, cube) in expanded_grid {
        let neighbours = get_neighbours(&point);
        let active_neighbours = get_active_count(&data, neighbours);
        let new_cube = match cube {
            Cube::Active => match active_neighbours {
                2 | 3 => Cube::Active,
                _ => Cube::Inactive,
            },
            Cube::Inactive => match active_neighbours {
                3 => Cube::Active,
                _ => Cube::Inactive,
            },
        };
        result.insert(point, new_cube);
    }
    result
}

fn tick4d(data: Grid4d) -> Grid4d {
    let mut result: Grid4d = HashMap::new();
    // expand data by one in every direction and set those points to be Inactive, these points can have the right amount of active neighbours too
    let expanded_grid = expand_grid4d(data.clone());
    for (point, cube) in expanded_grid {
        let neighbours = get_neighbours4d(&point);
        let active_neighbours = get_active_count4d(&data, neighbours);
        let new_cube = match cube {
            Cube::Active => match active_neighbours {
                2 | 3 => Cube::Active,
                _ => Cube::Inactive,
            },
            Cube::Inactive => match active_neighbours {
                3 => Cube::Active,
                _ => Cube::Inactive,
            },
        };
        result.insert(point, new_cube);
    }
    result
}

fn part_one(data: Grid) -> usize {
    // 239 is too high
    // 198 is too low
    get_final_state(data, 6)
        .values()
        .filter(|&cube| *cube == Cube::Active)
        .count()
}

fn part_two(data: Grid4d) -> usize {
    // 239 is too high
    // 198 is too low
    get_final_state4d(data, 6)
        .values()
        .filter(|&cube| *cube == Cube::Active)
        .count()
}

fn get_final_state(data: Grid, iterations: usize) -> Grid {
    if iterations == 0 {
        return data;
    }
    let new_state = tick(data);
    get_final_state(new_state, iterations - 1)
}

fn get_final_state4d(data: Grid4d, iterations: usize) -> Grid4d {
    if iterations == 0 {
        return data;
    }
    let new_state = tick4d(data);
    get_final_state4d(new_state, iterations - 1)
}

fn get_neighbours(point: &Point) -> Vec<Point> {
    let mut result: Vec<Point> = Vec::new();
    for x in (point.x - 1)..=(point.x + 1) {
        for y in (point.y - 1)..=(point.y + 1) {
            for z in (point.z - 1)..=(point.z + 1) {
                let point = Point { x, y, z };
                result.push(point);
            }
        }
    }
    // filter out the point that's identical to the function input
    result.into_iter().filter(|item| point != item).collect()
}

fn get_neighbours4d(point: &Point4d) -> Vec<Point4d> {
    let mut result: Vec<Point4d> = Vec::new();
    for x in (point.x - 1)..=(point.x + 1) {
        for y in (point.y - 1)..=(point.y + 1) {
            for z in (point.z - 1)..=(point.z + 1) {
                for w in point.w - 1..=point.w + 1 {
                    let point = Point4d { x, y, z, w };
                    result.push(point);
                }
            }
        }
    }
    // filter out the point that's identical to the function input
    result.into_iter().filter(|item| point != item).collect()
}

fn get_active_count(grid: &Grid, points: Vec<Point>) -> usize {
    points
        .iter()
        .map(|point| grid.get(point).unwrap_or(&Cube::Inactive))
        .filter(|&cube| *cube == Cube::Active)
        .count()
}

fn get_active_count4d(grid: &Grid4d, points: Vec<Point4d>) -> usize {
    points
        .iter()
        .map(|point| grid.get(point).unwrap_or(&Cube::Inactive))
        .filter(|&cube| *cube == Cube::Active)
        .count()
}

fn expand_grid(mut grid: Grid) -> Grid {
    let max_x = grid.iter().map(|(k, _)| k.x).max().unwrap_or(0);
    let min_x = grid.iter().map(|(k, _)| k.x).min().unwrap_or(0);
    let max_y = grid.iter().map(|(k, _)| k.y).max().unwrap_or(0);
    let min_y = grid.iter().map(|(k, _)| k.y).min().unwrap_or(0);
    let max_z = grid.iter().map(|(k, _)| k.z).max().unwrap_or(0);
    let min_z = grid.iter().map(|(k, _)| k.z).min().unwrap_or(0);
    for x in min_x - 1..=max_x + 1 {
        for y in min_y - 1..=max_y + 1 {
            for z in min_z - 1..=max_z + 1 {
                let point = Point { x, y, z };
                if !grid.contains_key(&point) {
                    grid.insert(point, Cube::Inactive);
                }
            }
        }
    }
    grid
}

fn expand_grid4d(mut grid: Grid4d) -> Grid4d {
    let max_x = grid.iter().map(|(k, _)| k.x).max().unwrap_or(0);
    let min_x = grid.iter().map(|(k, _)| k.x).min().unwrap_or(0);
    let max_y = grid.iter().map(|(k, _)| k.y).max().unwrap_or(0);
    let min_y = grid.iter().map(|(k, _)| k.y).min().unwrap_or(0);
    let max_z = grid.iter().map(|(k, _)| k.z).max().unwrap_or(0);
    let min_z = grid.iter().map(|(k, _)| k.z).min().unwrap_or(0);
    let max_w = grid.iter().map(|(k, _)| k.w).max().unwrap_or(0);
    let min_w = grid.iter().map(|(k, _)| k.w).min().unwrap_or(0);
    for x in min_x - 1..=max_x + 1 {
        for y in min_y - 1..=max_y + 1 {
            for z in min_z - 1..=max_z + 1 {
                for w in min_w - 1..=max_w + 1 {
                    let point = Point4d { x, y, z, w };
                    if !grid.contains_key(&point) {
                        grid.insert(point, Cube::Inactive);
                    }
                }
            }
        }
    }
    grid
}

fn print_grid(grid: &Grid) {
    // contrasting my solution to the example output in the question
    // at iteration 3, at z=1,
    // 2 cubes seem to be in the wrong state (inactive instead of active)
    // all other cubes are in the correct state
    let max_x = grid.iter().map(|(k, _)| k.x).max().unwrap_or(0);
    let min_x = grid.iter().map(|(k, _)| k.x).min().unwrap_or(0);
    let max_y = grid.iter().map(|(k, _)| k.y).max().unwrap_or(0);
    let min_y = grid.iter().map(|(k, _)| k.y).min().unwrap_or(0);
    let max_z = grid.iter().map(|(k, _)| k.z).max().unwrap_or(0);
    let min_z = grid.iter().map(|(k, _)| k.z).min().unwrap_or(0);
    for z in min_z..=max_z {
        println!("z = {}", z);
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                print_cube(grid, Point { x, y, z });
            }
            println!("\n");
        }
    }
}

fn print_cube(grid: &Grid, point: Point) {
    // why does this panic if I don't unwrap_or? It should only be called with valid points.
    let cube = grid.get(&point).unwrap_or(&Cube::Inactive);
    let cube = match *cube {
        Cube::Inactive => '.',
        Cube::Active => '#',
    };
    print!("{}", cube)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_part_one() {
        let input = ".#.
..#
###"
        .to_owned();
        let data = parse(&input);

        assert_eq!(part_one(data), 112);
    }

    #[test]
    fn solves_part_two() {
        let input = ".#.
..#
###"
        .to_owned();
        let data = parse4d(&input);

        assert_eq!(part_two(data), 848);
    }
}
