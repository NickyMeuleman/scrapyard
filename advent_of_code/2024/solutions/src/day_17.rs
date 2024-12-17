// Blog writeup with simpler Rust code (I should handle errors here):
// https://nickymeuleman.netlify.app/blog/aoc2024-day17/

use crate::{AoCData, AoCResult};
use aoc_core::AoCError;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Data(Vec<u64>);

struct Computer<'a> {
    ip: usize,
    a: u64,
    b: u64,
    c: u64,
    program: &'a [u64],
}

impl Computer<'_> {
    fn combo(&self, operand: u64) -> Result<u64, AoCError> {
        match operand {
            0..=3 => Ok(operand),
            4 => Ok(self.a),
            5 => Ok(self.b),
            6 => Ok(self.c),
            _ => Err(AoCError::Solving),
        }
    }

    fn run(&mut self) -> Result<Option<u64>, AoCError> {
        while self.ip < self.program.len() {
            let opcode = self.program[self.ip];
            let operand = self.program[self.ip + 1];
            self.ip += 2;

            match opcode {
                // adv: a/2^combo, store in a
                0 => self.a >>= self.combo(operand)?,
                // bxl: bitwise xor of b and literal-operand, store in b
                1 => self.b ^= operand,
                // bst: combo % 8, store in b
                2 => self.b = self.combo(operand)? % 8,
                // jnz: jump to literal operand if a not zero
                3 => {
                    if self.a != 0 {
                        self.ip = operand as usize;
                        continue;
                    }
                }
                // bxc: bitwise xor of b and c, store in b (ignores operand)
                4 => self.b ^= self.c,
                // out: combo % 8, outputs result
                5 => {
                    let out = self.combo(operand)? % 8;
                    return Ok(Some(out));
                }
                // bdv: a/2^combo, store in b
                6 => self.b = self.a >> self.combo(operand)?,
                // cdv: a/2^combo, store in c
                7 => self.c = self.a >> self.combo(operand)?,
                _ => return Err(AoCError::Solving),
            }
        }

        Ok(None)
    }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        Ok(Self(
            input
                .split(|c: char| !c.is_ascii_digit())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect(),
        ))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut computer = Computer {
            ip: 0,
            a: self.0[0],
            b: self.0[1],
            c: self.0[2],
            program: &self.0[3..],
        };
        let mut out = Vec::new();

        while let Some(n) = computer.run()? {
            out.push(n);
        }

        Ok(out
            .into_iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(","))
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let program = &self.0[3..];
        // a list of all valid values for a for the current step that causes the correct next number in
        // the program being output.
        // starts with [0] because a must be 0 at the end for the program to end
        // reason: at the end of the input there is a jnz instruction that jumps back to ip 0
        // (that means your program ends in 3,0)
        let mut valid = vec![0];

        // work backwards through the program to recreate it using known valid values for a that
        // produce the correct next output number in the program
        for &wanted in program.iter().rev() {
            let mut curr_valid = Vec::new();

            for valid_next_a in valid {
                // try all possible values n that recreate the new a, so the valid_next_a << 3 operation still has the same result
                for n in 0..=7 {
                    // the value in the a register is only changed once, it is divided by 2^3 (or
                    // shifted left by 3)
                    // shift v 3 to the left, then add the n to get the original value for a
                    let a = (valid_next_a << 3) | n;
                    let mut computer = Computer {
                        ip: 0,
                        a,
                        // both b and c are set from the value in a so their starting value does not
                        // matter
                        b: 0,
                        c: 0,
                        program,
                    };

                    // if this program outputs the previous value in the program, the value for a was
                    // correct
                    if let Some(result) = computer.run()? {
                        if result == wanted {
                            curr_valid.push(a);
                        }
                    }
                }
            }

            valid = curr_valid;
        }

        // the program is in the starting state now, so valid holds a list of valid numbers for the a
        // register that output the entire program at this point
        valid
            .into_iter()
            .min()
            .ok_or(AoCError::Solving)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part_2() {
        let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "117440");
    }
}
