use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let answers = parse(&input);
    println!("part one answer: {}", part_one(&answers));
    println!("part two answer: {}", part_two(&answers));
}

fn parse(input: &String) -> Vec<Vec<Vec<char>>> {
    let blocks: Vec<&str> = input.split("\n\n").collect();
    let mut parsed = Vec::new();
    for block in blocks {
        let block = parse_block(block);
        parsed.push(block);
    }
    parsed
}

fn parse_block(block: &str) -> Vec<Vec<char>> {
    // TODO: rewrite with iterators
    let lines: Vec<&str> = block.split("\n").collect();
    let mut parsed_lines = Vec::new();
    for line in lines {
        let parsed_line = parse_line(line);
        parsed_lines.push(parsed_line)
    }
    parsed_lines
}

fn parse_line(line: &str) -> Vec<char> {
    line.chars().collect()
}

fn part_one(answers: &Vec<Vec<Vec<char>>>) -> usize {
    let mut num_ans_per_block: Vec<usize> = Vec::new();
    // loop over blocks
    for block in answers {
        // for each block, create hashset
        let mut block_answers = HashSet::new();
        // loop over answers
        for line in block {
            // loop over answers in line
            for c in line {
                // add to hashset if not already present
                block_answers.replace(c);
            }
        }
        // count items in HashSet
        // add to vec
        num_ans_per_block.push(block_answers.len());
    }
    // sum answer count per block
    num_ans_per_block.iter().sum()
}

fn part_two(answers: &Vec<Vec<Vec<char>>>) -> usize {
    let mut num_ans_per_block: Vec<usize> = Vec::new();
    // loop over blocks
    for block in answers {
        // for each block, create hashmap
        let mut block_answers: HashMap<char, usize> = HashMap::new();
        // loop over answers
        for line in block {
            // loop over answers in line
            for c in line {
                // add to block hashmap with count of occurrences
                block_answers
                    .entry(*c)
                    .and_modify(|val| *val += 1)
                    .or_insert(1);
            }
        }
        let mut count = 0;
        for (k, v) in block_answers {
            if v == block.len() {
                count+=1
            }
        }
        num_ans_per_block.push(count);
    }
    num_ans_per_block.iter().sum()
}
// fn part_two(answers: &Vec<Vec<Vec<char>>>) -> usize {
//     let mut num_ans_per_block: Vec<usize> = Vec::new();
//     // loop over blocks
//     for block in answers {
//         // for each block, create hashmap
//         let mut block_answers: HashMap<char, usize> = HashMap::new();
//         // loop over answers
//         for line in block {
//             // loop over answers in line
//             for c in line {
//                 // add to hashmap with count of occurrences
//                 block_answers
//                     .entry(*c)
//                     .and_modify(|val| *val += 1)
//                     .or_insert(1);
//             }
//         }
//         // loop over hashmap
//         // if item count equal to amound of lines, increment count
//         let count = block_answers.iter().filter_map(|entry| {
//             if *entry.1 == block.len() {
//                 Some(entry.0)
//             } else {
//                 None
//             }
//         }).count();
//         num_ans_per_block.push(count);
//     }
//     num_ans_per_block.iter().sum()
// }

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
