use std::{fmt::Display, str::FromStr};

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<Instruction>);

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(isize),
    JumpIfEven(Register, isize),
    JumpIfOne(Register, isize),
}

#[derive(Debug, Clone, Copy)]
enum Register {
    A,
    B,
}

impl FromStr for Register {
    type Err = AoCError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" | "a," => Ok(Self::A),
            "b" | "b," => Ok(Self::B),
            _ => return Err(AoCError::Parsing),
        }
    }
}

struct Computer {
    reg_a: u32,
    reg_b: u32,
}

impl Computer {
    fn get_reg(&self, reg: &Register) -> u32 {
        match reg {
            Register::A => self.reg_a,
            Register::B => self.reg_b,
        }
    }

    fn set_reg(&mut self, reg: &Register, val: u32) {
        match reg {
            Register::A => self.reg_a = val,
            Register::B => self.reg_b = val,
        }
    }

    //executes an instruction and returns new position of pointer
    fn execute(&mut self, ins: &Instruction, pointer: usize) -> usize {
        match ins {
            Instruction::Half(reg) => {
                let new = self.get_reg(reg) / 2;
                self.set_reg(reg, new);
                pointer + 1
            }
            Instruction::Triple(reg) => {
                let new = self.get_reg(reg) * 3;
                self.set_reg(reg, new);
                pointer + 1
            }
            Instruction::Increment(reg) => {
                let new = self.get_reg(reg) + 1;
                self.set_reg(reg, new);
                pointer + 1
            }
            Instruction::Jump(offset) => {
                // can overflow
                (pointer as isize + offset) as usize
            }
            Instruction::JumpIfEven(reg, offset) => {
                if self.get_reg(reg) % 2 == 0 {
                    // can overflow
                    (pointer as isize + offset) as usize
                } else {
                    pointer + 1
                }
            }
            Instruction::JumpIfOne(reg, offset) => {
                if self.get_reg(reg) == 1 {
                    // can overflow
                    (pointer as isize + offset) as usize
                } else {
                    pointer + 1
                }
            }
        }
    }

    fn run(&mut self, instructions: &[Instruction]) {
        let mut pointer = 0;
        while pointer < instructions.len() {
            let instruction = instructions[pointer];
            pointer = self.execute(&instruction, pointer);
        }
    }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut instructions = Vec::new();
        for line in input.lines() {
            let mut toks = line.split_whitespace();
            let ins = match toks.next().ok_or(AoCError::Parsing)? {
                ins @ ("hlf" | "tpl" | "inc") => {
                    let reg = toks
                        .next()
                        .ok_or(AoCError::Parsing)?
                        .parse()?;
                    match ins {
                        "hlf" => Instruction::Half(reg),
                        "tpl" => Instruction::Triple(reg),
                        "inc" => Instruction::Increment(reg),
                        _ => unreachable!(),
                    }
                }
                "jmp" => {
                    let offset = toks
                        .next()
                        .ok_or(AoCError::Parsing)?
                        .parse()?;
                    Instruction::Jump(offset)
                }
                ins @ ("jie" | "jio") => {
                    let reg = toks
                        .next()
                        .ok_or(AoCError::Parsing)?
                        .parse()?;
                    let offset = toks
                        .next()
                        .ok_or(AoCError::Parsing)?
                        .parse()?;
                    match ins {
                        "jie" => Instruction::JumpIfEven(reg, offset),
                        "jio" => Instruction::JumpIfOne(reg, offset),
                        _ => unreachable!(),
                    }
                }
                _ => panic!("invalid input"),
            };
            instructions.push(ins);
        }
        Ok(Self(instructions))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut computer = Computer { reg_a: 0, reg_b: 0 };
        computer.run(&self.0);

        Ok(computer.reg_b)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut computer = Computer { reg_a: 1, reg_b: 0 };
        computer.run(&self.0);

        Ok(computer.reg_b)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "inc a
jio a, +2
tpl a
inc a";
        let data = Data::try_new(input).unwrap();
        let mut computer = Computer { reg_a: 0, reg_b: 0 };
        computer.run(&data.0);
        assert_eq!(computer.reg_a, 2);
    }

    #[test]
    fn part_2() {
        let input = "jio a, +22
inc a
tpl a
tpl a
tpl a
inc a
tpl a
inc a
tpl a
inc a
inc a
tpl a
inc a
inc a
tpl a
inc a
inc a
tpl a
inc a
inc a
tpl a
jmp +19
tpl a
tpl a
tpl a
tpl a
inc a
inc a
tpl a
inc a
tpl a
inc a
inc a
tpl a
inc a
inc a
tpl a
inc a
tpl a
tpl a
jio a, +8
inc b
jie a, +4
tpl a
inc a
jmp +2
hlf a
jmp -7
";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "334");
    }
}
