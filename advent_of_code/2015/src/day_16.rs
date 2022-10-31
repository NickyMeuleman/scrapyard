use std::collections::HashMap;

use crate::AoCData;

pub struct Data(HashMap<u16, SueProps>);

type SueProps = HashMap<SueKey, u16>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum SueKey {
    Children,
    Cats,
    Samoyeds,
    Pomeranians,
    Akitas,
    Vizslas,
    Goldfish,
    Trees,
    Cars,
    Perfumes,
}

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        let mut sues = HashMap::new();
        for line in input.lines() {
            let mut sueprops: SueProps = HashMap::new();
            let line = line.strip_prefix("Sue ")?;
            let (idx, rest) = line.split_once(": ")?;
            let idx = idx.parse().ok()?;
            for prop in rest.split(", ") {
                let (name, amount) = prop.split_once(": ")?;
                let amount = amount.parse().ok()?;
                let suekey = match name {
                    "children" => SueKey::Children,
                    "cats" => SueKey::Cats,
                    "samoyeds" => SueKey::Samoyeds,
                    "pomeranians" => SueKey::Pomeranians,
                    "akitas" => SueKey::Akitas,
                    "vizslas" => SueKey::Vizslas,
                    "goldfish" => SueKey::Goldfish,
                    "trees" => SueKey::Trees,
                    "cars" => SueKey::Cars,
                    "perfumes" => SueKey::Perfumes,
                    _ => panic!("invalid input"),
                };
                sueprops.insert(suekey, amount);
            }
            sues.insert(idx, sueprops);
        }
        Some(Self(sues))
    }

    fn part_1(&self) -> String {
        let mut one_true_sue = HashMap::new();
        one_true_sue.insert(SueKey::Children, 3);
        one_true_sue.insert(SueKey::Cats, 7);
        one_true_sue.insert(SueKey::Samoyeds, 2);
        one_true_sue.insert(SueKey::Pomeranians, 3);
        one_true_sue.insert(SueKey::Akitas, 0);
        one_true_sue.insert(SueKey::Vizslas, 0);
        one_true_sue.insert(SueKey::Goldfish, 5);
        one_true_sue.insert(SueKey::Trees, 3);
        one_true_sue.insert(SueKey::Cars, 2);
        one_true_sue.insert(SueKey::Perfumes, 1);

        self.0
            .iter()
            .find(|(_k, v)| v.iter().all(|(k, v)| one_true_sue.get(k) == Some(v)))
            .map(|(k, _v)| k)
            .unwrap_or(&0)
            .to_string()
    }

    fn part_2(&self) -> String {
        let mut one_true_sue = HashMap::new();
        one_true_sue.insert(SueKey::Children, 3);
        one_true_sue.insert(SueKey::Cats, 7);
        one_true_sue.insert(SueKey::Samoyeds, 2);
        one_true_sue.insert(SueKey::Pomeranians, 3);
        one_true_sue.insert(SueKey::Akitas, 0);
        one_true_sue.insert(SueKey::Vizslas, 0);
        one_true_sue.insert(SueKey::Goldfish, 5);
        one_true_sue.insert(SueKey::Trees, 3);
        one_true_sue.insert(SueKey::Cars, 2);
        one_true_sue.insert(SueKey::Perfumes, 1);

        self.0
            .iter()
            .find(|(_k, v)| {
                v.iter().all(|(k, v)| match k {
                    SueKey::Cats | SueKey::Trees => {
                        one_true_sue.get(k).unwrap() < v
                    },
                    SueKey::Pomeranians | SueKey::Goldfish => {
                        one_true_sue.get(k).unwrap() > v
                    },
                    _ => one_true_sue.get(k) == Some(v),
                })
            })
            .map(|(k, _v)| k)
            .unwrap_or(&0)
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_input(16);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "213");
    }

    #[test]
    fn part_2() {
        let input = utils::get_input(16);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "323");
    }
}