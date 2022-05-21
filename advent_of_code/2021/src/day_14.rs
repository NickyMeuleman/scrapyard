use std::collections::HashMap;

use crate::AoCData;

pub struct Data {
    template: Vec<char>,
    rules: HashMap<[char; 2], char>,
}

impl Data {
    fn initial_counts(&self) -> HashMap<[char; 2], u64> {
        self.template
            .windows(2)
            .fold(HashMap::new(), |mut acc, window| {
                *acc.entry([window[0], window[1]]).or_default() += 1;
                acc
            })
    }
}

struct State {
    pair_counts: HashMap<[char; 2], u64>,
}

impl State {
    fn tick(self, rules: &HashMap<[char; 2], char>) -> Self {
        // build up a new hashmap of pair counts
        // loop over every old pair and apply the relevant rule
        // Both new pairs a rule creates occur "count before the rule" amount of times, so increment the count for the new pair with that
        let pair_counts =
            self.pair_counts
                .into_iter()
                .fold(HashMap::new(), |mut acc, ([left, right], count)| {
                    let mid = rules.get(&[left, right]).unwrap();
                    *acc.entry([left, *mid]).or_default() += count;
                    *acc.entry([*mid, right]).or_default() += count;
                    acc
                });

        Self { pair_counts }
    }

    fn count_letters(&self, last_letter: char) -> HashMap<char, u64> {
        // count amount of times a letter occurs
        // pairs overlap, so only increment the count for the first letter in a pair, or you end up counting letters twice
        let mut letter_counts =
            self.pair_counts
                .iter()
                .fold(HashMap::new(), |mut acc, ([first, _], count)| {
                    *acc.entry(*first).or_default() += count;
                    acc
                });
        // this doesn't consider the last letter, so we increment the count of the last letter by 1 at the end
        *letter_counts.entry(last_letter).or_default() += 1;

        letter_counts
    }

    fn min_max(&self, last_letter: char) -> (u64, u64) {
        self.count_letters(last_letter)
            .values()
            .fold((u64::MAX, u64::MIN), |acc, &val| {
                (acc.0.min(val), acc.1.max(val))
            })
    }
}

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        let (template, rules) = input.trim().split_once("\n\n")?;
        let template = template.chars().collect();
        let rules = rules
            .lines()
            .map(|line| {
                let (left, right) = line.split_once(" -> ")?;
                let mut pair = left.chars();
                let insert = right.chars().next()?;

                Some(([pair.next().unwrap(), pair.next().unwrap()], insert))
            })
            .collect::<Option<HashMap<_, _>>>()?;

        Some(Self { template, rules })
    }

    fn part_1(&self) -> String {
        let initial_state = State {
            pair_counts: self.initial_counts(),
        };
        let final_state = (0..10).fold(initial_state, |acc, _| acc.tick(&self.rules));

        let last_letter = self.template.iter().last().unwrap();
        let (min, max) = final_state.min_max(*last_letter);

        let result = max - min;

        result.to_string()
    }

    fn part_2(&self) -> String {
        let initial_state = State {
            pair_counts: self.initial_counts(),
        };
        let final_state = (0..40).fold(initial_state, |acc, _| acc.tick(&self.rules));

        let last_letter = self.template.iter().last().unwrap();
        let (min, max) = final_state.min_max(*last_letter);

        let result = max - min;

        result.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(14);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "1588");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(14);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "2188189693529");
    }
}
