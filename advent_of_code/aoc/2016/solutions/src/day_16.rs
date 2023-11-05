use std::fmt::Display;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<bool>);

fn apply_dragon_step(a: &mut Vec<bool>) {
    let b: Vec<bool> = a.iter().rev().map(|b| !b).collect();
    a.push(false);
    a.extend(b);
}

fn checksum(data: &[bool]) -> Vec<bool> {
    data.chunks(2)
        .map(|chunk| chunk[0] == chunk[1])
        .collect()
}

fn helper(mut data: Vec<bool>, disk_size: usize) -> String {
    while data.len() < disk_size {
        apply_dragon_step(&mut data);
    }

    data.truncate(disk_size);

    while data.len() % 2 == 0 {
        data = checksum(&data);
    }

    data.into_iter()
        .map(|b| if b { "1" } else { "0" })
        .collect()
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let nums = input
            .chars()
            .map(|c| match c {
                '1' => true,
                '0' => false,
                _ => panic!(),
            })
            .collect();
        Ok(Self(nums))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        Ok(helper(self.0.clone(), 272))
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        Ok(helper(self.0.clone(), 35_651_584))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "10000";
        let data = Data::try_new(input).unwrap();
        assert_eq!(helper(data.0, 20), "01100");
    }

    #[test]
    fn part_2() {
        let input = "10111100110001111";
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2().unwrap().to_string(), "10001101010000101");
    }
}
