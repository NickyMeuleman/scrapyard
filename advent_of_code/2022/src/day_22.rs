use crate::AoCData;

pub struct Data {}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> Option<Self> {
        Some(Self {})
    }

    fn part_1(&self) -> String {
        String::new()
    }

    fn part_2(&self) -> String {
        String::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(22);
        let data = Data::try_new(&input).unwrap();
        assert_eq!(data.part_1(), "");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(1);
        let data = Data::try_new(&input).unwrap();
        assert_eq!(data.part_2(), "");
    }
}
