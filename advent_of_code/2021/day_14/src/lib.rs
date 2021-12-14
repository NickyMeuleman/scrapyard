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
    fn count_letters(&self, last_letter: char) -> HashMap<char, u64> {
        // count amount of times a letter occurs
        // pairs overlap, so only increment the count for the first letter in a pair
        // this doesn't consider the last letter, so we increment the count of the last letter by 1 at the end
        let mut final_counts = HashMap::new();
        for ([first, _], count) in &self.pair_counts {
            *final_counts.entry(*first).or_default() += count;
        }
        *final_counts.entry(last_letter).or_default() += 1;

        final_counts
    }

    fn min_max(self, last_letter: char) -> (u64, u64) {
        let letter_counts = self.count_letters(last_letter);
        let min = letter_counts.clone().into_values().min().unwrap();
        let max = letter_counts.into_values().max().unwrap();

        (min, max)
    }
}

impl Data {
    pub fn part_one(&self) -> u64 {
        let pair_counts = (0..10).fold(self.initial_pairs(), |curr_pairs, _| self.tick(curr_pairs));

        let last_letter = self.template.iter().last().unwrap();
        let state = State { pair_counts };
        let (min, max) = state.min_max(*last_letter);

        max - min
    }

    pub fn part_two(&self) -> u64 {
        let pair_counts = (0..40).fold(self.initial_pairs(), |curr_pairs, _| self.tick(curr_pairs));

        let last_letter = self.template.iter().last().unwrap();
        let state = State { pair_counts };
        let (min, max) = state.min_max(*last_letter);

        max - min
    }

    fn tick(&self, pairs: HashMap<[char; 2], u64>) -> HashMap<[char; 2], u64> {
        let mut new_counts = HashMap::new();
        for ([left, right], count) in pairs {
            let mid = self.rules.get(&[left, right]).unwrap();
            // each newly created pair gets incremented by the count of the pair we're looping over
            *new_counts.entry([left, *mid]).or_default() += count;
            *new_counts.entry([*mid, right]).or_default() += count;
        }

        new_counts
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
