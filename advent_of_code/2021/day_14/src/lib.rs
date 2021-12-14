use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Data {
    template: Vec<char>,
    rules: HashMap<[char; 2], char>,
}

impl Data {
    fn initial_pairs(&self) -> HashMap<[char; 2], u64> {
        // build up hashmap that counts occurrences of unique pairs
        self.template
            .windows(2)
            .fold(HashMap::new(), |mut acc, window| {
                let count = acc.entry([window[0], window[1]]).or_default();
                *count += 1;
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
        let letter_counts = self.count_letters(last_letter);
        let min = *letter_counts.values().min().unwrap();
        let max = *letter_counts.values().max().unwrap();

        (min, max)
    }
}

impl Data {
    pub fn part_one(&self) -> u64 {
        let initial_state = State { pair_counts: self.initial_pairs()};
        let final_state = (0..10).fold(initial_state, |acc, _| acc.tick(&self.rules));

        let last_letter = self.template.iter().last().unwrap();
        let (min, max) = final_state.min_max(*last_letter);

        max - min
    }

    pub fn part_two(&self) -> u64 {
        let initial_state = State { pair_counts: self.initial_pairs()};
        let final_state = (0..40).fold(initial_state, |acc, _| acc.tick(&self.rules));

        let last_letter = self.template.iter().last().unwrap();
        let (min, max) = final_state.min_max(*last_letter);

        max - min
    }
}

impl FromStr for Data {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (template, rules) = input.split_once("\n\n").unwrap();
        let template: Vec<char> = template.chars().collect();
        let rules: HashMap<[char; 2], char> = rules
            .lines()
            .map(|line| {
                let (from, to) = line.split_once(" -> ").unwrap();
                let from: Vec<char> = from.chars().collect();
                ([from[0], from[1]], to.chars().next().unwrap())
            })
            .collect();
        Ok(Self { template, rules })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_one(), 1588);
    }

    #[test]
    fn part_two_example() {
        let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_two(), 2188189693529);
    }
}
