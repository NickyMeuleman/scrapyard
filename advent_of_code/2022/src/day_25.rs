use crate::AoCData;

pub struct Data<'a>(&'a str);

fn to_decimal(snafu: &str) -> i64 {
    snafu.chars().fold(0, |decimal, snafu_digit| {
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
    fn try_new(input: &'a str) -> Option<Self> {
        Some(Self(input))
    }

    fn part_1(&self) -> String {
        let sum = self.0.lines().map(to_decimal).sum();
        to_snafu(sum)
    }

    fn part_2(&self) -> String {
        String::from("You make a smoothie from 50 stars!")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(25);
        let data = Data::try_new(&input).unwrap();
        assert_eq!(data.part_1(), "2=-1=0");
    }
}
