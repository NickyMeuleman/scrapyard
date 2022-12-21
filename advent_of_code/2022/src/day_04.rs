use crate::AoCData;

pub struct Data(Vec<[[u32; 2]; 2]>);

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> Option<Self> {
        fn to_range(s: &str) -> Option<[u32; 2]> {
            let (min, max) = s.split_once('-')?;
            Some([min.parse().ok()?, max.parse().ok()?])
        }

        let mut pairs = Vec::new();
        for line in input.lines() {
            let (elf1, elf2) = line.split_once(',')?;
            pairs.push([to_range(elf1)?, to_range(elf2)?]);
        }
        Some(Self(pairs))
    }

    fn part_1(&self) -> String {
        self.0
            .iter()
            .filter(|[[min1, max1], [min2, max2]]| {
                (min1 >= min2 && max1 <= max2) || (min2 >= min1 && max2 <= max1)
                // equivalent:
                // (min1 <= min2 && max1 >= max2) || (min2 <= min1 && max2 >= max1)
            })
            .count()
            .to_string()
    }

    fn part_2(&self) -> String {
        self.0
            .iter()
            .filter(|[[min1, max1], [min2, max2]]| {
                min1.max(min2) <= max1.min(max2)
                // equivalent:
                // min1 <= max2 && max1 >= min2
            })
            .count()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(4);
        let data = Data::try_new(&input).unwrap();
        assert_eq!(data.part_1(), "2");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(4);
        let data = Data::try_new(&input).unwrap();
        assert_eq!(data.part_2(), "4");
    }
}
