use std::{fmt::Display, iter};

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(usize);

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let num = input.trim().parse()?;
        Ok(Self(num))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        // https://www.youtube.com/watch?v=uCsD3ZGzMgE
        let count = self.0;
        let leading_zeros = count.leading_zeros();
        let leading_one_removed = count << (leading_zeros + 1);
        let leading_one_removed = leading_one_removed >> leading_zeros;
        let trailing_one_added = leading_one_removed + 1;

        Ok(trailing_one_added)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        // pos: the idx that points to the steelee
        // elves[pos]: the idx of the steelee
        // elves[elves[pos]]: the idx the steelee points to
        let mut count = self.0;
        let mut elves: Vec<_> = (1..count)
            .chain(iter::once(0))
            .collect();
        let mut pos = count / 2 - 1;
        while count > 1 {
            elves[pos] = elves[elves[pos]];
            if count % 2 != 0 {
                pos = elves[pos];
            }
            count -= 1;
        }

        Ok(pos + 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "5";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "3");
    }

    #[test]
    fn part_2() {
        let input = "5";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "2");
    }
}
