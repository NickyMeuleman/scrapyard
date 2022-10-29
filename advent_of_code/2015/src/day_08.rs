use crate::{utils::Solution, AoCData};

pub struct Data(String);

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        Some(Self(input))
    }

    fn part_1(&self) -> String {
        let mut result = 0;
        for line in self.0.lines() {
            let literal = line.len();
            // skip opening and closing quotes
            let mut in_memory = 0;
            let line = &line[1..literal - 1];
            let mut chars = line.chars().peekable();
            while let Some(c) = chars.next() {
                in_memory += 1;
                if c == '\\' {
                    let next = chars.peek();
                    if let Some('x') = next {
                        // hexadecimal char found, skip 3 chars after backslash, \xab counts as 1 in memory
                        chars.next();
                        chars.next();
                        chars.next();
                    } else {
                        // skip char after backslash, \\ counts as 1 in memory
                        chars.next();
                    }
                }
            }
            result += literal - in_memory;
        }

        result.to_string()
    }

    fn part_2(&self) -> String {
        self.0
            .lines()
            .fold(0, |acc, line| {
                let literal = line
                    .chars()
                    // map every char to the size of the literal that represents it
                    // \ turns into \\ for size 2, " turns into \" for size 2, everything else is size 1
                    .map(|c| match c {
                        '\\' | '"' => 2, // escape it, add a backslash
                        _ => 1, // no escaping needed
                    })
                    .sum::<usize>() + 2; // add 2 for the starting and ending "
                acc + (literal - line.len())
            })
            .to_string()
    }

    /// way less readable version that does the 2 parts in one pass
    fn solve(self) -> crate::utils::Solution
    where
        Self: Sized,
    {
        let mut result_1 = 0;
        let mut result_2 = 0;

        for line in self.0.lines() {
            let input_len = line.len();
            let mut in_memory = 0; // size in memory
            let mut literal = 6; // literal size after escaping, start at 6 to escape the surrounding "
            let line = &line[1..line.len() - 1];
            let mut chars = line.chars().peekable();

            while let Some(c) = chars.next() {
                in_memory += 1;
                literal += 1;
                if c == '\\' {
                    let next = chars.peek();
                    if let Some('x') = next {
                        // hexadecimal char found, skip 3 chars after backslash
                        chars.next();
                        chars.next();
                        chars.next();
                        literal += 4;
                    } else {
                        // skip char after backslash
                        chars.next();
                        literal += 3;
                        // note: a " always follows a backslash, so we there is no special logic for it
                    }
                }
            }

            result_1 += input_len - in_memory;
            result_2 += literal - input_len;
        }

        Solution {
            part1: result_1.to_string(),
            part2: result_2.to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(8);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "12");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(8);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "19");
    }

    #[test]
    fn solve() {
        let input = utils::get_sample_input(8);
        let data = Data::try_new(input).unwrap();
        let solution = data.solve();

        assert_eq!(solution.part1, "12");
        assert_eq!(solution.part2, "19");
    }

    // #[test]
    // fn solve() {
    //     let input = "aa\"aa".to_string();
    //     let data = Data::try_new(input).unwrap();
    //     let solution = data.solve();

    //     assert_eq!(solution.part1, "12");
    //     assert_eq!(solution.part2, "19");
    // }
}
