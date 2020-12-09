use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let instructions = parse(&input);
    println!("part one answer: {}", part_one(&instructions));
    println!("part two answer: {}", part_two(&instructions));
}

fn parse(input: &String) -> Vec<(&str, i32)> {
    input.lines().map(|line| parse_line(line)).collect()
}

fn parse_line(line: &str) -> (&str, i32) {
    let (opcode, value): (&str, &str) = line.splitn(2, " ").collect_tuple().unwrap();
    let value = i32::from_str_radix(value, 10).unwrap();
    (opcode, value)
}

fn part_one(instructions: &Vec<(&str, i32)>) -> i32 {
    execute(instructions).unwrap_err()
}

fn execute(instructions: &Vec<(&str, i32)>) -> Result<i32, i32> {
    let mut curr_idx: isize = 0;
    let mut acc = 0;
    let mut seen: HashSet<isize> = HashSet::new();
    loop {
        // check for invalid index
        if curr_idx < 0 || curr_idx > instructions.len() as isize + 1 {
            dbg!("oh noes");
            break Err(acc);
        }
        // check if instruction was already seen
        if seen.contains(&curr_idx) {
            break Err(acc);
        }
        // check if idx is same as length
        if curr_idx == instructions.len() as isize {
            break Ok(acc);
        }
        seen.insert(curr_idx);
        match instructions[curr_idx as usize] {
            ("nop", _) => curr_idx += 1,
            ("acc", val) => {
                curr_idx += 1;
                acc += val;
            }
            ("jmp", val) => {
                curr_idx += val as isize;
            }
            _ => (),
        }
    }
}

fn part_two(instructions: &Vec<(&str, i32)>) -> i32 {
    for (idx, &instruction) in instructions.iter().enumerate() {
        // if jmp or nop found, swap and try to run resulting program
        // if program finishes, return acc
        match instruction {
            ("acc", _) => continue,
            ("nop", val) => {
                let mut new_instructions = instructions.clone();
                new_instructions[idx] = ("jmp", val);
                if let Ok(acc) = execute(&new_instructions) {
                    return acc;
                }
            }
            ("jmp", val) => {
                let mut new_instructions = instructions.clone();
                new_instructions[idx] = ("nop", val);
                if let Ok(acc) = execute(&new_instructions) {
                    return acc;
                }
            }
            _ => continue,
        }
    }
    0
}
