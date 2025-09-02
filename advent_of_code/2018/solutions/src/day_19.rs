use std::fmt::Display;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data {
    ip_idx: usize,
    instructions: Vec<Ins>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Ins {
    op: Op,
    args: [u32; 3],
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
    fn execute(&self, registers: &mut [u32; 6]) {
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
            Op::Gtir => (a > get_reg_val(b)) as u32,
            Op::Gtri => (get_reg_val(a) > b) as u32,
            Op::Gtrr => (get_reg_val(a) > get_reg_val(b)) as u32,
            Op::Eqir => (a == get_reg_val(b)) as u32,
            Op::Eqri => (get_reg_val(a) == b) as u32,
            Op::Eqrr => (get_reg_val(a) == get_reg_val(b)) as u32,
        };
    }
}

fn run(ip_idx: usize, instructions: &[Ins], reg0: u32) -> u32 {
    let mut registers = [reg0, 0, 0, 0, 0, 0];

    let mut ip = registers[ip_idx] as usize;
    while ip < instructions.len() {
        instructions[ip].execute(&mut registers);
        registers[ip_idx] += 1;
        ip = registers[ip_idx] as usize;
    }

    registers[0]
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
        let res = run(self.ip_idx, &self.instructions, 0);
        Ok(res)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut registers = [1, 0, 0, 0, 0, 0];

        let mut ip = registers[self.ip_idx] as usize;
        // loop until ip is 1, at that point the initialization is finished
        while ip != 1 {
            self.instructions[ip].execute(&mut registers);
            registers[self.ip_idx] += 1;
            ip = registers[self.ip_idx] as usize;
        }

        // the program calculates factors for a number
        // the largest number in the registers after initialization is the one the program calculates factors for
        let num = *registers.iter().max().unwrap();

        // sum all factors of num
        // slower but easier to read
        // let mut total = 0;
        // for i in 1..=num {
        //     if num % i == 0 {
        //         total += i;
        //     }
        // }
        // faster, but harder to read
        let total: u32 = (1..=num)
            .take_while(|&i| i * i <= num)
            .filter_map(|i| {
                let is_factor = num % i == 0;
                is_factor.then(|| {
                    let other_factor = num / i;
                    if i == other_factor {
                        i
                    } else {
                        i + other_factor
                    }
                })
            })
            .sum();
        Ok(total)
    }
}
