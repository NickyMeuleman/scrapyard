use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use aoc_core::AoCError;
use itertools::Itertools;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
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
                        let p1_change = self
                            .map
                            .get(&(p1.to_string(), p2.to_string()))
                            .unwrap();
                        // sitting p2 next to p1 causes this change
                        let p2_change = self
                            .map
                            .get(&(p2.to_string(), p1.to_string()))
                            .unwrap();
                        // new total happiness is the sum of the prev happiness and the changes
                        acc + p1_change + p2_change
                    })
            })
            .max()
            .unwrap()
    }
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut people = HashSet::new();
        let mut map = HashMap::new();

        for line in input.lines() {
            let line = line
                .strip_suffix('.')
                .ok_or(AoCError::Parsing)?;
            let (left, p2) = line
                .split_once(" happiness units by sitting next to ")
                .ok_or(AoCError::Parsing)?;
            let (p1, modifier) = left
                .split_once(" would ")
                .ok_or(AoCError::Parsing)?;
            let (direction, amount) = modifier
                .split_once(' ')
                .ok_or(AoCError::Parsing)?;
            let amount: i32 = amount.parse()?;
            let amount = match direction {
                "gain" => amount,
                "lose" => -amount,
                _ => panic!("invalid input"),
            };

            people.insert(p1.to_string());
            people.insert(p2.to_string());
            map.insert((p1.to_string(), p2.to_string()), amount);
        }

        Ok(Self { people, map })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        Ok(self.max_happiness())
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let me = "nicky";
        let mut map = self.map.clone();
        let mut people = self.people.clone();

        for person in &self.people {
            map.insert((me.to_string(), person.to_string()), 0);
            map.insert((person.to_string(), me.to_string()), 0);
        }

        people.insert(me.to_string());

        let data = Data { people, map };
        Ok(data.max_happiness())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "330");
    }

    #[test]
    fn part_2() {
        let input = "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "286");
    }
}
