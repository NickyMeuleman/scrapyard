use crate::AoCData;
use std::str::FromStr;
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" | "a," => Ok(Self::A),
            "b" | "b," => Ok(Self::B),
            _ => panic!("Unexpected register: {s}"),
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
}

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        let mut instructions = Vec::new();
        for line in input.lines() {
            let mut toks = line.split_whitespace();
            let ins = match toks.next()? {
                ins @ ("hlf" | "tpl" | "inc") => {
                    let reg = toks.next()?.parse().ok()?;
                    match ins {
                        "hlf" => Instruction::Half(reg),
                        "tpl" => Instruction::Triple(reg),
                        "inc" => Instruction::Increment(reg),
                        _ => unreachable!(),
                    }
                }
                "jmp" => {
                    let offset = toks.next()?.parse().ok()?;
                    Instruction::Jump(offset)
                }
                ins @ ("jie" | "jio") => {
                    let reg = toks.next()?.parse().ok()?;
                    let offset = toks.next()?.parse().ok()?;
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
        Some(Self(instructions))
    }

    fn part_1(&self) -> String {
        let mut pointer = 0;
        let instructions = &self.0;
        let mut computer = Computer {
            reg_a: 0,
            reg_b: 0,
        };
        while pointer < instructions.len() {
            let instruction = instructions[pointer];
            pointer = computer.execute(&instruction, pointer);
        }
        computer.reg_b.to_string()
    }

    fn part_2(&self) -> String {
        let mut pointer = 0;
        let instructions = &self.0;
        let mut computer = Computer {
            reg_a: 1,
            reg_b: 0,
        };
        while pointer < instructions.len() {
            let instruction = instructions[pointer];
            pointer = computer.execute(&instruction, pointer);
        }
        computer.reg_b.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(23);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(23);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "");
    }
}
