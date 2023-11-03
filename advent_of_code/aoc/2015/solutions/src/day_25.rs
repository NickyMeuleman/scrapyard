use std::fmt::Display;

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data {
    row: u32,
    col: u32,
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let input = input
            .trim()
            .strip_prefix(
                "To continue, please consult the code grid in the manual.  Enter the code at row ",
            )
            .ok_or(AoCError::Parsing)?;
        let input = input
            .strip_suffix('.')
            .ok_or(AoCError::Parsing)?;
        let (row, col) = input
            .split_once(", column ")
            .ok_or(AoCError::Parsing)?;
        let row: u32 = row.parse()?;
        let col: u32 = col.parse()?;
        Ok(Self { row, col })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut code: i64 = 20_151_125;
        let mut row = 1;
        let mut col = 1;
        while (row, col) != (self.row, self.col) {
            if row == 1 {
                row = col + 1;
                col = 1;
            } else {
                row -= 1;
                col += 1;
            }
            code = (code * 252_533) % 33_554_393;
        }

        Ok(code)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        Ok("Snow begins to fall.")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "To continue, please consult the code grid in the manual.  Enter the code at row 1, column 1.";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "20151125");
    }
}
