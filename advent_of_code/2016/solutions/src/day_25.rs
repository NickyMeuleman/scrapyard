use std::fmt::Display;

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<Instruction>);

#[derive(Debug, Clone)]
enum Instruction {
    Cpy(Val, Register),
    Inc(Register),
    Dec(Register),
    Jnz(Val, Val),
    Invalid,
    Tgl(Val),
    Out(Val),
}

impl Instruction {
    fn try_new(s: &str) -> Option<Self> {
        let (kind, rest) = s.split_once(' ')?;
        let ins = {
            match kind {
                "cpy" => {
                    let (val, reg) = rest.split_once(' ')?;
                    let val = Val::try_new(val)?;
                    let reg = Register::try_new(reg)?;
                    Instruction::Cpy(val, reg)
                }
                "inc" => {
                    let reg = Register::try_new(rest)?;
                    Instruction::Inc(reg)
                }
                "dec" => {
                    let reg = Register::try_new(rest)?;
                    Instruction::Dec(reg)
                }
                "jnz" => {
                    let (val, offset) = rest.split_once(' ')?;
                    let val = Val::try_new(val)?;
                    let offset = Val::try_new(offset)?;
                    Instruction::Jnz(val, offset)
                }
                "tgl" => {
                    let val = Val::try_new(rest)?;
                    Instruction::Tgl(val)
                }
                "out" => {
                    let val = Val::try_new(rest)?;
                    Instruction::Out(val)
                }
                _ => return None,
            }
        };
        Some(ins)
    }
}

#[derive(Debug, Clone, Copy)]
enum Register {
    A,
    B,
    C,
    D,
}

impl Register {
    fn try_new(s: &str) -> Option<Self> {
        let reg = match s {
            "a" => Register::A,
            "b" => Register::B,
            "c" => Register::C,
            "d" => Register::D,
            _ => return None,
        };
        Some(reg)
    }
}

#[derive(Debug, Clone, Copy)]
enum Val {
    Literal(i32),
    Register(Register),
}

impl Val {
    fn try_new(s: &str) -> Option<Self> {
        let val = match s {
            "a" | "b" | "c" | "d" => {
                let reg = Register::try_new(s)?;
                Val::Register(reg)
            }
            _ => {
                let num = s.parse().ok()?;
                Val::Literal(num)
            }
        };
        Some(val)
    }
}

#[derive(Debug, Clone)]
struct Cpu {
    reg_a: i32,
    reg_b: i32,
    reg_c: i32,
    reg_d: i32,
    ins_pointer: i32,
    out: i32,
}

impl Cpu {
    fn new(regs: [i32; 4]) -> Self {
        Self {
            reg_a: regs[0],
            reg_b: regs[1],
            reg_c: regs[2],
            reg_d: regs[3],
            ins_pointer: 0,
            out: 2, // any value other than 0 or 1
        }
    }

    fn get_val(&self, val: &Val) -> i32 {
        match val {
            Val::Literal(num) => *num,
            Val::Register(reg) => self.get_reg(reg),
        }
    }

    fn get_reg(&self, reg: &Register) -> i32 {
        match reg {
            Register::A => self.reg_a,
            Register::B => self.reg_b,
            Register::C => self.reg_c,
            Register::D => self.reg_d,
        }
    }

    fn set_reg(&mut self, reg: &Register, num: i32) {
        match reg {
            Register::A => self.reg_a = num,
            Register::B => self.reg_b = num,
            Register::C => self.reg_c = num,
            Register::D => self.reg_d = num,
        }
    }

    fn cpy(&mut self, val: &Val, reg: &Register) {
        let num = self.get_val(val);
        self.set_reg(reg, num);
        self.ins_pointer += 1;
    }

    fn inc(&mut self, reg: &Register) {
        let num = self.get_reg(reg) + 1;
        self.set_reg(reg, num);
        self.ins_pointer += 1;
    }

    fn dec(&mut self, reg: &Register) {
        let num = self.get_reg(reg) - 1;
        self.set_reg(reg, num);
        self.ins_pointer += 1;
    }

    fn out(&mut self, val: &Val) {
        let val = self.get_val(val);
        self.out = val;
        self.ins_pointer += 1;
    }

    fn jnz(&mut self, val: &Val, offset: &Val) {
        let val = self.get_val(val);
        let offset = self.get_val(offset);
        if val != 0 {
            self.ins_pointer += offset;
        } else {
            self.ins_pointer += 1;
        }
    }

    fn execute(&mut self, ins: &Instruction, instructions: &mut [Instruction]) {
        match ins {
            Instruction::Cpy(val, reg) => self.cpy(val, reg),
            Instruction::Inc(reg) => self.inc(reg),
            Instruction::Dec(reg) => self.dec(reg),
            Instruction::Jnz(val, offset) => self.jnz(val, offset),
            Instruction::Tgl(offset) => {
                let offset = self.get_val(offset);
                let pointer = usize::try_from(self.ins_pointer).unwrap();
                if let Ok(offset) = usize::try_from(offset) {
                    if let Some(ins) = instructions.get_mut(pointer + offset) {
                        match ins {
                            Instruction::Inc(reg) => {
                                *ins = Instruction::Dec(*reg);
                            }
                            Instruction::Dec(reg) => {
                                *ins = Instruction::Inc(*reg);
                            }
                            Instruction::Out(val) => match val {
                                Val::Register(reg) => *ins = Instruction::Inc(*reg),
                                Val::Literal(_) => *ins = Instruction::Invalid,
                            },
                            Instruction::Tgl(val) => match val {
                                Val::Register(reg) => *ins = Instruction::Inc(*reg),
                                Val::Literal(_) => *ins = Instruction::Invalid,
                            },
                            Instruction::Jnz(val, offset) => match offset {
                                Val::Literal(_) => *ins = Instruction::Invalid,
                                Val::Register(reg) => *ins = Instruction::Cpy(*val, *reg),
                            },
                            Instruction::Cpy(val, reg) => {
                                *ins = Instruction::Jnz(*val, Val::Register(*reg))
                            }
                            Instruction::Invalid => {}
                        }
                    }
                }
                self.ins_pointer += 1;
            }
            Instruction::Out(val) => self.out(val),
            Instruction::Invalid => {
                self.ins_pointer += 1;
            }
        }
    }

    fn run(&mut self, instructions: Vec<Instruction>) -> Option<i32> {
        let mut instructions = instructions;
        let starting_a = self.reg_a;
        let mut count = 0;
        loop {
            let index: usize = self.ins_pointer.try_into().ok()?;
            let ins = instructions.get(index)?.clone();
            let prev = self.out;
            self.execute(&ins, &mut instructions);
            if let Instruction::Out(_) = ins {
                let curr = self.out;
                // all these early returns can become a big one
                // they're seperate because that's nice and understandable to read
                if count == 0 && curr != 0 {
                    return None;
                }
                if prev == curr {
                    return None;
                }
                if curr < 0 {
                    return None;
                }
                if curr > 1 {
                    return None;
                }
                if count > 1000 {
                    // maybe infinite loop?
                    return Some(starting_a);
                }
                count += 1
            }
        }
    }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let instructions = input
            .lines()
            .map(Instruction::try_new)
            .collect::<Option<Vec<Instruction>>>()
            .ok_or(AoCError::Parsing)?;
        Ok(Self(instructions))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        // if an infinite loop begins, it's the solution
        // this is an .... euhm ... problem.
        // a ...halting... problem: https://en.wikipedia.org/wiki/Halting_problem
        for i in 0..10_000 {
            let mut cpu = Cpu::new([i, 0, 0, 0]);
            if let Some(loops_at) = cpu.run(self.0.clone()) {
                return Ok(loops_at);
            }
        }
        Err(AoCError::Solving)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        Ok("Suddenly, you see the sleigh fly past you!")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "cpy a d
cpy 4 c
cpy 633 b
inc d
dec b
jnz b -2
dec c
jnz c -5
cpy d a
jnz 0 0
cpy a b
cpy 0 a
cpy 2 c
jnz b 2
jnz 1 6
dec b
dec c
jnz c -4
inc a
jnz 1 -7
cpy 2 b
jnz c 2
jnz 1 4
dec b
dec c
jnz 1 -4
jnz 0 0
out b
jnz a -19
jnz 1 -21";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "198");
    }
}
