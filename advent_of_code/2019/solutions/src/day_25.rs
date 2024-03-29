use std::fmt::Display;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data;

impl AoCData<'_> for Data {
    fn try_new(_input: &str) -> AoCResult<Self> {
        Ok(Self)
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        Ok("")
    }

    fn part_2(&self) -> AoCResult<impl Display> {
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
