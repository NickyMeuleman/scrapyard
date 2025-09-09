use std::{collections::HashSet, fmt::Display};

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data {
    ip_idx: usize,
    instructions: Vec<Ins>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Ins {
    op: Op,
    args: [u64; 3],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Op {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl Ins {
    fn execute(&self, registers: &mut [u64; 6]) {
        let a = self.args[0];
        let b = self.args[1];
        let c = self.args[2];
        let get_reg_val = |idx| registers[idx as usize];
        registers[c as usize] = match self.op {
            Op::Addr => get_reg_val(a) + get_reg_val(b),
            Op::Addi => get_reg_val(a) + b,
            Op::Mulr => get_reg_val(a) * get_reg_val(b),
            Op::Muli => get_reg_val(a) * b,
            Op::Banr => get_reg_val(a) & get_reg_val(b),
            Op::Bani => get_reg_val(a) & b,
            Op::Borr => get_reg_val(a) | get_reg_val(b),
            Op::Bori => get_reg_val(a) | b,
            Op::Setr => get_reg_val(a),
            Op::Seti => a,
            Op::Gtir => (a > get_reg_val(b)) as u64,
            Op::Gtri => (get_reg_val(a) > b) as u64,
            Op::Gtrr => (get_reg_val(a) > get_reg_val(b)) as u64,
            Op::Eqir => (a == get_reg_val(b)) as u64,
            Op::Eqri => (get_reg_val(a) == b) as u64,
            Op::Eqrr => (get_reg_val(a) == get_reg_val(b)) as u64,
        };
    }
}
// works, but slower
// fn sum_of_factors(n: u64) -> u64 {
//     (1..=(n as f64).sqrt() as u64)
//         .filter(|i| n.is_multiple_of(*i))
//         .flat_map(|i| if i * i == n { vec![i] } else { vec![i, n / i] })
//         .sum()
// }
//
// fn run(ip_idx: usize, instructions: &[Ins], part2: bool) -> u64 {
//     let mut registers = [0, 0, 0, 0, 0, 0];
//     let mut seen = HashSet::new();
//     let mut prev = 0;
//     while (registers[ip_idx] as usize) < instructions.len() {
//         let ip = registers[ip_idx] as usize;
//
//         if part2 && ip == 1 {
//             // This is the core factorization loop
//             registers[3] = sum_of_factors(registers[5]);
//             registers[ip_idx] = 26;
//             continue;
//         }
//
//         if ip == 28 {
//             let value = registers[instructions[ip].args[0] as usize];
//             if !part2 {
//                 return value;
//             }
//             if !seen.insert(value) {
//                 return prev;
//             }
//             prev = value;
//         }
//
//         instructions[ip].execute(&mut registers);
//         registers[ip_idx] += 1;
//     }
//
//     registers[0]
// }
//
// fn run(ip_idx: usize, instructions: &[Ins], part2: bool) -> u64 {
//     let mut registers = [0, 0, 0, 0, 0, 0];
//     let mut seen = HashSet::new();
//     let mut ip = registers[ip_idx] as usize;
//     let mut prev = 0;
//     while ip < instructions.len() {
//         // optimization: replace inner loop
//         if ip + 9 < instructions.len()
//             && instructions[ip].op == Op::Seti
//             && instructions[ip + 1].op == Op::Addi
//             && instructions[ip + 2].op == Op::Muli
//             && instructions[ip + 3].op == Op::Gtrr
//             && instructions[ip + 4].op == Op::Addr
//             && instructions[ip + 5].op == Op::Addi
//             && instructions[ip + 6].op == Op::Seti
//             && instructions[ip + 7].op == Op::Addi
//             && instructions[ip + 8].op == Op::Seti
//         {
//             let muli = &instructions[ip + 2];
//             let divide_by = muli.args[1];
//             let gtrr = &instructions[ip + 3];
//             let divided_reg = gtrr.args[1];
//             registers[divided_reg as usize] /= divide_by;
//
//             registers[ip_idx] = (ip + 9) as u64;
//             ip += 10;
//             continue;
//         }
//         // Halting check: detect condition where program ends
//         if instructions[ip].op == Op::Eqrr {
//             let idx = instructions[ip].args[0] as usize;
//             let value = registers[idx];
//             if !part2 {
//                 return value;
//             }
//             if !seen.insert(value) {
//                 return prev;
//             }
//             prev = value;
//         }
//         instructions[ip].execute(&mut registers);
//         registers[ip_idx] += 1;
//         ip = registers[ip_idx] as usize;
//     }
//
//     registers[0]
// }

fn run(ip_idx: usize, instructions: &[Ins], part2: bool) -> u64 {
    let mut registers = [0; 6];
    let mut seen = HashSet::new();
    let mut ip = 0;
    let mut prev = 0;

    while ip < instructions.len() {
        // Attempt to optimize the inner loop
        if let Some(next_ip) = try_optimize_division(ip, instructions, &mut registers) {
            registers[ip_idx] = next_ip as u64;
            ip = next_ip;
            continue;
        }

        // Halting condition detection
        if instructions[ip].op == Op::Eqrr {
            let idx = instructions[ip].args[0] as usize;
            let value = registers[idx];
            if !part2 {
                return value;
            }
            if !seen.insert(value) {
                return prev;
            }
            prev = value;
        }

        // Execute instruction
        instructions[ip].execute(&mut registers);
        registers[ip_idx] += 1;
        ip = registers[ip_idx] as usize;
    }

    registers[0]
}

fn try_optimize_division(
    ip: usize,
    instructions: &[Ins],
    registers: &mut [u64; 6],
) -> Option<usize> {
    const PATTERN: &[Op] = &[
        Op::Seti,
        Op::Addi,
        Op::Muli,
        Op::Gtrr,
        Op::Addr,
        Op::Addi,
        Op::Seti,
        Op::Addi,
        Op::Seti,
    ];

    let window = instructions.get(ip..ip + PATTERN.len())?;
    window
        .iter()
        .zip(PATTERN)
        .all(|(ins, &op)| ins.op == op)
        .then(|| {
            let divide_by = window[2].args[1];
            let divided_reg = window[3].args[1];
            registers[divided_reg as usize] /= divide_by;
            ip + PATTERN.len() + 1
        })
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let (ip_idx, instructions) = input.split_once("\n").unwrap();
        let (_, ip_idx) = ip_idx.rsplit_once(" ").unwrap();
        let ip_idx = ip_idx.parse().unwrap();
        let instructions = instructions
            .lines()
            .map(|line| {
                let (op, args) = line.split_once(" ").unwrap();
                let op = match op {
                    "addr" => Op::Addr,
                    "addi" => Op::Addi,
                    "mulr" => Op::Mulr,
                    "muli" => Op::Muli,
                    "banr" => Op::Banr,
                    "bani" => Op::Bani,
                    "borr" => Op::Borr,
                    "bori" => Op::Bori,
                    "setr" => Op::Setr,
                    "seti" => Op::Seti,
                    "gtir" => Op::Gtir,
                    "gtri" => Op::Gtri,
                    "gtrr" => Op::Gtrr,
                    "eqir" => Op::Eqir,
                    "eqri" => Op::Eqri,
                    "eqrr" => Op::Eqrr,
                    _ => panic!("at the disco"),
                };

                let args = args
                    .split(" ")
                    .map(|num| num.parse().unwrap())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();

                Ins { op, args }
            })
            .collect();
        Ok(Self {
            ip_idx,
            instructions,
        })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let res = run(self.ip_idx, &self.instructions, false);
        Ok(res)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let res = run(self.ip_idx, &self.instructions, true);
        Ok(res)
    }
}
