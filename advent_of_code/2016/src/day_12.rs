use crate::AoCData;

pub struct Data(Vec<Instruction>);

enum Instruction {
    Cpy(Val, Register),
    Inc(Register),
    Dec(Register),
    Jnz(Val, i32),
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
                    let offset = offset.parse().ok()?;
                    Instruction::Jnz(val, offset)
                }
                _ => return None,
            }
        };
        Some(ins)
    }
}

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

    fn jnz(&mut self, val: &Val, offset: i32) {
        let val = self.get_val(val);
        if val != 0 {
            self.ins_pointer += offset;
        } else {
            self.ins_pointer += 1;
        }
    }

    fn execute(&mut self, ins: &Instruction) {
        match ins {
            Instruction::Cpy(val, reg) => self.cpy(val, reg),
            Instruction::Inc(reg) => self.inc(reg),
            Instruction::Dec(reg) => self.dec(reg),
            Instruction::Jnz(val, offset) => self.jnz(val, *offset),
        }
    }

    fn run(&mut self, instructions: &[Instruction]) -> Option<()> {
        loop {
            let index: usize = self.ins_pointer.try_into().ok()?;
            let ins = instructions.get(index)?;
            self.execute(ins);
        }
    }
}

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        let instructions = input
            .lines()
            .map(Instruction::try_new)
            .collect::<Option<Vec<Instruction>>>()?;
        Some(Self(instructions))
    }

    fn part_1(&self) -> String {
        let mut cpu = Cpu::new([0, 0, 0, 0]);
        cpu.run(&self.0);
        cpu.reg_a.to_string()
    }

    fn part_2(&self) -> String {
        let mut cpu = Cpu::new([0, 0, 1, 0]);
        cpu.run(&self.0);
        cpu.reg_a.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(12);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "42");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(12);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "42");
    }
}
