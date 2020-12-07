use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let answers = parse(&input);
    println!("part one answer: {}", part_one(&answers));
    println!("part two answer: {}", part_two(&answers));
}

fn parse(input: &String) -> Vec<Vec<HashSet<char>>> {
    input
        .split("\n\n")
        .map(|block| parse_block(block))
        .collect()
}

fn parse_block(block: &str) -> Vec<HashSet<char>> {
    block.split("\n").map(|line| parse_line(line)).collect()
}

fn parse_line(line: &str) -> HashSet<char> {
    line.chars().collect()
}

fn part_one(answers: &Vec<Vec<HashSet<char>>>) -> usize {
    answers
        .into_iter()
        .map(|block| {
            let start_acc = HashSet::new();
            block
                .into_iter()
                .fold(start_acc, |acc, line| {
                    acc.union(line).map(|c| *c).collect()
                })
                .len()
        })
        .sum()
}

fn part_two(answers: &Vec<Vec<HashSet<char>>>) -> usize {
    answers
        .into_iter()
        .map(|block| {
            // Rust has an experimental fold_first method where the first item is the starting value of the acc
            let start_acc = block[0].clone();
            block
                .into_iter()
                .fold(start_acc, |acc, line| acc.intersection(line).map(|c| *c).collect())
                .len()
        })
        .sum()
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
    // edit: it was formatting, used the r#"a string with preserved whitespace"# syntax
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
    assert_eq!(part_two(&answers), 6);
}
