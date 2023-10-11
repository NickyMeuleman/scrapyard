use std::fmt::Display;

use itertools::Itertools;

use crate::AoCData;

pub struct Data<'a>(&'a str);

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> Option<Self> {
        Some(Self(input))
    }

    fn part_1(&self) -> impl Display {
        self.0
            .split("\n\n")
            .map(|elf| {
                elf.lines()
                    .filter_map(|s| s.parse::<u32>().ok())
                    .sum::<u32>()
            })
            .max()
            .unwrap()
    }

    fn part_2(&self) -> impl Display {
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
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::get_input;

    #[test]
    fn part_1() {
        let input = get_input(2, true).unwrap();
        let data = Data::try_new(&input).unwrap();
        let result = data.part_1().to_string();
        assert_eq!(result, "");
    }

    #[test]
    fn part_2() {
        let input = get_input(2, true).unwrap();
        let data = Data::try_new(&input).unwrap();
        let result = data.part_2().to_string();
        assert_eq!(result, "");
    }
}
