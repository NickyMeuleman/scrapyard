use std::fmt::Display;

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

struct Lens<'a> {
    label: &'a str,
    focal: u8,
}

enum Instruction<'a> {
    Remove(&'a str),
    Add(Lens<'a>),
}

impl<'a> Instruction<'a> {
    fn try_new(s: &'a str) -> AoCResult<Self> {
        if let Some(label) = s.strip_suffix('-') {
            Ok(Self::Remove(label))
        } else {
            let (label, focal) = s
                .split_once('=')
                .ok_or(AoCError::Parsing)?;
            let focal = focal
                .parse()
                .map_err(|_| AoCError::Parsing)?;
            let lens = Lens { label, focal };
            Ok(Self::Add(lens))
        }
    }
}

// https://www.khanacademy.org/computing/computer-science/cryptography/modarithmetic/a/modular-addition-and-subtraction
// (A + B) mod C = (A mod C + B mod C) mod C
// https://www.khanacademy.org/computing/computer-science/cryptography/modarithmetic/a/modular-multiplication
// (A * B) mod C = (A mod C * B mod C) mod C
// combining those two rules:
// ((A + B) * C) mod D = (((A + B) mod D) * C) mod D
fn hash(s: &str) -> u8 {
    s.bytes()
        .fold(0, |acc, byte| acc.wrapping_add(byte).wrapping_mul(17))
}

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let sum: u32 = self
            .0
            .trim()
            .split(',')
            .map(|s| hash(s) as u32)
            .sum();
        Ok(sum)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        const BOX: Vec<Lens> = Vec::new();
        let mut boxes = [BOX; 256];

        for instr in self
            .0
            .trim_end()
            .split(',')
            .map(Instruction::try_new)
        {
            match instr? {
                Instruction::Remove(label) => {
                    let hash = hash(label);
                    boxes[hash as usize].retain(|item| item.label != label);
                }
                Instruction::Add(lens) => {
                    let hash = hash(lens.label);
                    let lenses = &mut boxes[hash as usize];
                    if let Some(old) = lenses
                        .iter_mut()
                        .find(|item| lens.label == item.label)
                    {
                        // update focal length of lens with this label
                        old.focal = lens.focal;
                    } else {
                        // add lens to end of box
                        lenses.push(lens);
                    }
                }
            }
        }

        let sum: usize = boxes
            .into_iter()
            .enumerate()
            .map(|(box_idx, lenses)| {
                let box_focusing_power: usize = lenses
                    .into_iter()
                    .enumerate()
                    .map(|(lens_idx, lens)| (box_idx + 1) * (lens_idx + 1) * lens.focal as usize)
                    .sum();
                box_focusing_power
            })
            .sum();

        Ok(sum)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "1320");
    }

    #[test]
    fn part_2() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "145");
    }
}
