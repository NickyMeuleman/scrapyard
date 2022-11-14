use std::collections::HashMap;

use itertools::{Either, Itertools, MinMaxResult};

use crate::AoCData;

pub struct Data(Vec<Instruction>);

#[derive(Debug)]
enum Instruction {
    Start(u8, u8),                          // (chip_num, bot_num)
    Transfer(u8, Destination, Destination), // (chip_num, low_dest, high_dest)
}
#[derive(Debug)]
enum Destination {
    Output(u8),
    Bot(u8),
}

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        let mut instructions = Vec::new();
        for line in input.lines() {
            let instruction = match line.starts_with("value") {
                true => {
                    let rest = line.strip_prefix("value ")?;
                    let (value, bot) = rest.split_once(" goes to bot ")?;
                    Instruction::Start(value.parse().ok()?, bot.parse().ok()?)
                }
                false => {
                    let rest = line.strip_prefix("bot ")?;
                    let (bot, rest) = rest.split_once(" gives low to ")?;
                    let (low_dest, high_dest) = rest.split_once(" and high to ")?;
                    let low_dest = if let Some(low_dest) = low_dest.strip_prefix("bot ") {
                        Destination::Bot(low_dest.parse().ok()?)
                    } else {
                        let low_dest = low_dest.strip_prefix("output ")?;
                        Destination::Output(low_dest.parse().ok()?)
                    };
                    let high_dest = if let Some(high_dest) = high_dest.strip_prefix("bot ") {
                        Destination::Bot(high_dest.parse().ok()?)
                    } else {
                        let high_dest = high_dest.strip_prefix("output ")?;
                        Destination::Output(high_dest.parse().ok()?)
                    };
                    Instruction::Transfer(bot.parse().ok()?, low_dest, high_dest)
                }
            };
            instructions.push(instruction);
        }
        Some(Self(instructions))
    }

    fn part_1(&self) -> String {
        let mut state = HashMap::new();
        let (start_instructions, transfer_instructions): (Vec<_>, Vec<_>) =
            self.0.iter().partition_map(|ins| match ins {
                Instruction::Start(val, bot) => Either::Left((*val, *bot)),
                Instruction::Transfer(val, low, high) => Either::Right((*val, low, high)),
            });

        for (bot_num, _, _) in &transfer_instructions {
            state.insert(*bot_num, vec![]);
        }

        for (val, bot_num) in start_instructions {
            if let Some(chips) = state.get_mut(&bot_num) {
                chips.push(val);
            }
        }

        loop {
            for (bot_num, low_dest, high_dest) in &transfer_instructions {
                let chips = state.get_mut(bot_num).unwrap();
                if chips.len() == 2 {
                    if let MinMaxResult::MinMax(min, max) = chips.drain(..).minmax() {
                        if min == 17 && max == 61 {
                            return bot_num.to_string();
                        }
                        if let Destination::Bot(n) = low_dest {
                            state.entry(*n).and_modify(|chips| chips.push(min));
                        }
                        if let Destination::Bot(n) = high_dest {
                            state.entry(*n).and_modify(|chips| chips.push(max));
                        }
                    }
                }
            }
        }
    }

    fn part_2(&self) -> String {
        let mut state = HashMap::new();
        let mut outputs = [None; 3];
        let (start_instructions, transfer_instructions): (Vec<_>, Vec<_>) =
            self.0.iter().partition_map(|ins| match ins {
                Instruction::Start(val, bot) => Either::Left((*val, *bot)),
                Instruction::Transfer(val, low, high) => Either::Right((*val, low, high)),
            });

        for (bot_num, _, _) in &transfer_instructions {
            state.insert(*bot_num, vec![]);
        }

        for (val, bot_num) in start_instructions {
            if let Some(chips) = state.get_mut(&bot_num) {
                chips.push(val);
            }
        }

        loop {
            if let Some(vec) = outputs.into_iter().collect::<Option<Vec<u8>>>() {
                return vec
                    .iter()
                    .map(|n| u32::from(*n))
                    .product::<u32>()
                    .to_string();
            }
            for (bot_num, low_dest, high_dest) in &transfer_instructions {
                let chips = state.get_mut(bot_num).unwrap();
                if chips.len() == 2 {
                    if let MinMaxResult::MinMax(min, max) = chips.drain(..).minmax() {
                        match low_dest {
                            Destination::Bot(n) => {
                                state.entry(*n).and_modify(|chips| chips.push(min));
                            }
                            Destination::Output(n) if *n < 3 => {
                                outputs[*n as usize] = Some(min);
                            }
                            _ => {}
                        }
                        match high_dest {
                            Destination::Bot(n) => {
                                state.entry(*n).and_modify(|chips| chips.push(max));
                            }
                            Destination::Output(n) if *n < 3 => {
                                outputs[*n as usize] = Some(max);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(10);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), " ");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(10);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "");
    }
}
