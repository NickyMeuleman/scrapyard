use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<Command>);

#[derive(Debug, Clone)]
enum Command {
    Mem((u64, u64)),
    Mask(String),
}

fn apply_mask(mask: &str, val: u64) -> u64 {
    // step 1: keep everything in the X positions
    // intermediary_result = val OR mask with X replaced with 0
    // step 2: overwrite everything with the given value
    // result = intermediary_result AND mask with X replaced with 1
    let zero_mask = mask.replace("X", "0");
    let zero_mask = u64::from_str_radix(&zero_mask, 2).unwrap();
    let one_mask = mask.replace("X", "1");
    let one_mask = u64::from_str_radix(&one_mask, 2).unwrap();
    let intermediary_result = val | zero_mask;
    intermediary_result & one_mask
}

fn apply_mask_part_two(mask: &str, val: u64) -> Vec<Option<u8>> {
    // return vector with Some(1) for 1, Some(0) for 0, or None for X
    let mut mask_vec = Vec::new();
    for c in mask.chars() {
        match c {
            '1' => mask_vec.push(Some(1)),
            '0' => mask_vec.push(Some(0)),
            'X' => mask_vec.push(None),
            _ => panic!("invalid input to get_result_part_2"),
        }
    }
    let mut result = Vec::new();
    // apply that mask to the val;
    for idx in 0..mask_vec.len() {
        let mask_bit = mask_vec[idx];
        match mask_bit {
            None => result.push(None),
            Some(1) => result.push(Some(1)),
            Some(0) => {
                // turn val into a binary representation String and pad with 0s
                let binary_str = format!("{:036b}", val);
                let bit = &binary_str[idx..idx + 1];
                let bit = bit.parse().unwrap();
                result.push(Some(bit));
            }
            _ => panic!("invalid number found in binary"),
        }
    }
    result
}

fn get_options(input: Vec<Option<u8>>) -> Vec<u64> {
    // use HashSet instead, there are duplicates
    let mut result: Vec<u64> = Vec::new();
    // return a vec of every possible resulting binary number parsed to a decimal digit
    if input.iter().any(|&item| item == None) {
        // for every None, change to a 0 or a 1, repeat until no None left
        let none_idx = input
            .iter()
            .position(|&item| item == None)
            .unwrap();

        let mut floating_as_zero = input.clone();
        floating_as_zero[none_idx] = Some(0);
        let mut options_for_0 = get_options(floating_as_zero);
        result.append(&mut options_for_0);

        let mut floating_as_one = input.clone();
        floating_as_one[none_idx] = Some(1);
        let mut options_for_1 = get_options(floating_as_one);
        result.append(&mut options_for_1);
    } else {
        // turn binary Vec into binary string
        let input_string = input
            .iter()
            .fold(String::new(), |mut acc, item| {
                let bit = item.unwrap();
                let bit = format!("{}", bit);
                acc.push(bit.chars().next().unwrap());
                acc
            });
        // turn binary string into u64
        let num = u64::from_str_radix(&input_string, 2).unwrap();
        // push u64 into result
        result.push(num);
    }
    result
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut result: Vec<Command> = Vec::new();
        let lines: Vec<&str> = input.lines().collect();
        for line in lines {
            let (instruction, value): (&str, &str) = line
                .splitn(2, " = ")
                .collect_tuple()
                .unwrap();
            match instruction {
                "mask" => result.push(Command::Mask(value.to_owned())),
                _ => {
                    let address = &instruction[4..instruction.len() - 1];
                    let address = address.parse().unwrap();
                    let value = value.parse().unwrap();
                    result.push(Command::Mem((address, value)))
                }
            }
        }
        Ok(Self(result))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        // map keys = memory address
        // values = resulting decimal number
        let mut map: HashMap<u64, u64> = HashMap::new();
        let mut mask = "";
        for command in &self.0 {
            match command {
                Command::Mask(val) => mask = val,
                Command::Mem((address, val)) => {
                    let result = apply_mask(mask, *val);
                    map.insert(*address, result);
                }
            }
        }

        let result: u64 = map.values().sum();
        Ok(result)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut map: HashMap<u64, u64> = HashMap::new();
        let mut mask = "";
        for command in &self.0 {
            match command {
                Command::Mask(val) => mask = val,
                Command::Mem((address, val)) => {
                    let floating_bit_str = apply_mask_part_two(mask, *address);
                    let results = get_options(floating_bit_str);
                    for num in results {
                        map.insert(num, *val);
                    }
                }
            }
        }

        let result: u64 = map.values().sum();
        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "165");
    }

    #[test]
    fn part_2() {
        let input = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "208");
    }
}
