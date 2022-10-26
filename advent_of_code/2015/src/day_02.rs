use crate::AoCData;

pub struct Data(Vec<Dimensions>);

#[derive(Debug, Clone, Copy)]
struct Dimensions {
    l: i32,
    w: i32,
    h: i32,
}

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        let dimensions = input
            .lines()
            .map(|line| {
                let nums = line
                    .split('x')
                    .map(|s| s.parse().ok())
                    .collect::<Option<Vec<i32>>>()?;
                Some(Dimensions {
                    l: nums[0],
                    w: nums[1],
                    h: nums[2],
                })
            })
            .collect::<Option<Vec<Dimensions>>>()?;

        Some(Self(dimensions))
    }

    fn part_1(&self) -> String {
        self.0
            .iter()
            .map(|Dimensions { l, w, h }| {
                2 * l * w + 2 * w * h + 2 * l * h + (l * w).min(w * h).min(l * h)
            })
            .sum::<i32>()
            .to_string()
    }

    fn part_2(&self) -> String {
        self.0
            .iter()
            .map(|Dimensions { w, l, h }| {
                let mut sides = vec![w, l, h];
                sides.sort();
                2 * sides[0] + 2 * sides[1] + l * w * h
            })
            .sum::<i32>()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(2);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "58");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(2);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "34");
    }
}
