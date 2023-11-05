use std::fmt::Display;

use itertools::Itertools;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input.trim()))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let chars = self.0.chars().collect_vec();

        let result: u32 = chars
            .iter()
            .circular_tuple_windows()
            .filter(|(c1, c2)| c1 == c2)
            .filter_map(|(c1, _)| c1.to_digit(10))
            .sum();

        Ok(result)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let chars = self.0.chars().collect_vec();
        let chars2 = chars
            .iter()
            .cycle()
            .skip(chars.len() / 2)
            .take(chars.len())
            .collect_vec();
        let result: u32 = chars
            .iter()
            .zip(chars2)
            .circular_tuple_windows()
            .filter(|((c1, c2), (_, _))| c1 == c2)
            .filter_map(|(c1, _)| c1.0.to_digit(10))
            .sum();

        Ok(result)
    }
    // fn part_2(&self) -> AoCResult<impl Display> {
    //     let chars = self.0.chars().cycle();
    //     let chars2 = self
    //         .0
    //         .chars()
    //         .cycle()
    //         .skip(self.0.len() / 2);
    //     let result: u32 = chars
    //         .zip(chars2)
    //         .take(self.0.len() + 1)
    //         .filter(|(c1, c2)| c1 == c2)
    //         .filter_map(|(c1, _)| c1.to_digit(10))
    //         .sum();

    //     Ok(result)
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "91212129";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "9");
    }

    #[test]
    fn part_2() {
        let input = "123425";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "4");
    }
}
