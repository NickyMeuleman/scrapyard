use std::iter;

use crate::AoCData;

pub struct Data(usize);

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        let num = input.trim().parse().ok()?;
        Some(Self(num))
    }

    fn part_1(&self) -> String {
        // https://www.youtube.com/watch?v=uCsD3ZGzMgE
        let count = self.0;
        let leading_zeros = count.leading_zeros();
        let leading_one_removed = count << (leading_zeros + 1);
        let leading_one_removed = leading_one_removed >> leading_zeros;
        let trailing_one_added = leading_one_removed + 1;
        trailing_one_added.to_string()
    }

    fn part_2(&self) -> String {
        // pos: the idx that points to the steelee
        // elves[pos]: the idx of the steelee
        // elves[elves[pos]]: the idx the steelee points to
        let mut count = self.0;
        let mut elves: Vec<_> = (1..count).chain(iter::once(0)).collect();
        let mut pos = count / 2 - 1;
        while count > 1 {
            elves[pos] = elves[elves[pos]];
            if count % 2 != 0 {
                pos = elves[pos];
            }
            count -= 1;
        }
        (pos + 1).to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(19);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "3");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(19);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "2");
    }
}
