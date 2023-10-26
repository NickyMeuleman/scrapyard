use std::fmt::Display;

use crate::AoCDay;

#[derive(Debug, Clone)]
pub struct Data;

impl AoCDay<'_> for Data {
    fn try_new(_input: &str) -> Option<Self> {
        Some(Self)
    }

    fn part_1(&self) -> impl Display {
        ""
    }

    fn part_2(&self) -> impl Display {
        ""
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().to_string();
        assert_eq!(result, "");
    }

    #[test]
    fn part_2() {
        let input = "";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().to_string();
        assert_eq!(result, "");
    }
}
