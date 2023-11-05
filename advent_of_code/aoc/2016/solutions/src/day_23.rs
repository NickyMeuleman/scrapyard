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
}

impl Cpu {
    fn new(regs: [i32; 4]) -> Self {
        Self {
            reg_a: regs[0],
            reg_b: regs[1],
            reg_c: regs[2],
            reg_d: regs[3],
            ins_pointer: 0,
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

    fn jnz(&mut self, val: &Val, offset: &Val) {
        let val = self.get_val(val);
        let offset = self.get_val(offset);
        if val != 0 {
            self.ins_pointer += offset;
        } else {
            self.ins_pointer += 1;
        }
    }

    fn tgl(&mut self, offset: &Val, instructions: &mut [Instruction]) {
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

    fn invalid(&mut self) {
        self.ins_pointer += 1;
    }

    fn execute(&mut self, ins: &Instruction, instructions: &mut [Instruction]) {
        match ins {
            Instruction::Cpy(val, reg) => self.cpy(val, reg),
            Instruction::Inc(reg) => self.inc(reg),
            Instruction::Dec(reg) => self.dec(reg),
            Instruction::Jnz(val, offset) => self.jnz(val, offset),
            Instruction::Tgl(offset) => self.tgl(offset, instructions),
            Instruction::Invalid => self.invalid(),
        }
    }

    fn run(&mut self, instructions: Vec<Instruction>) -> Option<()> {
        let mut instructions = instructions;
        loop {
            let index: usize = self.ins_pointer.try_into().ok()?;
            let ins = instructions.get(index)?.clone();
            self.execute(&ins, &mut instructions);
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
        let mut cpu = Cpu::new([7, 0, 0, 0]);
        cpu.run(self.0.clone());
        Ok(cpu.reg_a)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        // should probably find an optimization, but Rust is fast enough and not changing anything still has a tolerable runtime
        // ideas for optimizations: https://www.reddit.com/r/adventofcode/comments/5jvbzt/2016_day_23_solutions/
        let mut cpu = Cpu::new([12, 0, 0, 0]);
        cpu.run(self.0.clone());
        Ok(cpu.reg_a)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "3");
    }

    #[test]
    fn part_2() {
        let input = "cpy a b
dec b
cpy a d
cpy 0 a
cpy b c
inc a
dec c
jnz c -2
dec d
jnz d -5
dec b
cpy b c
cpy c d
dec d
inc c
jnz d -2
tgl c
cpy -16 c
jnz 1 c
cpy 74 c
jnz 88 d
inc a
inc d
jnz d -2
inc c
jnz c -5
";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "479008112");
    }
}
