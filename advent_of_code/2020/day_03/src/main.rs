use std::fs;
use std::convert::TryInto;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let map = parse_input(input);
    println!("part one answer: {}", part_one(&map));
    println!("part two answer: {}", part_two(&map));
}

#[derive(PartialEq)]
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

fn traverse_map(map: &Vec<Vec<Cell>>, slope: (usize, usize)) -> u32 {
    let map_height = map.len();
    let map_width = map[0].len();
    // the / operator gives the rounded down integer, if the result is 2.999999999 it will become 2
    let steps: usize = map_height / slope.1;
    let mut count = 0;
    for num in 0..steps {
        let column_idx: usize = (num * slope.0) % map_width;
        let row_idx: usize = num * slope.1;
        let cell = &map[row_idx][column_idx];
        if *cell == Cell::Tree {
            count += 1;
        }
    }
    count
}

fn traverse_map_2(map: &Vec<Vec<Cell>>, slope: (usize, usize)) -> u32 {
    let map_height = map.len();
    let map_width = map[0].len();
    // the / operator gives the rounded down integer, if the result is 2.999999999 it will become 2
    let steps = map_height / slope.1;
    (0..steps)
        .map(|num| {
            let column_idx = (num * slope.0) % map_width;
            let row_idx = num * slope.1;
            &map[row_idx][column_idx]
        })
        .filter(|&cell| *cell == Cell::Tree)
        .count()
        .try_into()
        .unwrap()
}

fn part_one(map: &Vec<Vec<Cell>>) -> u32 {
    traverse_map(map, (3, 1))
}

fn part_two(map: &Vec<Vec<Cell>>) -> u64 {
    let slopes: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    // overflowing i32, ohno
    slopes
        .iter()
        .map(|&slope| traverse_map_2(map, slope))
        .map(|num| u64::from(num))
        .product()
}
