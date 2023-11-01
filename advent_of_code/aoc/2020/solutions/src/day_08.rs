use std::{collections::HashSet, fmt::Display};

use itertools::Itertools;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data<'a>(Vec<(&'a str, i32)>);

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

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        fn parse_line(line: &str) -> (&str, i32) {
            let (opcode, value): (&str, &str) = line
                .splitn(2, " ")
                .collect_tuple()
                .unwrap();
            let value = i32::from_str_radix(value, 10).unwrap();
            (opcode, value)
        }
        Ok(Self(
            input
                .lines()
                .map(|line| parse_line(line))
                .collect(),
        ))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        Ok(execute(&self.0).unwrap_err())
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        for (idx, &instruction) in self.0.iter().enumerate() {
            // if jmp or nop found, swap and try to run resulting program
            // if program finishes, return acc
            match instruction {
                ("acc", _) => continue,
                ("nop", val) => {
                    let mut new_instructions = self.0.clone();
                    new_instructions[idx] = ("jmp", val);
                    if let Ok(acc) = execute(&new_instructions) {
                        return Ok(acc);
                    }
                }
                ("jmp", val) => {
                    let mut new_instructions = self.0.clone();
                    new_instructions[idx] = ("nop", val);
                    if let Ok(acc) = execute(&new_instructions) {
                        return Ok(acc);
                    }
                }
                _ => continue,
            }
        }

        Ok(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "5");
    }

    #[test]
    fn part_2() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "8");
    }
}
