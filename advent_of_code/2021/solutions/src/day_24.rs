use std::{collections::HashMap, fmt::Display, str::FromStr};

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

// Find modelnum and solve logic from u/supersmurfen, thanks!
// https://github.com/AxlLind/AdventOfCode2021/blob/main/src/bin/24.rs
// that link now has the faster manual decompiling non-bruteforce solution

#[derive(Debug, Clone)]
pub struct Data {
    instructions: Vec<Instruction>,
}

#[derive(Debug, Clone)]
struct Alu {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

impl Alu {
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
    fn val(&self, alu: &Alu) -> i64 {
        match self {
            Register::X => alu.x,
            Register::Y => alu.y,
            Register::Z => alu.z,
            Register::W => alu.w,
        }
    }
}

impl FromStr for Register {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Register::X),
            "y" => Ok(Register::Y),
            "z" => Ok(Register::Z),
            "w" => Ok(Register::W),
            _ => Err(()),
        }
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
    fn val(&self, alu: &Alu) -> i64 {
        match self {
            Operand::Register(register) => register.val(alu),
            Operand::Val(num) => *num,
        }
    }
}

impl FromStr for Operand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(num) = s.parse() {
            Ok(Operand::Val(num))
        } else if let Ok(register) = s.parse() {
            Ok(Operand::Register(register))
        } else {
            Err(())
        }
    }
}

fn find_modelnum(
    memo: &mut HashMap<(i64, usize), Option<i64>>,
    blocks: &[Vec<Instruction>],
    block: usize,
    z: i64,
    range: &[i64; 9],
) -> Option<i64> {
    // early return if the memo has an entry
    // the memo object has a key of the final alu.z value and the current block
    // the entire alu-state does not have to be stored, only alu.z is significant
    if let Some(&answer) = memo.get(&(z, block)) {
        return answer;
    }

    for &digit in range {
        // initialize an ALU with w set to the current digit and z set to the z-parameter
        let mut alu = Alu::new(0, 0, z, digit);

        // execute all instructions in the current block
        for inst in &blocks[block] {
            alu.execute(inst)
        }

        // if entered: all instructions for a modelnumber were executed
        if block + 1 == blocks.len() {
            if alu.z == 0 {
                // the modelnumber was valid,add it to the memo and return the current digit
                memo.insert((alu.z, block), Some(digit));
                return Some(digit);
            } else {
                // the modelnumber wasn't valid, check the next one
                continue;
            }
        }

        // recurse using the next block of instructions and the z parameter set to the current z register in the ALU
        if let Some(best) = find_modelnum(memo, blocks, block + 1, alu.z, range) {
            // that had a result, add it to the memo
            // multiply the result by 10 and add the current digit to it (like concatenating a string, but with numbers)
            // this prepends the number, so we have to reverse the result once it's done
            memo.insert((alu.z, block), Some(best * 10 + digit));
            return Some(best * 10 + digit);
        }
    }

    // no result was found, add the relevant entry to the memo and return
    memo.insert((z, block), None);
    None
}

fn solve(blocks: &[Vec<Instruction>], biggest: bool) -> String {
    let range = if biggest {
        [9, 8, 7, 6, 5, 4, 3, 2, 1]
    } else {
        [1, 2, 3, 4, 5, 6, 7, 8, 9]
    };
    // the answer will have as much digits are there are blocks.
    // in this input size, that's 14, the length of a modelnumber
    let answer = find_modelnum(&mut HashMap::new(), blocks, 0, 0, &range).unwrap();

    answer
        .to_string()
        .chars()
        .rev()
        .collect()
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let instructions = input
            .trim()
            .lines()
            .map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                let instruction = *parts.get(0).ok_or(AoCError::Parsing)?;
                let register = parts
                    .get(1)
                    .ok_or(AoCError::Parsing)?
                    .parse()
                    .map_err(|_| AoCError::Parsing)?;

                match instruction {
                    "inp" => Ok(Instruction::Inp(register)),
                    _ => {
                        let operand = parts
                            .get(2)
                            .ok_or(AoCError::Parsing)?
                            .parse()
                            .map_err(|_| AoCError::Parsing)?;
                        match instruction {
                            "add" => Ok(Instruction::Add(register, operand)),
                            "mul" => Ok(Instruction::Mul(register, operand)),
                            "div" => Ok(Instruction::Div(register, operand)),
                            "mod" => Ok(Instruction::Mod(register, operand)),
                            "eql" => Ok(Instruction::Eql(register, operand)),
                            _ => Err(AoCError::Parsing),
                        }
                    }
                }
            })
            .collect::<AoCResult<Vec<_>>>()?;

        Ok(Self { instructions })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        // get blocks of instructions, one for each digit in the 14-digit number. Leave the input instruction off.
        let blocks: Vec<_> = self
            .instructions
            .chunks(18)
            .map(|c| c.iter().skip(1).copied().collect())
            .collect();

        Ok(solve(&blocks, true))
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let blocks: Vec<_> = self
            .instructions
            .chunks(18)
            .map(|c| c.iter().skip(1).copied().collect())
            .collect();

        Ok(solve(&blocks, false))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "inp w
mul x 0
add x z
mod x 26
div z 1
add x 10
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 12
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 10
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 10
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 12
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 8
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 11
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 4
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x 0
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 3
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 15
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 10
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 13
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 6
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -12
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 13
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -15
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 8
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -15
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 1
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -4
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 7
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 10
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 6
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -5
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 9
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -12
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 9
mul y x
add z y
";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "93959993429899");
    }

    #[test]
    fn part_2() {
        let input = "inp w
mul x 0
add x z
mod x 26
div z 1
add x 10
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 12
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 10
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 10
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 12
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 8
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 11
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 4
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x 0
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 3
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 15
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 10
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 13
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 6
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -12
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 13
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -15
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 8
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -15
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 1
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -4
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 7
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 10
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 6
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -5
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 9
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -12
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 9
mul y x
add z y
";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "11815671117121");
    }
}
