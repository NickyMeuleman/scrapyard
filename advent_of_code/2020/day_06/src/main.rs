use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let answers = parse(&input);
    println!("part one answer: {}", part_one(&answers));
    println!("part two answer: {}", part_two(&answers));
}

fn parse(input: &String) -> Vec<(usize, HashMap<char, usize>)> {
    input
        .split("\n\n")
        .map(|block| parse_block(block))
        .collect()
}

fn parse_block(block: &str) -> (usize, HashMap<char, usize>) {
    let block: Vec<HashSet<char>> = block.split("\n").map(|line| parse_line(line)).collect();
    let mut map: HashMap<char, usize> = HashMap::new();
    let mut num_lines = 0;
    for line in block {
        for c in line {
            map.entry(c).and_modify(|val| *val += 1).or_insert(1);
        }
        num_lines += 1;
    }
    (num_lines, map)
}

fn parse_line(line: &str) -> HashSet<char> {
    line.chars().collect()
}

fn part_one(answers: &Vec<(usize, HashMap<char, usize>)>) -> usize {
    answers.iter().map(|block| block.1.len()).sum()
}

fn part_two(answers: &Vec<(usize, HashMap<char, usize>)>) -> usize {
    answers.iter().map(|block| get_every(block.0, &block.1)).sum()
}

fn get_every(line_num: usize, map: &HashMap<char, usize>) -> usize {
    let mut count = 0;
    for (_, v) in map {
        if *v == line_num {
            count += 1
        }
    }
    dbg!(line_num,&map);
    count
}

#[test]
fn part1() {
    let input = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#
    .to_owned();
    let answers = parse(&input);
    // why is this failing when the puzzle passed with the big input in input.txt? Is the test input wrong?
    // edit: it was formatting, used the r#"a string whit preserved whitespace"# syntax
    assert_eq!(part_one(&answers), 11);
}

#[test]
fn part2() {
    let input = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#
    .to_owned();
    let answers = parse(&input);
    // why is this failing when the puzzle passed with the big input in input.txt? Is the test input wrong?
    assert_eq!(part_two(&answers), 6);
}
