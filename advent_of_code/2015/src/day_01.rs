use crate::AoCData;

pub struct Data(String);

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        Some(Self(input))
    }

    fn part_1(&self) -> String {
        self.0
            .chars()
            .fold(0, |acc, c| match c {
                '(' => acc + 1,
                ')' => acc - 1,
                _ => panic!("invalid input"),
            })
            .to_string()
    }

    fn part_2(&self) -> String {
        self.0
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
            .unwrap_or_else(|| "Santa never entered the basement".to_string())

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
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(1);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "-1");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(1);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "5");
    }
}
