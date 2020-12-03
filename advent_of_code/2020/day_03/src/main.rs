use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let map = parse_input(input);
    println!("part one answer: {}", part_one(&map));
    println!("part two answer: {}", part_two(&map));
}

#[derive(Debug, PartialEq)]
enum Cell {
    Tree,
    Empty,
}

fn parse_input(input: String) -> Vec<Vec<Cell>> {
    input.lines().map(|line| parse_line(line)).collect()
}

fn parse_line(line: &str) -> Vec<Cell> {
    line.chars().map(|c| parse_char(c)).collect()
}

fn parse_char(c: char) -> Cell {
    match c {
        '#' => Cell::Tree,
        _ => Cell::Empty,
    }
}

fn traverse_map(map: &Vec<Vec<Cell>>, slope: (i32, i32)) -> i32 {
    // TODO: overhaul all these i32 and usize types to avoid casting
    let map_height = map.len();
    let map_width = map[0].len();
    // the / operator gives the rounded down integer, if the result is 2.999999999 it will become 2
    let steps = map_height as i32 / slope.1;
    let mut count = 0;
    for num in 0..steps {
        let column_idx = (num * slope.0) as usize % map_width;
        let row_idx = num as usize * slope.1 as usize;
        let cell = &map[row_idx][column_idx];
        if *cell == Cell::Tree {
            count += 1;
        }
    }
    count
}

fn part_one(map: &Vec<Vec<Cell>>) -> i32 {
    traverse_map(map, (3, 1))
}

fn part_two(map: &Vec<Vec<Cell>>) -> i64 {
    let slopes: [(i32, i32); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut trees_for_slopes: Vec<i32> = Vec::new();
    for slope in &slopes {
        let num_trees = traverse_map(map, *slope);
        trees_for_slopes.push(num_trees);
    }
    // overflowing i32, ohno
    trees_for_slopes.iter().map(|num| *num as i64).product()
}
