use crate::AoCData;

pub struct Data {
    positions: Vec<u16>,
}

impl Data {
    fn min_max(&self) -> (u16, u16) {
        self.positions
            .iter()
            .fold((u16::MAX, u16::MIN), |(min, max), pos| {
                (*pos.min(&min), *pos.max(&max))
            })
    }
}

impl AoCData for Data {
    fn new(input: String) -> Self {
        Self {
            positions: input
                .trim()
                .split(',')
                .filter_map(|s| s.parse().ok())
                .collect(),
        }
    }

    fn part_1(&self) -> String {
        let (min, max) = self.min_max();

        (min..=max)
            .map(|target_pos| {
                // calculate the total cost if every crab went to target_pos
                self.positions
                    .iter()
                    .map(move |&start_pos| (start_pos, target_pos))
                    // calculate cost for a single crab:
                    // get amount a crab has to move to get to target_pos
                    // get cost for that distance
                    .map(|(to, from)| (to as i32 - from as i32).abs())
                    // sum all costs for each crab
                    .sum::<i32>()
            })
            .min()
            .unwrap()
            .to_string()
    }

    fn part_2(&self) -> String {
        let (min, max) = self.min_max();

        (min..=max)
            .map(|target_pos| {
                // calculate the total cost if every crab went to target_pos
                self.positions
                    .iter()
                    .map(move |&start_pos| (start_pos, target_pos))
                    // calculate cost for a single crab:
                    .map(|(to, from)| {
                        // get amount a crab has to move to get to target_pos
                        let distance = (to as i32 - from as i32).abs();
                        let steps = distance;
                        let first = 1;
                        let last = distance;
                        // get cost for that distance
                        // This is the sum of an arithmetic sequence: https://en.wikipedia.org/wiki/Arithmetic_progression
                        steps * (first + last) / 2
                    })
                    // sum all costs for each crab
                    .sum::<i32>()
            })
            .min()
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(7);
        let data = Data::new(input);
        assert_eq!(data.part_1(), "37");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(7);
        let data = Data::new(input);
        assert_eq!(data.part_2(), "168");
    }
}
