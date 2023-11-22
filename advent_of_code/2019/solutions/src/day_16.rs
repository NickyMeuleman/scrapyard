use std::{fmt::Display, mem};

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<u8>);

const BASE_PATTERN: [i8; 4] = [0, 1, 0, -1];

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        Ok(Self(
            input
                .chars()
                .filter_map(|s| s.to_digit(10))
                .filter_map(|n| n.try_into().ok())
                .collect(),
        ))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut numbers = self.0.clone();

        for _ in 0..100 {
            numbers = (1..=numbers.len())
                .map(|n| {
                    let sum: i32 = numbers
                        .iter()
                        .zip(
                            BASE_PATTERN
                                .iter()
                                .flat_map(move |num| std::iter::repeat(num).take(n))
                                .cycle()
                                .skip(1),
                        )
                        .map(|(&input_digit, &pattern_val)| input_digit as i32 * pattern_val as i32)
                        .sum();

                    (sum.abs() % 10) as u8
                })
                .collect();
        }

        Ok(numbers
            .into_iter()
            .take(8)
            .fold(0, |acc, curr| acc * 10 + curr as u32))
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        // https://github.com/LinAGKar/advent-of-code-2019-rust/blob/master/day16b/src/main.rs
        // https://www.reddit.com/r/adventofcode/comments/ebf5cy/comment/fb4bvw4/
        // and https://dhconnelly.com/advent-of-code-2019-commentary.html#day-16
        // At some point here I noticed another thing
        // in the debug output I was dumping to the console:
        // it seemed like the coefficients were all ones!
        // While taking a break, something occurred to me
        // that I noticed during my matrix math diversion:
        // rows in the lower half of the matrix were all ones,
        // regardless of what size matrix I wrote out for doing manual products
        // (to look for a general formula).
        // And then it was obvious:
        // the first n-1 elements of the nth row are zero,
        // and then the next n elements are one,
        // and so together this means that
        // if we're looking at a row more than half way down the matrix,
        // the zeroes and ones make up the entire row!
        let mut values: Vec<u8> = self
            .0
            .iter()
            .map(|n| *n as u8)
            .cycle()
            .take(self.0.len() * 10_000)
            .collect();

        let offset = self
            .0
            .iter()
            .take(7)
            .fold(0, |acc, &curr| acc * 10 + curr as usize);
        assert!(offset > values.len() / 2);

        values = values[offset..].to_vec();
        let mut new_values: Vec<u8> = vec![0; values.len()];

        for _ in 0..100 {
            let mut sum = 0;
            for (idx, val) in values.iter().enumerate().rev() {
                sum += *val as u32;
                new_values[idx] = (sum % 10) as u8;
            }

            // let mut sum = 0;
            // for (val, new_val) in values
            //     .iter()
            //     .zip(new_values.iter_mut())
            //     .rev()
            // {
            //     sum += *val as u32;
            //     *new_val = (sum % 10) as u8;
            // }

            // values
            //     .iter()
            //     .zip(new_values.iter_mut())
            //     .rev()
            //     .fold(0, |acc, (&val, new_val)| {
            //         let sum = acc + val as u32;
            //         *new_val = (sum % 10) as u8;
            //         sum
            //     });
            mem::swap(&mut values, &mut new_values);
        }

        Ok(values
            .into_iter()
            .take(8)
            .fold(0, |acc, curr| acc * 10 + curr as u32))
    }
    // https://github.com/prscoelho/aoc2019/blob/master/src/aoc16/mod.rs
    // fn part_2(&self) -> AoCResult<impl Display> {
    //     let numbers = self.0.clone();
    //     let start = numbers
    //         .iter()
    //         .take(7)
    //         .fold(0, |acc, curr| acc * 10 + curr) as usize;
    //     let end = numbers.len() * 10_000;

    //     let mut current = Vec::new();
    //     for i in start..end {
    //         current.push(numbers[i % numbers.len()]);
    //     }

    //     for _ in 0..100 {
    //         let mut sums = Vec::new();
    //         let mut total = 0;
    //         sums.push(0);
    //         for i in 0..current.len() {
    //             total += current[i];
    //             sums.push(total);
    //         }

    //         for i in 0..current.len() {
    //             let value = sums.last().unwrap() - sums[i];
    //             current[i] = value % 10;
    //         }
    //     }

    //     Ok(current
    //         .into_iter()
    //         .take(8)
    //         .fold(0, |acc, curr| acc * 10 + curr))
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "80871224585914546619083218645595";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "24176176");

        let input = "19617804207202209144916044189917";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "73745418");

        let input = "69317163492948606335995924319873";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "52432133");
    }

    #[test]
    fn part_2() {
        let input = "03036732577212944063491565474664";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "84462026");

        let input = "02935109699940807407585447034323";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "78725270");

        let input = "03081770884921959731165446850517";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "53553731");
    }
}
