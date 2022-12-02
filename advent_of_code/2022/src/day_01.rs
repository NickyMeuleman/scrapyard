use itertools::Itertools;

use crate::AoCData;

pub struct Data(String);

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        Some(Self(input))
    }

    fn part_1(&self) -> String {
        self.0
            .split("\n\n")
            .map(|elf| {
                elf.lines()
                    .filter_map(|s| s.parse::<u32>().ok())
                    .sum::<u32>()
            })
            .max()
            .unwrap()
            .to_string()
    }

    fn part_2(&self) -> String {
        self.0
            .split("\n\n")
            .map(|elf| {
                elf.lines()
                    .filter_map(|s| s.parse::<u32>().ok())
                    .sum::<u32>()
            })
            .sorted()
            .rev()
            .take(3)
            .sum::<u32>()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(1);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "24000");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(1);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "45000");
    }
}
