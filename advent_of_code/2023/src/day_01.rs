use crate::AoCData;

#[derive(Debug)]
pub struct Data(Vec<u32>);

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> Option<Self> {
        Some(Self(
            input
                .lines()
                .filter_map(|line| line.parse().ok())
                .collect::<Vec<u32>>(),
        ))
    }

    fn part_1(&self) -> String {
        use std::{thread, time};

        thread::sleep(time::Duration::from_millis(10));
        self.0
            .iter()
            .map(|mass| mass / 3 - 2)
            .sum::<u32>()
            .to_string()
    }

    fn part_2(&self) -> String {
        // fn needed_fuel(mass: u32) -> u32 {
        //     (mass / 3).saturating_sub(2)
        // }

        // iteration
        // let mut total = 0;
        // for mass in &self.0 {
        //     let mut previous = needed_fuel(*mass);
        //     total += previous;
        //     loop {
        //         let needed = needed_fuel(previous);
        //         if needed > 0 {
        //             total += needed;
        //             previous = needed;
        //         } else {
        //             break;
        //         }
        //     }
        // }
        // total.to_string()

        // recursion
        fn needed_fuel(mass: u32) -> u32 {
            if mass == 0 {
                return 0;
            }
            let needed = (mass / 3).saturating_sub(2);
            needed + needed_fuel(needed)
        }

        self.0
            .iter()
            .map(|mass| needed_fuel(*mass))
            .sum::<u32>()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    extern crate test;
    use super::*;
    use crate::utils;
    use test::{black_box, Bencher};

    #[test]
    fn part_1() {
        let input = utils::get_input(1, true).unwrap();
        let data = Data::try_new(&input).unwrap();
        assert_eq!(data.part_1(), "");
    }

    #[test]
    fn part_2() {
        let input = utils::get_input(1, true).unwrap();
        let data = Data::try_new(&input).unwrap();
        assert_eq!(data.part_2(), "");
    }

    #[bench]
    fn bench_parsing(b: &mut Bencher) {
        let input = utils::get_input(1, true).unwrap();
        b.iter(|| black_box(Data::try_new(&input)));
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let input = utils::get_input(1, true).unwrap();
        let data = Data::try_new(&input).unwrap();
        b.iter(|| black_box(data.part_1()));
    }
}
