use std::{convert::Infallible, str::FromStr};

pub struct Data {
    nums: Vec<u32>,
}

impl Data {
    pub fn part_one(&self) -> usize {
        self.nums
            .windows(2)
            .filter(|window| window[0] < window[1])
            .count()
    }

    pub fn part_two(&self) -> usize {
        // turn data into sums of three-measurement windows
        self.nums
            .windows(3)
            .map(|window| window.iter().sum())
            .collect::<Vec<u32>>()
            // count the amount of times a three-measurement sum increases
            .windows(2)
            .filter(|window| window[0] < window[1])
            .count()
    }
}

impl FromStr for Data {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            nums: input
                .trim()
                .lines()
                .filter_map(|line| line.parse().ok())
                .collect(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = "199
200
208
210
200
207
240
269
260
263";

        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_one(), 7);
    }

    #[test]
    fn part_two() {
        let input = "199
200
208
210
200
207
240
269
260
263";

        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_two(), 5);
    }
}
