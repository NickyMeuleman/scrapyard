use std::fmt::Display;

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<i32>);

fn part_1_helper(list: &mut Vec<i32>) -> AoCResult<()> {
    let mut pointer = 0;

    loop {
        let opcode = list[pointer];
        if opcode == 99 {
            break;
        }
        let pos1 = list[pointer + 1];
        let pos2 = list[pointer + 2];
        let pos3 = list[pointer + 3];
        let num1 = list[pos1 as usize];
        let num2 = list[pos2 as usize];
        let result = match opcode {
            1 => num1 + num2,
            2 => num1 * num2,
            _ => return Err(AoCError::Solving),
        };

        list[pos3 as usize] = result;
        pointer += 4;
    }
    Ok(())
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        Ok(Self(
            input
                .split(",")
                .filter_map(|s| s.parse().ok())
                .collect(),
        ))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut list = self.0.clone();
        list[1] = 12;
        list[2] = 2;
        part_1_helper(&mut list)?;

        Ok(list[0])
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        for noun in 0..=99 {
            for verb in 0..=99 {
                let mut list = self.0.clone();

                list[1] = noun;
                list[2] = verb;

                let mut pointer = 0;

                loop {
                    let opcode = list[pointer];
                    if opcode == 99 {
                        break;
                    }

                    let pos1 = list[pointer + 1];
                    let pos2 = list[pointer + 2];
                    let pos3 = list[pointer + 3];
                    let num1 = list[pos1 as usize];
                    let num2 = list[pos2 as usize];
                    let result = match opcode {
                        1 => num1 + num2,
                        2 => num1 * num2,
                        _ => panic!("At the disco"),
                    };

                    list[pos3 as usize] = result;
                    pointer += 4;
                }

                if list[0] == 19690720 {
                    return Ok(100 * noun + verb);
                }
            }
        }
        Err(AoCError::Solving)
    }
}
