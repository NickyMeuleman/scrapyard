use crate::AoCData;

pub struct Data(usize);

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        Some(Self(input.parse().ok()?))
    }

    fn part_1(&self) -> String {
        let mut houses = vec![0; &self.0 / 10 + 1];
        for elf in 1..houses.len() {
            for house in (elf..houses.len()).step_by(elf) {
                houses[house] += elf * 10;
            }
        }

        houses
            .iter()
            .position(|x| x >= &self.0)
            .unwrap()
            .to_string()
    }

    fn part_2(&self) -> String {
        let mut houses = vec![0; &self.0 / 10 + 1];
        for elf in 1..houses.len() {
            let max_house = houses.len().min(elf * 50);
            for house in (elf..max_house).step_by(elf) {
                houses[house] += elf * 11;
            }
        }

        houses
            .iter()
            .position(|x| x >= &self.0)
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_input(20);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "831600");
    }

    #[test]
    fn part_2() {
        let input = utils::get_input(20);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "884520");
    }
}
