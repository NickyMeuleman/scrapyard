use crate::AoCData;

pub struct Data(Vec<bool>);

fn apply_dragon_step(a: &mut Vec<bool>) {
    let b: Vec<bool> = a.iter().rev().map(|b| !b).collect();
    a.push(false);
    a.extend(b);
}

fn checksum(data: &[bool]) -> Vec<bool> {
    data.chunks(2).map(|chunk| chunk[0] == chunk[1]).collect()
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

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        let nums = input
            .chars()
            .map(|c| match c {
                '1' => true,
                '0' => false,
                _ => panic!(),
            })
            .collect();
        Some(Self(nums))
    }

    fn part_1(&self) -> String {
        helper(self.0.clone(), 272)
    }

    fn part_2(&self) -> String {
        helper(self.0.clone(), 35_651_584)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(16);
        let data = Data::try_new(input).unwrap();
        assert_eq!(helper(data.0, 20), "01100");
    }

    #[test]
    fn part_2() {
        let input = utils::get_input(16);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "10001101010000101");
    }
}
