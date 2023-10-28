use std::fmt::Display;

use crate::{AoCDay, AoCError};

#[derive(Debug, Clone)]
pub struct Data;

impl AoCDay<'_> for Data {
    fn try_new(_input: &str) -> Result<Self, AoCError> {
        Ok(Self)
    }

    fn part_1(&self) -> Result<impl Display, AoCError> {
        Ok("")
    }

    fn part_2(&self) -> Result<impl Display, AoCError> {
        Ok("")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "");
    }
}
