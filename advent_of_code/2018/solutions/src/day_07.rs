use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Display,
};

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(HashMap<char, HashSet<char>>);

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut map: HashMap<char, HashSet<char>> = HashMap::new();
        for line in input.lines() {
            let line = line.strip_prefix("Step ").unwrap();
            let (from, _) = line.split_once(" ").unwrap();
            let from = from.chars().next().unwrap();

            let line = line
                .strip_suffix(" can begin.")
                .unwrap();
            let (_, to) = line.rsplit_once(" ").unwrap();
            let to = to.chars().next().unwrap();

            map.entry(to).or_default().insert(from);
            map.entry(from).or_default();
        }

        Ok(Self(map))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let mut map = self.0.clone();

        let mut ready_to_build: BinaryHeap<_> = map
            .iter()
            .filter(|(_step, reqs)| reqs.is_empty())
            .map(|(step, _reqs)| Reverse(*step))
            .collect();
        for Reverse(step) in ready_to_build.iter() {
            map.remove(step);
        }
        let mut done = String::new();

        while let Some(Reverse(next)) = ready_to_build.pop() {
            done.push(next);

            for (step, reqs) in map.iter_mut() {
                reqs.remove(&next);
                if reqs.is_empty() {
                    ready_to_build.push(Reverse(*step));
                }
            }
            for Reverse(step) in ready_to_build.iter() {
                map.remove(step);
            }
        }

        Ok(done)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let mut map = self.0.clone();

        let mut ready_to_build: BinaryHeap<_> = map
            .iter()
            .filter(|(_step, reqs)| reqs.is_empty())
            .map(|(step, _reqs)| Reverse(*step))
            .collect();
        for Reverse(step) in ready_to_build.iter() {
            map.remove(step);
        }

        let mut building = HashMap::new();
        let mut elapsed = 0;
        while !ready_to_build.is_empty() || !building.is_empty() {
            //  fill building
            while building.len() < 5 && !ready_to_build.is_empty() {
                let Reverse(next) = ready_to_build.pop().unwrap();
                building.insert(next, 60 + (next as u8 - b'A' + 1) as i32);
            }
            // advance time
            let to_advance = *building.values().min().unwrap();
            elapsed += to_advance;
            for remaining in building.values_mut() {
                *remaining -= to_advance;
            }

            let finished: Vec<_> = building
                .iter()
                .filter(|(_, remaining)| **remaining == 0)
                .map(|(step, _)| *step)
                .collect();
            for built in finished {
                building.remove(&built);
                for (&step, dependencies) in &mut map {
                    dependencies.remove(&built);
                    if dependencies.is_empty() {
                        ready_to_build.push(Reverse(step));
                    }
                }

                for Reverse(step) in ready_to_build.iter() {
                    map.remove(step);
                }
            }
        }
        Ok(elapsed)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "CABDFE");
    }
}
