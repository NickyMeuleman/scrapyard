use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use aoc_core::AoCError;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data {
    map: HashMap<String, HashSet<String>>,
    molecule: String,
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let (replacements, molecule) = input
            .trim()
            .split_once("\n\n")
            .ok_or(AoCError::Parsing)?;
        let mut map = HashMap::new();

        for line in replacements.lines() {
            let (from, to) = line
                .split_once(" => ")
                .ok_or(AoCError::Parsing)?;

            map.entry(from.to_string())
                .and_modify(|set: &mut HashSet<String>| {
                    set.insert(to.to_string());
                })
                .or_insert_with(|| HashSet::from([to.to_string()]));
        }

        Ok(Self {
            map,
            molecule: molecule.to_string(),
        })
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut new_molecules = HashSet::new();
        for (source, targets) in &self.map {
            for target in targets {
                for (start_idx, _) in self.molecule.match_indices(source) {
                    let in_front_of_source = &self.molecule[..start_idx];
                    let behind_source = &self.molecule[start_idx + source.len()..];
                    let new_molecule = format!("{}{}{}", in_front_of_source, target, behind_source);
                    new_molecules.insert(new_molecule);
                }
            }
        }

        Ok(new_molecules.len())
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        // work backwards to the single electron
        // start with the medicine and replace until you have a single e
        let mut reverse_map = HashMap::new();
        for (from, to_els) in &self.map {
            for to in to_els {
                reverse_map.insert(to, from);
            }
        }

        // sort by longest key first so the longest parts get replaced first
        // else, it CAN infinite loop. why?
        // edit: TODO: the test case still infinite loops sometimes
        let mut reverse_map = Vec::from_iter(reverse_map);
        reverse_map.sort_by(|&(key_a, _), &(key_b, _)| key_b.len().cmp(&key_a.len()));

        let mut curr = self.molecule.clone();
        let mut steps = 0;

        while curr != "e" {
            for (to, from) in &reverse_map {
                // dbg!(&curr);
                if curr.contains(*to) {
                    let match_count = curr.matches(*to).count();
                    curr = curr.replace(*to, from);
                    steps += match_count;
                }
            }
        }

        Ok(steps)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "H => HO
H => OH
O => HH

HOHOHO";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "7");
    }

    // sometimes infinite loops
    #[test]
    fn part_2() {
        let input = "e => H
e => O
H => HO
H => OH
O => HH

HOHOHO";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "6");
    }
}
