use std::collections::{HashMap, HashSet};

use crate::AoCData;

pub struct Data {
    map: HashMap<String, HashSet<String>>,
    molecule: String,
}

impl AoCData for Data {
    fn try_new(input: String) -> Option<Self> {
        let (replacements, molecule) = input.trim().split_once("\n\n")?;
        let mut map = HashMap::new();

        for line in replacements.lines() {
            let (from, to) = line.split_once(" => ")?;

            map.entry(from.to_string())
                .and_modify(|set: &mut HashSet<String>| {
                    set.insert(to.to_string());
                })
                .or_insert_with(|| HashSet::from([to.to_string()]));
        }

        Some(Self {
            map,
            molecule: molecule.to_string(),
        })
    }

    fn part_1(&self) -> String {
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
        new_molecules.len().to_string()
    }

    fn part_2(&self) -> String {
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
        let mut reverse_map = Vec::from_iter(reverse_map);
        reverse_map.sort_by(|&(key_a, _), &(key_b, _)| key_b.len().cmp(&key_a.len()));

        let mut curr = self.molecule.clone();
        let mut steps = 0;

        while curr != "e" {
            for (to, from) in &reverse_map {
                if curr.contains(*to) {
                    let match_count = curr.matches(*to).count();
                    curr = curr.replace(*to, from);
                    steps += match_count;
                }
            }
        }

        steps.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils;

    #[test]
    fn part_1() {
        let input = utils::get_sample_input(19);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1(), "");
    }

    #[test]
    fn part_2() {
        let input = utils::get_sample_input(19);
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2(), "");
    }
}
