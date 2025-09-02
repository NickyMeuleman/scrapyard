use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use itertools::Itertools;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Sample {
    before: [u32; 4],
    instruction: [u32; 4],
    after: [u32; 4],
}

impl Sample {
    fn is_possible_op(&self, op: &Op) -> bool {
        let mut registers = self.before;
        op.execute(&self.instruction, &mut registers);
        registers == self.after
    }
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

impl Op {
    fn all() -> [Op; 16] {
        [
            Op::Addr,
            Op::Addi,
            Op::Mulr,
            Op::Muli,
            Op::Banr,
            Op::Bani,
            Op::Borr,
            Op::Bori,
            Op::Setr,
            Op::Seti,
            Op::Gtir,
            Op::Gtri,
            Op::Gtrr,
            Op::Eqir,
            Op::Eqri,
            Op::Eqrr,
        ]
    }

    fn execute(&self, instruction: &[u32; 4], registers: &mut [u32; 4]) {
        let a = instruction[1];
        let b = instruction[2];
        let c = instruction[3] as usize;
        let get_reg_val = |idx: u32| registers[idx as usize];
        registers[c] = match self {
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

#[derive(Debug, Clone)]
pub struct Data {
    samples: Vec<Sample>,
    program: Vec<[u32; 4]>,
}

fn parse_registers(input: &str, prefix: &str) -> [u32; 4] {
    let input = input.strip_prefix(prefix).unwrap();
    let input = input.strip_suffix("]").unwrap();
    parse_instruction(input, ", ")
}

fn parse_instruction(input: &str, sep: &str) -> [u32; 4] {
    input
        .split(sep)
        .map(|num| num.parse().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let (samples, program) = input.split_once("\n\n\n\n").unwrap();
        let samples: Vec<Sample> = samples
            .split("\n\n")
            .map(|sample| {
                let Some((before, instruction, after)) = sample.split("\n").collect_tuple() else {
                    panic!();
                };
                Sample {
                    before: parse_registers(before, "Before: ["),
                    instruction: parse_instruction(instruction, " "),
                    after: parse_registers(after, "After:  ["),
                }
            })
            .collect();
        let program = program
            .lines()
            .map(|line| parse_instruction(line, " "))
            .collect();
        Ok(Self { samples, program })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        Ok(self
            .samples
            .iter()
            .filter(|sample| {
                Op::all()
                    .iter()
                    .filter(|op| sample.is_possible_op(op))
                    .count()
                    >= 3
            })
            .count())
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut candidates: HashMap<u32, HashSet<Op>> = HashMap::new();
        for sample in &self.samples {
            let entry = candidates
                .entry(sample.instruction[0])
                .or_insert(Op::all().iter().copied().collect());
            entry.retain(|&op| sample.is_possible_op(&op));
        }
        let mut opmap= HashMap::new();
        while opmap.len() < candidates.len() {
            let singles: Vec<(u32, Op)> = candidates
                .iter()
                .filter_map(|(&num, ops)| {
                    if ops.len() == 1 {
                        Some((num, *ops.iter().next().unwrap()))
                    } else {
                        None
                    }
                })
                .collect();
            for (num, op) in singles {
                opmap.insert(num, op);
                for ops in candidates.values_mut() {
                    ops.remove(&op);
                }
            }
        }

        let mut registers = [0; 4];
        for instruction in &self.program {
            let op = opmap.get(&instruction[0]).unwrap();
            op.execute(instruction, &mut registers);
        }
        Ok(registers[0])
    }
}
