use std::fmt::Display;

use crate::AoCData;

#[derive(Debug, Clone)]
pub struct Data(Vec<u32>);

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> Option<Self> {
        Some(Self(
            input
                .lines()
                .filter_map(|line| line.parse().ok())
                .collect(),
        ))
    }

    fn part_1(&self) -> impl Display {
        self.0
            .iter()
            .map(|mass| mass / 3 - 2)
            .sum::<u32>()
    }

    fn part_2(&self) -> impl Display {
        fn needed_fuel(mass: u32) -> u32 {
            (mass / 3).saturating_sub(2)
        }

        // iteration
        let mut total = 0;
        for mass in &self.0 {
            let mut previous = needed_fuel(*mass);
            total += previous;
            loop {
                let needed = needed_fuel(previous);
                if needed > 0 {
                    total += needed;
                    previous = needed;
                } else {
                    break;
                }
            }
        }
        total

        // recursion, slower, more memory
        // fn needed_fuel(mass: u32) -> u32 {
        //     if mass == 0 {
        //         return 0;
        //     }
        //     let needed = (mass / 3).saturating_sub(2);
        //     needed + needed_fuel(needed)
        // }

        // self.0
        //     .iter()
        //     .map(|mass| needed_fuel(*mass))
        //     .sum::<u32>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "12";
        let data = Data::try_new(&input).unwrap();
        let result = data.part_1().to_string();
        assert_eq!(result, "2");

        let input = "14";
        let data = Data::try_new(&input).unwrap();
        let result = data.part_1().to_string();
        assert_eq!(result, "2");

        let input = "1969";
        let data = Data::try_new(&input).unwrap();
        let result = data.part_1().to_string();
        assert_eq!(result, "654");

        let input = "100756";
        let data = Data::try_new(&input).unwrap();
        let result = data.part_1().to_string();
        assert_eq!(result, "33583");
    }

    #[test]
    fn part_2() {
        let input = "14";
        let data = Data::try_new(&input).unwrap();
        let result = data.part_2().to_string();
        assert_eq!(result, "2");

        let input = "1969";
        let data = Data::try_new(&input).unwrap();
        let result = data.part_2().to_string();
        assert_eq!(result, "966");

        let input = "100756";
        let data = Data::try_new(&input).unwrap();
        let result = data.part_2().to_string();
        assert_eq!(result, "50346");
    }
}
