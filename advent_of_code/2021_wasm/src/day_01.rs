use crate::AoCData;

pub struct Data {
    nums: Vec<u32>,
}

impl AoCData for Data {
    fn new(input: String) -> Self {
        Self {
            nums: input
                .trim()
                .lines()
                .filter_map(|line| line.parse().ok())
                .collect(),
        }
    }

    fn part_1(&self) -> String {
        self.nums
            .windows(2)
            .filter(|window| window[0] < window[1])
            .count()
            .to_string()
    }

    fn part_2(&self) -> String {
        // turn data into sums of three-measurement windows
        self.nums
            .windows(3)
            .map(|window| window.iter().sum())
            .collect::<Vec<u32>>()
            // count the amount of times a three-measurement sum increases
            .windows(2)
            .filter(|window| window[0] < window[1])
            .count()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(1);
        let data = Data::new(input);
        assert_eq!(data.part_1(), "7");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(1);
        let data = Data::new(input);
        assert_eq!(data.part_2(), "5");
    }
}
