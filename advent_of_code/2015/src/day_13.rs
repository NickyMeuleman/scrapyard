use crate::AoCData;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
pub struct Data {
    people: HashSet<String>,
    map: HashMap<(String, String), i32>,
}

impl Data {
    fn max_happiness(&self) -> i32 {
        self.people
            .iter()
            // all possible seating arrangements
            .permutations(self.people.len())
            // map to happiness for each seating arrangement
            .map(|perm| {
                perm.iter()
                    // wrap around to the start before the iteration stops because it's a round table
                    .circular_tuple_windows()
                    // keep track of total happiness
                    .fold(0, |acc, (p1, p2)| {
                        // sitting p1 next to p2 causes this change
                        let p1_change = self.map.get(&(p1.to_string(), p2.to_string())).unwrap();
                        // sitting p2 next to p1 causes this change
                        let p2_change = self.map.get(&(p2.to_string(), p1.to_string())).unwrap();
                        // new total happiness is the sum of the prev happiness and the changes
                        acc + p1_change + p2_change
                    })
            })
            .max()
            .unwrap()
    }
}

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        let mut people = HashSet::new();
        let mut map = HashMap::new();

        for line in input.lines() {
            let line = line.strip_suffix('.')?;
            let (left, p2) = line.split_once(" happiness units by sitting next to ")?;
            let (p1, modifier) = left.split_once(" would ")?;
            let (direction, amount) = modifier.split_once(' ')?;
            let amount: i32 = amount.parse().ok()?;
            let amount = match direction {
                "gain" => amount,
                "lose" => -amount,
                _ => panic!("invalid input"),
            };

            people.insert(p1.to_string());
            people.insert(p2.to_string());
            map.insert((p1.to_string(), p2.to_string()), amount);
        }

        Some(Self { people, map })
    }

    fn part_1(&self) -> String {
        self.max_happiness().to_string()
    }

    fn part_2(&self) -> String {
        let me = "nicky";
        let mut map = self.map.clone();
        let mut people = self.people.clone();

        for person in &self.people {
            map.insert((me.to_string(), person.to_string()), 0);
            map.insert((person.to_string(), me.to_string()), 0);
        }

        people.insert(me.to_string());

        let data = Data { people, map};
        data.max_happiness().to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(13);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "330");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(13);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "286");
    }
}
