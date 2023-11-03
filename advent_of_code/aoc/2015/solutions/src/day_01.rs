use std::fmt::Display;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(String);

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        Ok(Self(input.to_string()))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let result = self
            .0
            .chars()
            .fold(0, |acc, c| match c {
                '(' => acc + 1,
                ')' => acc - 1,
                _ => panic!("invalid input"),
            });

        Ok(result)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let result = self
            .0
            .chars()
            // create iterator that has the current floor Santa is on, starting at floor 0
            .scan(0, |floor, c| {
                // go up or down?
                let direction = match c {
                    '(' => 1,
                    ')' => -1,
                    _ => panic!("invalid input"),
                };
                // the next item in the iterator is the floor after santa moved
                *floor += direction;
                Some(*floor)
            })
            // find the first index where santa enters the basement
            .position(|floor| floor < 0)
            .map(|idx| (idx + 1).to_string())
            .unwrap_or_else(|| "Santa never entered the basement".to_string());

        // try_fold method
        // self.0
        //     .chars()
        //     .enumerate()
        //     .try_fold(0, |acc, (n, x)| match x {
        //         '(' => Ok(acc + 1),
        //         ')' => {
        //             if acc > 0 {
        //                 Ok(acc - 1)
        //             } else {
        //                 Err(n + 1)
        //             }
        //         }
        //         _ => Ok(acc),
        //     })
        //     .unwrap_err()
        //     .to_string()
        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "()())";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "-1");
    }

    #[test]
    fn part_2() {
        let input = "()())";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "5");
    }
}
