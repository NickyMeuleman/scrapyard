use std::iter;

use crate::AoCData;

pub struct Data(Vec<bool>);

fn helper(mut row: Vec<bool>, total: u32) -> usize {
    let mut count = row.iter().filter(|&trap| !trap).count();

    for _ in 1..total {
        let padded: Vec<bool> = iter::once(false)
            .chain(row.into_iter())
            .chain(iter::once(false))
            .collect();
        let new_row: Vec<bool> = padded
            .windows(3)
            .map(|window| window[0] != window[2])
            .collect();
        count += new_row.iter().filter(|&trap| !trap).count();
        row = new_row;
    }

    count
}

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        let mut row = Vec::new();
        for c in input.trim().chars() {
            let trap = match c {
                '.' => false,
                '^' => true,
                _ => return None,
            };
            row.push(trap);
        }
        Some(Self(row))
    }

    fn part_1(&self) -> String {
        helper(self.0.clone(), 40).to_string()
    }

    fn part_2(&self) -> String {
        helper(self.0.clone(), 400_000).to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(18);
        let data = Data::try_new(input).unwrap();
        assert_eq!(helper(data.0, 10), 38);
    }

    #[test]
    fn part_2() {
        let input = utils::get_input(18);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "19991126");
    }
}
