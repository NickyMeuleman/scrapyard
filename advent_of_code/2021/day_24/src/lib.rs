use std::collections::HashMap;
use std::{convert::Infallible, str::FromStr};

#[derive(Debug, Clone)]
pub struct Data {
    instructions: Vec<Instruction>,
}

impl Data {
    pub fn part_one(&self) -> String {
        let blocks: Vec<_> = self
            .instructions
            .chunks(18)
            .map(|c| c.iter().skip(1).copied().collect())
            .collect();
        solve(&blocks, true)
    }

    pub fn part_two(&self) -> String {
        let blocks: Vec<_> = self
            .instructions
            .chunks(18)
            .map(|c| c.iter().skip(1).copied().collect())
            .collect();
        solve(&blocks, false)
    }
}

impl FromStr for Data {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let instructions = input
            .trim()
            .lines()
            .map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                let instruction = parts[0];
                let register = parts[1].parse().unwrap();

                match instruction {
                    "inp" => Instruction::Inp(register),
                    _ => {
                        let operand = parts[2].parse().unwrap();
                        match instruction {
                            "add" => Instruction::Add(register, operand),
                            "mul" => Instruction::Mul(register, operand),
                            "div" => Instruction::Div(register, operand),
                            "mod" => Instruction::Mod(register, operand),
                            "eql" => Instruction::Eql(register, operand),
                            _ => unreachable!(),
                        }
                    }
                }
            })
            .collect();

        Ok(Self { instructions })
    }
}

#[derive(Debug, Clone)]
struct ALU {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

impl ALU {
    fn new(x: i64, y: i64, z: i64, w: i64) -> Self {
        Self { x, y, z, w }
    }

    // if we used the inp instruction in the solution, the implementation would look like this.
    // I made the ALU before coding the solution, so that's why it exists at all
    // fn inp(&mut self, register: &Register, num: i64) {
    //     match register {
    //         Register::X => self.x = num,
    //         Register::Y => self.y = num,
    //         Register::Z => self.z = num,
    //         Register::W => self.w = num,
    //     }
    // }

    fn execute(&mut self, instruction: &Instruction) {
        // get the register to store in, and the result of the operation
        let (register, num) = match instruction {
            Instruction::Inp(_) => unreachable!("can not take an input"),
            Instruction::Add(register, operand) => {
                let a = register.val(self);
                let b = operand.val(self);
                (register, a + b)
            }
            Instruction::Mul(register, operand) => {
                let a = register.val(self);
                let b = operand.val(self);
                (register, a * b)
            }
            Instruction::Div(register, operand) => {
                let a = register.val(self);
                let b = operand.val(self);
                (register, a / b)
            }
            Instruction::Mod(register, operand) => {
                let a = register.val(self);
                let b = operand.val(self);
                (register, a % b)
            }
            Instruction::Eql(register, operand) => {
                let a = register.val(self);
                let b = operand.val(self);
                (register, if a == b { 1 } else { 0 })
            }
        };
        // store num in register
        match register {
            Register::X => self.x = num,
            Register::Y => self.y = num,
            Register::Z => self.z = num,
            Register::W => self.w = num,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Register {
    X,
    Y,
    Z,
    W,
}

impl Register {
    fn val(&self, alu: &ALU) -> i64 {
        match self {
            Register::X => alu.x,
            Register::Y => alu.y,
            Register::Z => alu.z,
            Register::W => alu.w,
        }
    }
}

impl FromStr for Register {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = match s {
            "x" => Register::X,
            "y" => Register::Y,
            "z" => Register::Z,
            "w" => Register::W,
            _ => unreachable!(),
        };

        Ok(result)
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Inp(Register),
    Add(Register, Operand),
    Mul(Register, Operand),
    Div(Register, Operand),
    Mod(Register, Operand),
    Eql(Register, Operand),
}

#[derive(Debug, Clone, Copy)]
enum Operand {
    Register(Register),
    Val(i64),
}

impl Operand {
    fn val(&self, alu: &ALU) -> i64 {
        match self {
            Operand::Register(register) => register.val(alu),
            Operand::Val(num) => *num,
        }
    }
}

impl FromStr for Operand {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = if let Ok(num) = s.parse() {
            Operand::Val(num)
        } else {
            let register = s.parse().unwrap();
            Operand::Register(register)
        };

        Ok(result)
    }
}

fn find_modelnum(
    memo: &mut HashMap<(i64, usize), Option<i64>>,
    blocks: &[Vec<Instruction>],
    block: usize,
    z: i64,
    range: &[i64; 9],
) -> Option<i64> {
    if let Some(&answer) = memo.get(&(z, block)) {
        return answer;
    }

    for &digit in range {
        let mut alu = ALU::new(0, 0, z, digit);
        for inst in &blocks[block] {
            alu.execute(inst)
        }

        if block + 1 == blocks.len() {
            if alu.z == 0 {
                memo.insert((alu.z, block), Some(digit));
                return Some(digit);
            }
            continue;
        }

        if let Some(best) = find_modelnum(memo, blocks, block + 1, alu.z, range) {
            memo.insert((alu.z, block), Some(best * 10 + digit));
            return Some(best * 10 + digit);
        }
    }

    memo.insert((z, block), None);
    None
}

fn solve(blocks: &[Vec<Instruction>], biggest: bool) -> String {
    let range = if biggest {
        [9, 8, 7, 6, 5, 4, 3, 2, 1]
    } else {
        [1, 2, 3, 4, 5, 6, 7, 8, 9]
    };
    let answer = find_modelnum(&mut HashMap::new(), &blocks, 0, 0, &range).unwrap();
    answer.to_string().chars().rev().collect()
}
