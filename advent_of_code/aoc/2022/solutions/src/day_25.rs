use std::fmt::Display;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

fn to_decimal(snafu: &str) -> i64 {
    snafu
        .chars()
        .fold(0, |decimal, snafu_digit| {
            let decimal_digit = ['=', '-', '0', '1', '2']
                .into_iter()
                .position(|c| c == snafu_digit)
                .unwrap() as i64
                - 2;
            decimal * 5 + decimal_digit
        })
}

fn to_snafu(decimal: i64) -> String {
    if decimal == 0 {
        return String::new();
    }

    let decimal_remainder = decimal % 5;
    let snafu_digit = ['0', '1', '2', '=', '-'][decimal_remainder as usize];

    let new_decimal = (decimal + 2) / 5;
    let mut snafu = to_snafu(new_decimal);
    snafu.push(snafu_digit);

    snafu
}

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let sum = self.0.lines().map(to_decimal).sum();
        Ok(to_snafu(sum))
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        Ok("You make a smoothie from 50 stars!")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "2=-1=0");
    }
}
