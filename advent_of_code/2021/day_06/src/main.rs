use std::fs;
use std::str::FromStr;

struct Data {
    timers: Vec<u8>,
}

impl Data {
    fn initial_population(&self) -> Population {
        self.timers
            .iter()
            .fold(Population::default(), |mut acc, &num| {
                acc.0[num as usize] += 1;
                acc
            })
    }

    fn part_one(&self) -> usize {
        let mut population = self.initial_population();

        for _ in 0..80 {
            population.tick();
        }

        population.size()
    }

    fn part_two(&self) -> usize {
        let mut population = self.initial_population();

        for _ in 0..256 {
            population.tick();
        }

        population.size()
    }
}

impl FromStr for Data {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            timers: input.trim().split(",").filter_map(|s| s.parse().ok()).collect(),
        })
    }
}

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

impl Default for Population {
    fn default() -> Self {
        Self([0; 9])
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let data: Data = input.parse().unwrap();
    println!("Part one answer: {}", data.part_one());
    println!("Part two answer: {}", data.part_two());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_one_example() {
        let input = "3,4,3,1,2";
        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_one(), 5934);
    }

    #[test]

    fn part_two_example() {
        let input = "3,4,3,1,2";
        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_two(), 26984457539);
    }
}
