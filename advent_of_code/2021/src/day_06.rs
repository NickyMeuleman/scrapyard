use crate::AoCData;

pub struct Data {
    timers: Vec<u8>,
}

impl Data {
    fn initial_population(&self) -> Population {
        self.timers
            .iter()
            .fold(Population::default(), |mut acc, &num| {
                acc.0[usize::from(num)] += 1;
                acc
            })
    }
}

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        let timers = input
            .trim()
            .split(',')
            .map(|s| s.parse().ok())
            .collect::<Option<Vec<_>>>()?;

        Some(Self { timers })
    }

    fn part_1(&self) -> String {
        let mut population = self.initial_population();

        for _ in 0..80 {
            population.tick();
        }

        population.size().to_string()
    }

    fn part_2(&self) -> String {
        let mut population = self.initial_population();

        for _ in 0..256 {
            population.tick();
        }

        population.size().to_string()
    }
}

#[derive(Default)]
struct Population([usize; 9]);

impl Population {
    fn tick(&mut self) {
        self.0.rotate_left(1);
        self.0[6] += self.0[8];
    }

    fn size(&self) -> usize {
        self.0.iter().sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(6);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "5934");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(6);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "26984457539");
    }
}
