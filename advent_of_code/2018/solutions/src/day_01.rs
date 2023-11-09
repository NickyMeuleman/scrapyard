use std::{collections::HashSet, fmt::Display};

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data<'a>(&'a str);

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        Ok(Self(input))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        Ok(self
            .0
            .lines()
            .map(|line| line.parse::<i32>().unwrap())
            .sum::<i32>())
    }

    // foldwhile, shortcircuiting fold
    // pub fn part_2(input: &str) -> i32 {
    //     input
    //         .lines()
    //         .map(|line| line.parse::<i32>().unwrap())
    //         .cycle()
    //         .fold_while((HashSet::new(), 0), |(mut seen, sum), num| {
    //             let new = sum + num;
    //             if !seen.insert(new) {
    //                 Done((seen, new))
    //             } else {
    //                 Continue((seen, new))
    //             }
    //         })
    //         .into_inner().1
    // }

    fn part_2(&self) -> AoCResult<impl Display> {
        let nums = self
            .0
            .lines()
            .map(|line| line.parse::<i32>().unwrap())
            .cycle();
        let mut seen = HashSet::new();
        let mut sum = 0;

        for num in nums {
            sum += num;
            if seen.contains(&sum) {
                return Ok(sum);
            } else {
                seen.insert(sum);
            }
        }

        Ok(sum)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "+1
+1
+1";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "3");
    }

    #[test]
    fn part_2() {
        let input = "+7
+7
-2
-7
-4";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "14");
    }
}
