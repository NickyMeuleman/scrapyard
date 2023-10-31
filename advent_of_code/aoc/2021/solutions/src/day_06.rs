use std::fmt::Display;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
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

#[derive(Default)]
struct Population([u64; 9]);

impl Population {
    fn tick(&mut self) {
        self.0.rotate_left(1);
        self.0[6] += self.0[8];
    }

    fn size(&self) -> u64 {
        self.0.iter().sum()
    }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let timers = input
            .trim()
            .split(',')
            .map(|s| s.parse())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { timers })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut population = self.initial_population();

        for _ in 0..80 {
            population.tick();
        }

        Ok(population.size())
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut population = self.initial_population();

        for _ in 0..256 {
            population.tick();
        }

        Ok(population.size())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "3,4,3,1,2";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "5934");
    }

    #[test]
    fn part_2() {
        let input = "3,4,3,1,2";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "26984457539");
    }
}
