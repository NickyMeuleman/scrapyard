use std::fmt::Display;

use aoc_core::AoCError;

use crate::{intcode::Computer, AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<i64>);

// fn get_value(memory: &[i32], pointer: i32, mode: u32) -> i32 {
//     match mode {
//         0 => {
//             let pos = memory[pointer as usize];
//             memory[pos as usize]
//         }
//         1 => memory[pointer as usize],
//         _ => panic!("Unknown mode (not position or immediate)"),
//     }
// }

// fn parse_first_value(memory: &Vec<i32>, pointer: i32) -> (i32, Vec<u32>) {
//     // last 2 digits are the opcode
//     let num = memory[pointer as usize];
//     let mut modes = Vec::new();
//     let opcode = num % 100;
//     for c in num.to_string().chars().rev().skip(2) {
//         let mode = c.to_digit(10).unwrap();
//         modes.push(mode);
//     }
//     (opcode, modes)
// }

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        Ok(Self(
            input
                .split(",")
                .filter_map(|s| s.parse().ok())
                .collect(),
        ))
    }

    // fn part_1(&self) -> AoCResult<impl Display> {
    //     let mut memory = self.0.clone();
    //     let mut pointer = 0; // index into memory
    //     let register = 1; // the thing where input is stored
    //     let mut output = 0;

    //     loop {
    //         let (opcode, modes) = parse_first_value(&memory, pointer);
    //         match opcode {
    //             99 => break,
    //             1 => {
    //                 let num_1 = get_value(&memory, pointer + 1, *modes.get(0).unwrap_or(&0));
    //                 let num_2 = get_value(&memory, pointer + 2, *modes.get(1).unwrap_or(&0));
    //                 let output_pos = memory[pointer as usize + 3];
    //                 let result = num_1 + num_2;
    //                 memory[output_pos as usize] = result;
    //                 pointer += 4;
    //             }
    //             2 => {
    //                 let num_1 = get_value(&memory, pointer + 1, *modes.get(0).unwrap_or(&0));
    //                 let num_2 = get_value(&memory, pointer + 2, *modes.get(1).unwrap_or(&0));
    //                 let output_pos = memory[pointer as usize + 3];
    //                 let result = num_1 * num_2;
    //                 memory[output_pos as usize] = result;
    //                 pointer += 4;
    //             }
    //             3 => {
    //                 let output_pos = memory[pointer as usize + 1];
    //                 memory[output_pos as usize] = register;
    //                 pointer += 2;
    //             }
    //             4 => {
    //                 let value = get_value(&memory, pointer + 1, *modes.get(0).unwrap_or(&0));
    //                 output = value;
    //                 pointer += 2;
    //             }
    //             _ => return Err(AoCError::Solving),
    //         }
    //     }
    //     Ok(output)
    // }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut computer: Computer = Default::default();
        computer.set_memory(self.0.clone());
        computer.input(1);
        computer.run()?;
        computer
            .last_output()
            .ok_or(AoCError::Solving)
    }

    // fn part_2(&self) -> AoCResult<impl Display> {
    //     let mut memory = self.0.clone();
    //     let mut pointer = 0; // index into memory
    //     let register = 5; // the thing where input is stored
    //     let mut output = 0;

    //     loop {
    //         let (opcode, modes) = parse_first_value(&memory, pointer);
    //         match opcode {
    //             99 => break,
    //             1 => {
    //                 let num_1 = get_value(&memory, pointer + 1, *modes.get(0).unwrap_or(&0));
    //                 let num_2 = get_value(&memory, pointer + 2, *modes.get(1).unwrap_or(&0));
    //                 let output_pos = memory[pointer as usize + 3];
    //                 let result = num_1 + num_2;
    //                 memory[output_pos as usize] = result;
    //                 pointer += 4;
    //             }
    //             2 => {
    //                 let num_1 = get_value(&memory, pointer + 1, *modes.get(0).unwrap_or(&0));
    //                 let num_2 = get_value(&memory, pointer + 2, *modes.get(1).unwrap_or(&0));
    //                 let output_pos = memory[pointer as usize + 3];
    //                 let result = num_1 * num_2;
    //                 memory[output_pos as usize] = result;
    //                 pointer += 4;
    //             }
    //             3 => {
    //                 let output_pos = memory[pointer as usize + 1];
    //                 memory[output_pos as usize] = register;
    //                 pointer += 2;
    //             }
    //             4 => {
    //                 let value = get_value(&memory, pointer + 1, *modes.get(0).unwrap_or(&0));
    //                 output = value;
    //                 pointer += 2;
    //             }
    //             5 => {
    //                 let val_1 = get_value(&memory, pointer + 1, *modes.get(0).unwrap_or(&0));
    //                 if val_1 != 0 {
    //                     let val_2 = get_value(&memory, pointer + 2, *modes.get(1).unwrap_or(&0));
    //                     pointer = val_2;
    //                 } else {
    //                     pointer += 3;
    //                 }
    //             }
    //             6 => {
    //                 let val_1 = get_value(&memory, pointer + 1, *modes.get(0).unwrap_or(&0));
    //                 if val_1 == 0 {
    //                     let val_2 = get_value(&memory, pointer + 2, *modes.get(1).unwrap_or(&0));
    //                     pointer = val_2;
    //                 } else {
    //                     pointer += 3;
    //                 }
    //             }
    //             7 => {
    //                 let val_1 = get_value(&memory, pointer + 1, *modes.get(0).unwrap_or(&0));
    //                 let val_2 = get_value(&memory, pointer + 2, *modes.get(1).unwrap_or(&0));
    //                 let pos_1 = memory[pointer as usize + 3];
    //                 if val_1 < val_2 {
    //                     memory[pos_1 as usize] = 1;
    //                 } else {
    //                     memory[pos_1 as usize] = 0;
    //                 }
    //                 pointer += 4;
    //             }
    //             8 => {
    //                 let val_1 = get_value(&memory, pointer + 1, *modes.get(0).unwrap_or(&0));
    //                 let val_2 = get_value(&memory, pointer + 2, *modes.get(1).unwrap_or(&0));
    //                 let pos_1 = memory[pointer as usize + 3];
    //                 if val_1 == val_2 {
    //                     memory[pos_1 as usize] = 1;
    //                 } else {
    //                     memory[pos_1 as usize] = 0;
    //                 }
    //                 pointer += 4;
    //             }
    //             _ => return Err(AoCError::Solving),
    //         }
    //     }
    //     Ok(output)
    // }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut computer: Computer = Default::default();
        computer.set_memory(self.0.clone());
        computer.input(5);
        computer.run()?;
        computer
            .last_output()
            .ok_or(AoCError::Solving)
    }
}
