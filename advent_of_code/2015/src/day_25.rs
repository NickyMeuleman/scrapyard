use crate::AoCData;

pub struct Data {
    row: u32,
    col: u32,
}

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        let input = input.trim().strip_prefix(
            "To continue, please consult the code grid in the manual.  Enter the code at row ",
        )?;
        let input = input.strip_suffix('.')?;
        let (row, col) = input.split_once(", column ")?;
        let row: u32 = row.parse().ok()?;
        let col: u32 = col.parse().ok()?;
        Some(Self { row, col })
    }

    fn part_1(&self) -> String {
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

        code.to_string()
    }

    fn part_2(&self) -> String {
        String::from("Snow begins to fall.")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(25);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "20151125");
    }
}
